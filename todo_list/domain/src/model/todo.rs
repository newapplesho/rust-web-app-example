use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Todo {
    pub todo_id: Uuid,
    pub todo_list_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status_id: i32,
}

pub struct NewTodo {
    pub todo_list_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status_id: i32,
}
