use crate::error::service_error::{map_domain_error, ServiceError};
use crate::model::todo::TodoInfo;
use crate::model::todo_list::{CreateTodoListRequest, TodoListDetailsInfo, TodoListInfo};
use chrono::NaiveDateTime;
use domain::error::domain_error::DomainError;
use domain::model::todo_list::NewTodoList;
use domain::repository::todo_list_repository::ITodoListRepository;
use domain::repository::todo_repository::ITodoRepository;
use infrastructure::repository::connection_manager::DBContext;
use log::error;
use uuid::Uuid;

pub struct TodoListService<'a> {
    db_context: &'a DBContext,
}

impl<'a> TodoListService<'a>
where
    DBContext: ITodoListRepository + ITodoRepository,
{
    pub fn new(db_content: &'a DBContext) -> TodoListService<'a> {
        TodoListService {
            db_context: db_content,
        }
    }

    pub async fn register(
        &self,
        create_todo_list_request: &CreateTodoListRequest,
    ) -> Result<TodoListInfo, ServiceError> {
        let new_todo_list = self
            .db_context
            .create_todo_list(NewTodoList {
                name: create_todo_list_request.name.clone(),
            })
            .await;

        match new_todo_list {
            Ok(todo_list) => Ok(TodoListInfo {
                todo_list_id: todo_list.todo_list_id,
                name: todo_list.name,
                created_at: todo_list.created_at,
                updated_at: todo_list.updated_at,
            }),
            Err(DomainError::ResourceNotFound) => return Err(ServiceError::ResourceNotFound),
            Err(e) => {
                error!("Unexpected error from db: {:?}", e);
                return Err(ServiceError::UnexpectedServiceError);
            }
        }
    }

    pub async fn get_todo_list(
        &self,
        limit: Option<i32>,
        last_todo_list_id: Option<Uuid>,
        created_at: Option<NaiveDateTime>,
    ) -> Result<Vec<TodoListInfo>, ServiceError> {
        let todo_list_collection = self
            .db_context
            .get_todo_list(limit, last_todo_list_id, created_at)
            .await
            .map_err(map_domain_error)?;

        if todo_list_collection.is_empty() {
            return Err(ServiceError::ResourceNotFound);
        }

        let mut result = Vec::new();
        for todo_list in todo_list_collection {
            result.push(TodoListInfo::from_todo_list(todo_list));
        }
        Ok(result)
    }

    pub async fn get_todo_list_by_id(
        &self,
        todo_list_id: Uuid,
    ) -> Result<TodoListDetailsInfo, ServiceError> {
        let todo_collections = self
            .db_context
            .get_todo_by_list_id(todo_list_id)
            .await
            .map_err(map_domain_error)?;

        let search_todo_list = self.db_context.get_todo_list_by_id(todo_list_id).await;

        match search_todo_list {
            Ok(todo_list) => Ok(TodoListDetailsInfo {
                todo_list_id: todo_list.todo_list_id,
                name: todo_list.name,
                created_at: todo_list.created_at,
                updated_at: todo_list.updated_at,
                todo_count: todo_collections.len(),
                todo: todo_collections
                    .into_iter()
                    .map(|todo| TodoInfo::from_todo(todo))
                    .collect(),
            }),
            Err(_) => Err(ServiceError::ResourceNotFound),
        }
    }

    pub async fn delete(&self, todo_list_id: Uuid) -> Result<(), ServiceError> {
        let search_todo_list = self.db_context.get_todo_list_by_id(todo_list_id).await;
        if search_todo_list.is_err() {
            Err(ServiceError::ResourceNotFound)
        } else {
            let result = self.db_context.delete_todo_list(todo_list_id).await;

            match result {
                Ok(_) => Ok(()),
                Err(e) => return Err(map_domain_error(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::todo_list::CreateTodoListRequest;
    use crate::service::todo_list_service::TodoListService;
    use infrastructure::repository::connection_manager::DBContext;
    use uuid::Uuid;

    #[actix_rt::test]
    async fn create() {
        let db_context = DBContext::new().await;
        let todo_list_service = TodoListService::new(&db_context);
        let create_todo_list = CreateTodoListRequest {
            name: "New Todo List".parse().unwrap(),
        };
        let result = todo_list_service.register(&create_todo_list).await.unwrap();

        assert_eq!("New Todo List", result.name)
    }

    #[actix_rt::test]
    async fn get_todo_list() {
        let db_context = DBContext::new().await;
        let todo_list_service = TodoListService::new(&db_context);
        let create_todo_list = CreateTodoListRequest {
            name: "New Todo List".parse().unwrap(),
        };
        let created_todo_list = todo_list_service.register(&create_todo_list).await.unwrap();

        let result1 = todo_list_service
            .get_todo_list_by_id(created_todo_list.todo_list_id)
            .await
            .unwrap();

        assert_eq!(created_todo_list.todo_list_id, result1.todo_list_id);

        let result2 = todo_list_service
            .get_todo_list(None, None, None)
            .await
            .unwrap();

        println!("{:?}", result2.get(0).unwrap());

        assert!(result2.len() > 0);
    }

    #[actix_rt::test]
    async fn get_todo_list_by_id() {
        let db_context = DBContext::new().await;
        let todo_list_service = TodoListService::new(&db_context);
        let create_todo_list = CreateTodoListRequest {
            name: "New Todo List".parse().unwrap(),
        };
        let created_todo_list = todo_list_service.register(&create_todo_list).await.unwrap();

        let result1 = todo_list_service
            .get_todo_list_by_id(created_todo_list.todo_list_id)
            .await
            .unwrap();

        assert_eq!(created_todo_list.todo_list_id, result1.todo_list_id);

        let result2 = todo_list_service.get_todo_list_by_id(Uuid::new_v4()).await;

        assert!(result2.is_err());
    }

    #[actix_rt::test]
    async fn delete() {
        let db_context = DBContext::new().await;
        let todo_list_service = TodoListService::new(&db_context);
        let create_todo_list = CreateTodoListRequest {
            name: "New Todo List".parse().unwrap(),
        };
        let created_todo_list = todo_list_service.register(&create_todo_list).await.unwrap();

        assert_eq!("New Todo List", created_todo_list.name);

        let result = todo_list_service
            .delete(created_todo_list.todo_list_id)
            .await
            .unwrap();

        assert_eq!((), result)
    }
}
