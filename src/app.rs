use crate::db::Db;
use crate::model::{Status, Task};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
        let form = match &mut self.mode {
            Mode::Adding(f) => f,
            _ => unreachable!(),
        };

        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }

            KeyCode::Enter => {
                if !form.title.trim().is_empty() {
                    let mut t = Task::new(form.title.trim());
                    t.notes = form.notes.trim().to_string();
                    self.db.insert_task(&t).await?;

                    self.tasks = self.db.list_active_tasks().await?;
                    self.selected = 0;
                    self.mode = Mode::Normal
                }
            }

            KeyCode::Tab => {
                form.field = match form.field {
                    AddField::Title => AddField::Notes,
                    AddField::Notes => AddField::Title,
                };
            }
            KeyCode::BackTab => {
                form.field = match form.field {
                    AddField::Title => AddField::Notes,
                    AddField::Notes => AddField::Title,
                };
            }
            KeyCode::Backspace => {
                let s = match form.field {
                    AddField::Title => &mut form.title,
                    AddField::Notes => &mut form.notes,
                };
                s.pop();
            }
            // Hacky fix for backspace, checks if Control+H was sent for backspace
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let s = match form.field {
                    AddField::Title => &mut form.title,
                    AddField::Notes => &mut form.notes,
                };
                s.pop();
            }
            KeyCode::Char(c) => {
                if !key
                    .modifiers
                    .contains(KeyModifiers::CONTROL | KeyModifiers::ALT)
                {
                    let s = match form.field {
                        AddField::Title => &mut form.title,
                        AddField::Notes => &mut form.notes,
                    };
                    s.push(c);
                }
            }
            _ => {}
        }

        Ok(false)
    }
}
