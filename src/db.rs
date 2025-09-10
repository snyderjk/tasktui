use anyhow::{Context, Result};
use directories::ProjectDirs;
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::{fs, path::PathBuf, str::FromStr};

use crate::model::{Status, Task};
use chrono::Utc;
use std::sync::OnceLock;
use uuid::Uuid;

static DEVICE_ID: OnceLock<String> = OnceLock::new();

pub fn device_id() -> String {
    DEVICE_ID.get_or_init(|| Uuid::new_v4().to_string()).clone()
}

fn data_dir() -> Result<PathBuf> {
    let pd = ProjectDirs::from("com", "snyder", "timetui")
        .context("Could not determine a data driectory for timetui")?;
    let dir = pd.data_dir().to_path_buf();
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create data dir: {}", dir.display()))?;
    Ok(dir)
}

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn init() -> Result<Self> {
        let mut path = data_dir()?;
        path.push("timetui.db");

        let url = format!("sqlite://{}", path.display());
        let opts = SqliteConnectOptions::from_str(&url)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await
            .with_context(|| format!("Failed connecting to {}", path.display()))?;

        // migrations: for brevity, inline exec
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS tasks (
          id TEXT PRIMARY KEY,
          title TEXT NOT NULL,
          notes TEXT DEFAULT '',
          status TEXT NOT NULL,
          project TEXT DEFAULT '',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          deleted_at INTEGER,
          updated_by TEXT NOT NULL
        );
        "#,
        )
        .execute(&pool)
        .await?;
        Ok(Self { pool })
    }

    pub async fn list_active_tasks(&self) -> Result<Vec<Task>> {
        let rows = sqlx::query("SELECT id,title,notes,status,project,created_at,updated_at,deleted_at,updated_by FROM tasks WHERE deleted_at IS NULL ORDER BY updated_at DESC")
            .fetch_all(&self.pool).await?;

        let items = rows
            .into_iter()
            .map(|r| Task {
                id: r.get::<String, _>("id").parse().unwrap(),
                title: r.get("title"),
                notes: r.get("notes"),
                status: match r.get::<String, _>("status").as_str() {
                    "Doing" => Status::Doing,
                    "Done" => Status::Done,
                    "Archived" => Status::Archived,
                    _ => Status::Todo,
                },
                project: r.get("project"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                deleted_at: r.get("deleted_at"),
                updated_by: r.get("updated_by"),
            })
            .collect();
        Ok(items)
    }

    pub async fn insert_task(&self, t: &Task) -> Result<()> {
        sqlx::query("INSERT INTO tasks (id,title,notes,status,project,created_at,updated_at,deleted_at,updated_by) VALUES (?,?,?,?,?,?,?,?,?)")
            .bind(t.id.to_string())
            .bind(&t.title)
            .bind(&t.notes)
            .bind(format!("{:?}", t.status))
            .bind(&t.project)
            .bind(t.created_at)
            .bind(t.updated_at)
            .bind(t.deleted_at)
            .bind(&t.updated_by)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_task(&self, t: &Task) -> Result<()> {
        let now = Utc::now().timestamp_millis();
        sqlx::query("UPDATE tasks SET title=?, notes=?, status=?, project=?, updated_at=?, updated_by=? WHERE id=?")
            .bind(&t.title)
            .bind(&t.notes)
            .bind(format!("{:?}", t.status))
            .bind(&t.project)
            .bind(now)
            .bind(&t.updated_by)
            .bind(t.id.to_string())
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete_task(&self, t: &Task) -> Result<()> {
        sqlx::query("DELETE FROM tasks where id=?")
            .bind(t.id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
