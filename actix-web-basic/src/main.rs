mod handler;

use crate::handler::cookie_handler::get_cookie;
use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health_check)
            .service(get_cookie)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_health_check() {
        let app = App::new().service(health_check);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/healthcheck").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"OK"##);
    }
}
