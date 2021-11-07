use crate::error::api_error;
use crate::handler::health_check_handler::health_check;
use crate::handler::todo_handler::{create_todo, delete_todo, update_todo_status};
use crate::handler::todo_list_handler::{
    create_todo_list, delete_todo_list, get_todo_list, get_todo_list_by_id,
};
use actix_cors::Cors;
use actix_web::error::InternalError;
use actix_web::{web, App, HttpServer, ResponseError};
use actix_web::http::header;
use dotenv::dotenv;
use infrastructure::repository::connection_manager::DBContext;
use serde::Deserialize;

mod error;
mod handler;
mod model;

#[derive(Deserialize)]
struct ServerConfig {
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Clone)]
pub struct ServerData {
    pub db_context: DBContext,
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    dotenv().ok();

    let server_config = match envy::from_env::<ServerConfig>() {
        Ok(val) => val,
        Err(err) => {
            panic!("{}", err);
        }
    };

    // Setup backtrace
    std::env::set_var("RUST_BACKTRACE", "full");

    let host = server_config.host.unwrap_or("0.0.0.0".parse().unwrap());
    let port = server_config.port.unwrap_or(8080);

    let server_data = ServerData {
        db_context: DBContext::new().await,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::JsonConfig::default()
                    .limit(4096000)
                    .error_handler(|err, _req| {
                        let message = "Invalid data".to_owned();
                        InternalError::from_response(
                            err,
                            api_error::ApiErrorResponse::BadRequest(message).error_response(),
                        )
                        .into()
                    }),
            )
            .app_data(web::PathConfig::default().error_handler(|err, _req| {
                InternalError::from_response(
                    err,
                    api_error::ApiErrorResponse::NotFound.error_response(),
                )
                .into()
            }))
            .data(server_data.clone())
            .wrap(Cors::default().allowed_origin_fn(|origin, _req_head| true)
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600))
            .service(health_check)
            .service(
                web::scope("api/v1")
                    .service(create_todo_list)
                    .service(get_todo_list)
                    .service(get_todo_list_by_id)
                    .service(delete_todo_list)
                    .service(create_todo)
                    .service(update_todo_status)
                    .service(delete_todo),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;
    Ok(())
}
