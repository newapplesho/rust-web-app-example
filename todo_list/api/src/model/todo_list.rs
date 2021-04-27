use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TodoListQuery {
    pub limit: Option<i32>,
    pub last_todo_list_id: Option<Uuid>,
    pub created_at: Option<NaiveDateTime>,
}
