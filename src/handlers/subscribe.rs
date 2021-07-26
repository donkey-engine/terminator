use crate::storage::{NewSubscriber, ServerStorage};
use actix::Addr;
use actix_web::{web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SubscribeResponse {
    status: String,
    detail: Option<String>,
}

#[derive(Deserialize)]
pub struct SubscribeBody {
    server_id: usize,
}

pub async fn subscribe_route(
    _request: HttpRequest,
    server_storage: web::Data<Addr<ServerStorage>>,
    info: web::Json<SubscribeBody>,
) -> impl Responder {
    match server_storage
        .send(NewSubscriber {
            server_id: info.server_id,
        })
        .await
    {
        Ok(_) => web::Json(SubscribeResponse {
            status: "ok".to_string(),
            detail: None,
        }),
        Err(err) => web::Json(SubscribeResponse {
            status: "error".to_string(),
            detail: Some(err.to_string()),
        }),
    }
}
