use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub mod todo_http_handlers;
pub mod todo_repository;

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub due: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

impl TodoItem {
    pub fn new(title: String, description: String, is_done: bool, due: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            is_done,
            due,
            created: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoList {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub todos: Vec<TodoItem>,
    pub created: DateTime<Utc>,
}

impl TodoList {
    pub fn new(title: String, description: String, todos: Vec<TodoItem>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            todos,
            created: chrono::offset::Utc::now(),
        }
    }
}
