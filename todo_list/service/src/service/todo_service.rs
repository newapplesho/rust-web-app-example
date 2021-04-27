use crate::error::service_error::{map_domain_error, ServiceError};
use crate::model::todo::{CreateTodoRequest, TodoInfo};
use domain::model::todo::NewTodo;
use domain::repository::todo_repository::ITodoRepository;
use infrastructure::repository::connection_manager::DBContext;
use uuid::Uuid;

pub struct TodoService<'a> {
    db_context: &'a DBContext,
}

impl<'a> TodoService<'a>
where
    DBContext: ITodoRepository,
{
    pub fn new(db_content: &'a DBContext) -> TodoService<'a> {
        TodoService {
            db_context: db_content,
        }
    }

    pub async fn create_todo(
        &self,
        new_todo_request: &CreateTodoRequest,
    ) -> Result<TodoInfo, ServiceError> {
        // TODO: Checking todo status

        let created_todo = self
            .db_context
            .create_todo(NewTodo {
                todo_list_id: new_todo_request.todo_list_id,
                name: new_todo_request.name.clone(),
                description: new_todo_request.description.clone(),
                status_id: new_todo_request.status,
            })
            .await;

        match created_todo {
            Ok(todo) => Ok(TodoInfo::from_todo(todo)),
            Err(e) => Err(map_domain_error(e)),
        }
    }

    pub async fn update_status(
        &self,
        todo_id: Uuid,
        status_id: i32,
    ) -> Result<TodoInfo, ServiceError> {
        // TODO: Checking todo status

        let update_todo = self.db_context.update_status(todo_id, status_id).await;
        match update_todo {
            Ok(todo) => Ok(TodoInfo {
                todo_id: todo.todo_id,
                todo_list_id: todo.todo_list_id,
                name: todo.name,
                description: todo.description,
                created_at: todo.created_at,
                updated_at: todo.updated_at,
                status: todo.status_id,
            }),
            Err(e) => Err(map_domain_error(e)),
        }
    }

    pub async fn delete_todo(&self, todo_id: Uuid) -> Result<(), ServiceError> {
        self.db_context
            .delete_todo(todo_id)
            .await
            .map_err(map_domain_error)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::todo::CreateTodoRequest;
    use crate::model::todo_list::CreateTodoListRequest;
    use crate::service::todo_list_service::TodoListService;
    use crate::service::todo_service::TodoService;
    use domain::model::todo_status::TodoStatus;
    use infrastructure::repository::connection_manager::DBContext;

    #[actix_rt::test]
    async fn create_todo() {
        let db_context = DBContext::new().await;
        let todo_list_service = TodoListService::new(&db_context);
        let create_todo_list = CreateTodoListRequest {
            name: "New Todo List".parse().unwrap(),
        };
        let created_todo_list = todo_list_service.register(&create_todo_list).await.unwrap();

        let todo_service = TodoService::new(&db_context);
        let created_todo = todo_service
            .create_todo(&CreateTodoRequest {
                todo_list_id: created_todo_list.todo_list_id,
                name: "New Todo".to_string(),
                description: Option::from("Test Description".to_string()),
                status: TodoStatus::TODO,
            })
            .await
            .unwrap();

        assert_eq!("New Todo", created_todo.name);
    }

    #[actix_rt::test]
    async fn update_status() {
        let db_context = DBContext::new().await;
        let todo_list_service = TodoListService::new(&db_context);
        let create_todo_list = CreateTodoListRequest {
            name: "New Todo List".parse().unwrap(),
        };
        let created_todo_list = todo_list_service.register(&create_todo_list).await.unwrap();

        let todo_service = TodoService::new(&db_context);
        let created_todo = todo_service
            .create_todo(&CreateTodoRequest {
                todo_list_id: created_todo_list.todo_list_id,
                name: "New Todo".to_string(),
                description: Option::from("Test Description".to_string()),
                status: TodoStatus::TODO,
            })
            .await
            .unwrap();

        let result = todo_service
            .update_status(created_todo.todo_id, TodoStatus::DONE)
            .await
            .unwrap();

        assert_ne!(created_todo.status, result.status);
    }

    #[actix_rt::test]
    async fn delete_todo() {
        let db_context = DBContext::new().await;
        let todo_list_service = TodoListService::new(&db_context);
        let create_todo_list = CreateTodoListRequest {
            name: "New Todo List".parse().unwrap(),
        };
        let created_todo_list = todo_list_service.register(&create_todo_list).await.unwrap();

        let todo_service = TodoService::new(&db_context);
        let created_todo = todo_service
            .create_todo(&CreateTodoRequest {
                todo_list_id: created_todo_list.todo_list_id,
                name: "New Todo".to_string(),
                description: Option::from("Test Description".to_string()),
                status: TodoStatus::TODO,
            })
            .await
            .unwrap();

        let result1 = todo_service
            .delete_todo(created_todo.todo_id)
            .await
            .unwrap();
        assert_eq!((), result1);

        let result2 = todo_service.delete_todo(created_todo.todo_id).await;
        assert!(!result2.is_err());
    }
}
