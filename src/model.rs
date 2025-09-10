use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Todo,
    Doing,
    Done,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub notes: String,
    pub status: Status,
    pub project: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
    pub updated_by: String,
}

impl Task {
    pub fn new(title: &str) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            notes: String::new(),
            status: Status::Todo,
            project: String::new(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            updated_by: crate::db::device_id(),
        }
    }
}
