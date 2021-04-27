use crate::error::domain_error::DomainError;
use crate::model::todo::{NewTodo, Todo};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ITodoRepository {
    async fn create_todo(&self, new_todo: NewTodo) -> Result<Todo, DomainError>;
    async fn get_todo_by_list_id(&self, todo_list_id: Uuid) -> Result<Vec<Todo>, DomainError>;
    async fn delete_todo(&self, todo_id: Uuid) -> Result<(), DomainError>;
    async fn update_status(&self, todo_id: Uuid, status_id: i32) -> Result<Todo, DomainError>;
    async fn modify_todo(&self, todo: Todo) -> Result<Todo, DomainError>;
}
