use crate::error::api_error::{map_service_error, ApiErrorResponse};
use crate::ServerData;
use actix_web::{delete, patch, post, web, HttpResponse};
use service::model::todo::{CreateTodoRequest, UpdateTodoStatusRequest};
use service::service::todo_service::TodoService;
use uuid::Uuid;

#[post("/todo")]
pub async fn create_todo(
    data: web::Data<ServerData>,
    create_todo_request: web::Json<CreateTodoRequest>,
) -> Result<HttpResponse, ApiErrorResponse> {
    let todo_service = TodoService::new(&data.db_context);
    let create_todo = todo_service
        .create_todo(&create_todo_request)
        .await
        .map_err(map_service_error)?;
    Ok(HttpResponse::Ok().json(create_todo))
}

#[patch("/todo")]
pub async fn update_todo_status(
    data: web::Data<ServerData>,
    update_todo_status_request: web::Json<UpdateTodoStatusRequest>,
) -> Result<HttpResponse, ApiErrorResponse> {
    let todo_service = TodoService::new(&data.db_context);
    let updated_todo = todo_service
        .update_status(
            update_todo_status_request.todo_id,
            update_todo_status_request.status,
        )
        .await
        .map_err(map_service_error)?;
    Ok(HttpResponse::Ok().json(updated_todo))
}

#[delete("/todo/{todo_id}")]
pub async fn delete_todo(
    data: web::Data<ServerData>,
    web::Path(todo_id): web::Path<Uuid>,
) -> Result<HttpResponse, ApiErrorResponse> {
    let todo_service = TodoService::new(&data.db_context);
    let result = todo_service
        .delete_todo(todo_id)
        .await
        .map_err(map_service_error)?;
    Ok(HttpResponse::Ok().json(result))
}
