#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use log::LevelFilter;
use std::sync::Mutex;
use terminator::errors::TerminatorErrors;
use terminator::facade::ServerFacade;
use terminator::handlers::execute::execute_route;
use terminator::handlers::subscribe::subscribe_route;
use terminator::storage::{RedisConfig, RedisStorage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    info!("Running on 127.0.0.1:8585");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/subscribe", web::post().to(subscribe_route))
            .route("/execute", web::post().to(execute_route))
            .data_factory(|| async {
                let redis_storage = RedisStorage::new(RedisConfig {
                    host: "0.0.0.0".to_string(),
                    port: 6379,
                })
                .await?;
                Ok::<Mutex<ServerFacade>, TerminatorErrors>(Mutex::new(ServerFacade::init(
                    redis_storage,
                )))
            })
    })
    .bind(("127.0.0.1", 8585))?
    .workers(1)
    .run()
    .await
}
