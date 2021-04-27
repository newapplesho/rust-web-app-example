use crate::error::domain_error::DomainError;
use crate::model::todo_list::{NewTodoList, TodoList};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[async_trait]
pub trait ITodoListRepository {
    async fn create_todo_list(&self, new_todo_list: NewTodoList) -> Result<TodoList, DomainError>;
    async fn get_todo_list_by_id(&self, todo_list_id: Uuid) -> Result<TodoList, DomainError>;
    async fn get_todo_list(
        &self,
        limit: Option<i32>,
        lat_todo_list_id: Option<Uuid>,
        created_at: Option<NaiveDateTime>,
    ) -> Result<Vec<TodoList>, DomainError>;
    async fn modify_todo_list(
        &self,
        todo_list_id: Uuid,
        name: String,
    ) -> Result<TodoList, DomainError>;
    async fn delete_todo_list(&self, todo_list_id: Uuid) -> Result<(), DomainError>;
}
