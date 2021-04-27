use crate::error::api_error::{map_service_error, ApiErrorResponse};
use crate::model::todo_list::TodoListQuery;
use crate::ServerData;
use actix_web::{delete, get, post, web, HttpResponse};
use service::model::todo_list::CreateTodoListRequest;
use service::service::todo_list_service::TodoListService;
use uuid::Uuid;

#[post("/todo_list")]
pub async fn create_todo_list(
    data: web::Data<ServerData>,
    create_todo_request: web::Json<CreateTodoListRequest>,
) -> Result<HttpResponse, ApiErrorResponse> {
    let todo_list_service = TodoListService::new(&data.db_context);
    let todo_list_response = todo_list_service
        .register(&create_todo_request)
        .await
        .map_err(map_service_error)?;

    Ok(HttpResponse::Ok().json(todo_list_response))
}

#[get("/todo_list")]
pub async fn get_todo_list(
    data: web::Data<ServerData>,
    req_query: web::Query<TodoListQuery>,
) -> Result<HttpResponse, ApiErrorResponse> {
    let todo_list_service = TodoListService::new(&data.db_context);
    let todo_list_response = todo_list_service
        .get_todo_list(
            req_query.limit,
            req_query.last_todo_list_id,
            req_query.created_at,
        )
        .await
        .map_err(map_service_error)?;

    Ok(HttpResponse::Ok().json(todo_list_response))
}

#[get("/todo_list/{todo_list_id}")]
pub async fn get_todo_list_by_id(
    data: web::Data<ServerData>,
    web::Path(todo_list_id): web::Path<Uuid>,
) -> Result<HttpResponse, ApiErrorResponse> {
    let todo_list_service = TodoListService::new(&data.db_context);
    let todo_list_response = todo_list_service
        .get_todo_list_by_id(todo_list_id)
        .await
        .map_err(map_service_error)?;

    Ok(HttpResponse::Ok().json(todo_list_response))
}

#[delete("/todo_list/{todo_list_id}")]
pub async fn delete_todo_list(
    data: web::Data<ServerData>,
    web::Path(todo_list_id): web::Path<Uuid>,
) -> Result<HttpResponse, ApiErrorResponse> {
    let todo_list_service = TodoListService::new(&data.db_context);
    let todo_list_response = todo_list_service
        .delete(todo_list_id)
        .await
        .map_err(map_service_error)?;

    Ok(HttpResponse::Ok().json(todo_list_response))
}
