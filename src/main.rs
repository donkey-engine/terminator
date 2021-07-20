use actix::{Addr, ArbiterService};
use actix_web::{web, middleware::Logger, App, HttpServer};
use terminator::handlers::subscribe::subscribe_route;
use terminator::storage::ServerStorate;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data_factory(|| async {
                Ok::<Addr<ServerStorate>, ()>(ServerStorate::from_registry())
            })
            .route("/subscribe", web::post().to(subscribe_route))
    })
    .bind(("127.0.0.1", 8585))?
    .workers(1)
    .run()
    .await
}
