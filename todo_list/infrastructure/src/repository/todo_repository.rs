use crate::error::infrastructure_error::map_sqlx_error;
use crate::repository::connection_manager::DBContext;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use domain::error::domain_error::DomainError;
use domain::model::todo::{NewTodo, Todo};
use domain::repository::todo_repository::ITodoRepository;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::Row;

#[async_trait]
impl ITodoRepository for DBContext {
    async fn create_todo(&self, new_todo: NewTodo) -> Result<Todo, DomainError> {
        let mut tx = self.pool_write.begin().await.map_err(map_sqlx_error)?;

        let created_todo = sqlx::query(
            r"
                INSERT INTO todo(todo_list_id, name, description, status_id) VALUES($1, $2, $3, $4)
                RETURNING todo_id, todo_list_id, name, description, created_at, updated_at, status_id
            ",
        )
        .bind(&new_todo.todo_list_id)
        .bind(&new_todo.name)
        .bind(&new_todo.description)
        .bind(&new_todo.status_id)
        .try_map(|row: PgRow| { map_to_todo(row) })
        .fetch_one(&mut tx)
        .await
        .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error);

        Ok(created_todo)
    }

    async fn get_todo_by_list_id(&self, todo_list_id: Uuid) -> Result<Vec<Todo>, DomainError> {
        let search_todo = sqlx::query(
            r"
                SELECT
                    todo_id, todo_list_id, name, description, created_at, updated_at, status_id
                FROM TODO
                WHERE
                   todo_list_id = $1
                ORDER BY
                    updated_at DESC
            ",
        )
        .bind(todo_list_id)
        .try_map(|row: PgRow| map_to_todo(row))
        .fetch_all(&self.pool_read)
        .await
        .map_err(map_sqlx_error)?;
        Ok(search_todo)
    }

    async fn delete_todo(&self, todo_id: Uuid) -> Result<(), DomainError> {
        sqlx::query(r"DELETE FROM TODO WHERE todo_id = $1")
            .bind(todo_id)
            .execute(&self.pool_write)
            .await
            .map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn update_status(&self, todo_id: Uuid, status_id: i32) -> Result<Todo, DomainError> {
        let mut tx = self.pool_write.begin().await.map_err(map_sqlx_error)?;

        let update_todo = sqlx::query(
            r"UPDATE TODO 
            SET 
                updated_at = CURRENT_TIMESTAMP,
                status_id = $2 
            WHERE todo_id = $1
RETURNING *",
        )
        .bind(todo_id)
        .bind(status_id)
        .try_map(|row: PgRow| map_to_todo(row))
        .fetch_one(&mut tx)
        .await
        .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error);

        Ok(update_todo)
    }

    async fn modify_todo(&self, todo: Todo) -> Result<Todo, DomainError> {
        todo!()
    }
}

fn map_to_todo(row: PgRow) -> Result<Todo, sqlx::Error> {
    let todo_id: Uuid = row.try_get("todo_id")?;
    let todo_list_id: Uuid = row.try_get("todo_list_id")?;
    let name: String = row.try_get("name")?;
    let description: Option<String> = row.try_get("description")?;
    let created_at: NaiveDateTime = row.try_get("created_at")?;
    let updated_at: NaiveDateTime = row.try_get("updated_at")?;
    let status_id: i32 = row.try_get("status_id")?;

    Ok(Todo {
        todo_id,
        todo_list_id,
        name,
        description,
        created_at,
        updated_at,
        status_id,
    })
}

#[cfg(test)]
mod tests {
    use crate::repository::connection_manager::DBContext;
    use domain::model::todo::NewTodo;
    use domain::model::todo_list::NewTodoList;
    use domain::model::todo_status::TodoStatus;
    use domain::repository::todo_list_repository::ITodoListRepository;
    use domain::repository::todo_repository::ITodoRepository;
    use uuid::Uuid;

    #[actix_rt::test]
    async fn create_todo() {
        let db_context = DBContext::new().await;

        let todo_list_title = "New Todo list".to_string();

        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        let created_todo_list = ITodoListRepository::create_todo_list(&db_context, new_todo_list)
            .await
            .unwrap();

        let new_todo = NewTodo {
            todo_list_id: created_todo_list.todo_list_id,
            name: "Test New Todo".to_string(),
            description: Option::from("test".to_string()),
            status_id: TodoStatus::TODO,
        };

        let result = db_context.create_todo(new_todo).await.unwrap();

        assert_eq!("Test New Todo", result.name);
    }

    #[actix_rt::test]
    async fn update_status() {
        let db_context = DBContext::new().await;

        let todo_list_title = "New Todo list".to_string();

        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        let created_todo_list = ITodoListRepository::create_todo_list(&db_context, new_todo_list)
            .await
            .unwrap();

        let new_todo = NewTodo {
            todo_list_id: created_todo_list.todo_list_id,
            name: "Test New Todo".to_string(),
            description: Option::from("test".to_string()),
            status_id: TodoStatus::TODO,
        };

        let created_todo = db_context.create_todo(new_todo).await.unwrap();

        let result = db_context
            .update_status(created_todo.todo_id, TodoStatus::DONE)
            .await
            .unwrap();

        assert_eq!(TodoStatus::DONE, result.status_id);
        assert_eq!(created_todo.created_at, result.created_at);

        assert_ne!(created_todo.status_id, result.status_id);
        assert_ne!(created_todo.updated_at, result.updated_at);
    }

    #[actix_rt::test]
    async fn get_todo_by_list_id() {
        let db_context = DBContext::new().await;

        let todo_list_title = "New Todo list".to_string();
        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        let created_todo_list = ITodoListRepository::create_todo_list(&db_context, new_todo_list)
            .await
            .unwrap();

        let new_todo = NewTodo {
            todo_list_id: created_todo_list.todo_list_id,
            name: "Test New Todo".to_string(),
            description: Option::from("test".to_string()),
            status_id: TodoStatus::TODO,
        };
        db_context.create_todo(new_todo).await.unwrap();

        let results = db_context
            .get_todo_by_list_id(created_todo_list.todo_list_id)
            .await
            .unwrap();

        assert!(results.len() > 0);
    }

    #[actix_rt::test]
    async fn delete_todo() {
        let db_context = DBContext::new().await;

        let todo_list_title = "New Todo list".to_string();

        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        let created_todo_list = ITodoListRepository::create_todo_list(&db_context, new_todo_list)
            .await
            .unwrap();

        let new_todo = NewTodo {
            todo_list_id: created_todo_list.todo_list_id,
            name: "Test New Todo".to_string(),
            description: Option::from("test".to_string()),
            status_id: TodoStatus::TODO,
        };

        let created_todo = db_context.create_todo(new_todo).await.unwrap();

        let result = db_context
            .delete_todo(created_todo.todo_list_id)
            .await
            .unwrap();

        assert_eq!((), result)
    }
}
