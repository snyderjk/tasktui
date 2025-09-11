use crate::db::Db;
use crate::model::{Status, Task};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

pub enum Mode {
    Normal,
    Adding(AddForm),
}

pub struct AddForm {
    pub title: String,
    pub notes: String,
    pub field: AddField,
}

pub enum AddField {
    Title,
    Notes,
}

pub struct App {
    pub db: Db,
    pub selected: usize,
    pub tasks: Vec<Task>,
    pub show_done: bool,
    pub running_task: Option<Uuid>,
    pub mode: Mode,
}

impl App {
    pub async fn init() -> Result<Self> {
        let db = Db::init().await?;
        let tasks = db.list_active_tasks().await?;
        Ok(Self {
            db,
            selected: 0,
            tasks,
            show_done: false,
            running_task: None,
            mode: Mode::Normal,
        })
    }

    pub fn tick(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn on_key(&mut self, key: KeyEvent) -> Result<bool> {
        match &mut self.mode {
            Mode::Normal => self.on_key_normal(key).await,
            Mode::Adding(form) => self.on_key_adding(key).await,
        }
    }

    pub async fn on_key_normal(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected + 1 < self.tasks.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Char('a') => {
                let num = self.tasks.len() + 1;
                let t = Task::new(format!("Task {}", num).as_str());
                self.db.insert_task(&t).await?;
                self.tasks.insert(0, t);
                self.selected = 0;

                self.mode = Mode::Adding(AddForm {
                    title: String::new(),
                    notes: String::new(),
                    field: AddField::Title,
                })
            }
            KeyCode::Char('x') => {
                if let Some(t) = self.tasks.get(self.selected) {
                    self.db.delete_task(&t).await?;
                    self.tasks.remove(self.selected);
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
            }
            _ => {}
        }
        Ok(false)
    }

    // pub async fn on_key_adding(&mut self, key: KeyEvent, form: &mut AddForm) -> Result<bool> {
    pub async fn on_key_adding(&mut self, key: KeyEvent) -> Result<bool> {
        if key.code == KeyCode::Esc {
            self.mode = Mode::Normal;
            return Ok(false);
        }

        match key.code {
            _ => {}
        }

        Ok(false)
    }
}
