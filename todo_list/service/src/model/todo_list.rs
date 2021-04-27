use crate::model::todo::TodoInfo;
use chrono::NaiveDateTime;
use domain::model::todo_list::TodoList;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateTodoListRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TodoListInfo {
    pub todo_list_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TodoListInfo {
    pub fn from_todo_list(todo_list: TodoList) -> TodoListInfo {
        TodoListInfo {
            todo_list_id: todo_list.todo_list_id,
            name: todo_list.name,
            created_at: todo_list.created_at,
            updated_at: todo_list.updated_at,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TodoListDetailsInfo {
    pub todo_list_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub todo_count: usize,
    pub todo: Vec<TodoInfo>,
}
