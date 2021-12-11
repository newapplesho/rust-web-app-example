use crate::error::infrastructure_error::map_sqlx_error;
use crate::repository::connection_manager::DBContext;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use domain::error::domain_error::DomainError;
use domain::model::todo_list::{NewTodoList, TodoList};
use domain::repository::todo_list_repository::ITodoListRepository;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::Row;

#[async_trait]
impl ITodoListRepository for DBContext {
    async fn create_todo_list(&self, new_todo_list: NewTodoList) -> Result<TodoList, DomainError> {
        let mut tx = self.pool_write.begin().await.map_err(map_sqlx_error)?;

        let created_todo_list = sqlx::query(
            r"
                INSERT INTO todo_list(name) VALUES($1)
                RETURNING todo_list_id, name, created_at, updated_at
            ",
        )
        .bind(&new_todo_list.name)
        .try_map(|row: PgRow| {
            let todo_list_id: Uuid = row.try_get("todo_list_id")?;
            let name: String = row.try_get("name")?;
            let created_at: NaiveDateTime = row.try_get("created_at")?;
            let updated_at: NaiveDateTime = row.try_get("updated_at")?;

            Ok(TodoList {
                todo_list_id,
                name,
                created_at,
                updated_at,
            })
        })
        .fetch_one(&mut tx)
        .await
        .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(created_todo_list)
    }

    async fn get_todo_list_by_id(&self, todo_list_id: Uuid) -> Result<TodoList, DomainError> {
        let result_todo_list = sqlx::query(
            r"
        SELECT
            todo_list_id,
            name,
            created_at,
            updated_at
        FROM
            todo_list
        WHERE
            todo_list_id = $1
        ",
        )
        .bind(todo_list_id)
        .try_map(|row: PgRow| map_to_todo_list(row))
        .fetch_one(&self.pool_read)
        .await
        .map_err(map_sqlx_error);
        result_todo_list
    }

    async fn get_todo_list(
        &self,
        limit: Option<i32>,
        last_todo_list_id: Option<Uuid>,
        created_at: Option<NaiveDateTime>,
    ) -> Result<Vec<TodoList>, DomainError> {
        let list_per_page = match limit {
            Some(v) => v,
            None => 10,
        };

        if last_todo_list_id.is_none() {
            let result_todo_list = sqlx::query(
                r"
        SELECT
            todo_list_id,
            name,
            created_at,
            updated_at
        FROM
            todo_list
        ORDER BY
            created_at DESC, todo_list_id DESC 
        LIMIT $1
        ",
            )
            .bind(list_per_page)
            .try_map(|row: PgRow| {
                let todo_list_id: Uuid = row.try_get("todo_list_id")?;
                let name: String = row.try_get("name")?;
                let created_at: NaiveDateTime = row.try_get("created_at")?;
                let updated_at: NaiveDateTime = row.try_get("updated_at")?;

                Ok(TodoList {
                    todo_list_id,
                    name,
                    created_at,
                    updated_at,
                })
            })
            .fetch_all(&self.pool_read)
            .await
            .map_err(map_sqlx_error)?;

            Ok(result_todo_list)
        } else {
            let result_todo_list = sqlx::query(
                r"
        SELECT
            todo_list_id,
            name,
            created_at,
            updated_at
        FROM
            todo_list
        WHERE
            created_at < $2
        OR
        (
            created_at = $2
        AND
            todo_list_id < $3
        )
        ORDER BY
            created_at DESC, todo_list_id DESC 
        LIMIT $1
        ",
            )
            .bind(list_per_page)
            .bind(created_at)
            .bind(last_todo_list_id)
            .try_map(|row: PgRow| map_to_todo_list(row))
            .fetch_all(&self.pool_read)
            .await
            .map_err(map_sqlx_error)?;
            Ok(result_todo_list)
        }
    }

    async fn modify_todo_list(
        &self,
        todo_list_id: Uuid,
        name: String,
    ) -> Result<TodoList, DomainError> {
        let mut tx = self.pool_write.begin().await.map_err(map_sqlx_error)?;

        let updated_todo_list = sqlx::query(
            r#"
                    UPDATE
                        todo_list
                    SET
                        updated_at = CURRENT_TIMESTAMP,
                        name = $2
                    WHERE todo_list_id = $1
                    RETURNING *"#,
        )
        .bind(todo_list_id)
        .bind(name)
        .try_map(|row: PgRow| map_to_todo_list(row))
        .fetch_one(&mut tx)
        .await
        .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(updated_todo_list)
    }

    async fn delete_todo_list(&self, todo_list_id: Uuid) -> Result<(), DomainError> {
        let tx = self.pool_write.begin().await.map_err(map_sqlx_error)?;

        sqlx::query(r"DELETE FROM todo_list WHERE todo_list_id = $1")
            .bind(todo_list_id)
            .execute(&self.pool_write)
            .await
            .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}

fn map_to_todo_list(row: PgRow) -> Result<TodoList, sqlx::Error> {
    let todo_list_id: Uuid = row.try_get("todo_list_id")?;
    let name: String = row.try_get("name")?;
    let created_at: NaiveDateTime = row.try_get("created_at")?;
    let updated_at: NaiveDateTime = row.try_get("updated_at")?;

    Ok(TodoList {
        todo_list_id,
        name,
        created_at,
        updated_at,
    })
}

#[cfg(test)]
mod tests {
    use crate::repository::connection_manager::DBContext;
    use crate::repository::todo_list_repository::ITodoListRepository;
    use domain::model::todo_list::NewTodoList;

    #[actix_rt::test]
    async fn create_todo_list() {
        let db_context = DBContext::new().await;

        let todo_list_title = "New Todo list".to_string();

        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        let result_todo_list = ITodoListRepository::create_todo_list(&db_context, new_todo_list)
            .await
            .unwrap();

        assert_eq!(todo_list_title, result_todo_list.name)
    }

    #[actix_rt::test]
    async fn get_todo_list_by_id() {
        let db_context = DBContext::new().await;

        let todo_list_title = "New Todo list".to_string();

        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        let created_new_todo_list =
            ITodoListRepository::create_todo_list(&db_context, new_todo_list)
                .await
                .unwrap();

        let get_todo_list = db_context
            .get_todo_list_by_id(created_new_todo_list.todo_list_id)
            .await
            .unwrap();

        assert_eq!(
            created_new_todo_list.todo_list_id,
            get_todo_list.todo_list_id
        );
    }

    #[actix_rt::test]
    async fn get_todo_list() {
        let db_context = DBContext::new().await;
        let todo_list_title = "New Todo list".to_string();
        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        ITodoListRepository::create_todo_list(&db_context, new_todo_list)
            .await
            .unwrap();

        let result_list = db_context
            .get_todo_list(Option::from(5), None, None)
            .await
            .unwrap();

        assert!(result_list.len() > 0)
    }

    #[actix_rt::test]
    async fn modify_todo_list() {
        let db_context = DBContext::new().await;
        let todo_list_title = "New Todo list".to_string();
        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        let created_new_todo_list =
            ITodoListRepository::create_todo_list(&db_context, new_todo_list)
                .await
                .unwrap();

        let modify_todo_list = ITodoListRepository::modify_todo_list(
            &db_context,
            created_new_todo_list.todo_list_id,
            "Updated Todo List name".to_string(),
        )
        .await
        .unwrap();

        assert_eq!(
            created_new_todo_list.todo_list_id,
            modify_todo_list.todo_list_id
        );
        assert_ne!(
            created_new_todo_list.updated_at,
            modify_todo_list.updated_at
        );
    }

    #[actix_rt::test]
    async fn delete_todo_list() {
        let db_context = DBContext::new().await;

        let todo_list_title = "Delete Todo list".to_string();

        let new_todo_list = NewTodoList {
            name: todo_list_title.clone(),
        };

        let created_new_todo_list =
            ITodoListRepository::create_todo_list(&db_context, new_todo_list)
                .await
                .unwrap();

        db_context
            .delete_todo_list(created_new_todo_list.todo_list_id)
            .await
            .unwrap();

        let get_todo_list = db_context
            .get_todo_list_by_id(created_new_todo_list.todo_list_id)
            .await;

        assert!(get_todo_list.is_err());
    }
}
