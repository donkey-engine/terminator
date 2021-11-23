use actix_web::{HttpRequest, HttpResponse};

pub async fn subscribe_route(_request: HttpRequest) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body("Hello world")
}
