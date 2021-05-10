use actix_web::cookie::Cookie;
use actix_web::{get, Error, HttpRequest, HttpResponse};
use time::Duration;
use uuid::Uuid;

#[get("/cookie")]
pub async fn get_cookie(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let cookie = Cookie::build("_my_site", Uuid::new_v4().to_string())
        // .secure(true)
        // .http_only(true)
        // .same_site(SameSite::None)
        // .domain("example.com")
        .path("/")
        .max_age(Duration::days(365))
        .finish();

    return Ok(HttpResponse::Ok()
        .header("Set-Cookie", cookie.to_string())
        .body(""));
}
