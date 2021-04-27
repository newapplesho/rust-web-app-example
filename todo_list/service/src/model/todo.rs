use chrono::NaiveDateTime;
use domain::model::todo::Todo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TodoInfo {
    pub todo_id: Uuid,
    pub todo_list_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: i32,
}

impl TodoInfo {
    pub fn from_todo(todo: Todo) -> TodoInfo {
        TodoInfo {
            todo_id: todo.todo_id,
            todo_list_id: todo.todo_list_id,
            name: todo.name,
            description: todo.description,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
            status: todo.status_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateTodoRequest {
    pub todo_list_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateTodoStatusRequest {
    pub todo_id: Uuid,
    pub status: i32,
}
