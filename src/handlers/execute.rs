use crate::facade::ServerFacade;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct ExecuteRequest {
    server_id: usize,
    command: String,
}

#[derive(Serialize)]
pub struct ExecuteResponse {
    status: String,
    response: String,
}

#[derive(Serialize)]
pub struct ExecuteErrorResponse {
    status: String,
    error: String,
}

pub async fn execute_route(
    body: web::Json<ExecuteRequest>,
    facade: web::Data<Mutex<ServerFacade>>,
) -> Result<HttpResponse, Error> {
    let fr = facade.into_inner();
    let mut fl = fr.lock().unwrap();
    Ok(
        match fl.execute(body.server_id, body.command.clone()).await {
            Ok(result) => HttpResponse::Ok().json(ExecuteResponse {
                status: String::from("success"),
                response: result,
            }),
            Err(error) => HttpResponse::BadRequest().json(ExecuteErrorResponse {
                status: String::from("error"),
                error: error.to_string(),
            }),
        },
    )
}
