use actix::{Addr, Arbiter, SystemService};
use actix_web::{middleware::Logger, web, App, HttpServer};
use terminator::handlers::subscribe::subscribe_route;
use terminator::polling;
use terminator::storage::ServerStorage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Arbiter::spawn(polling::run(ServerStorage::from_registry()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data_factory(|| async move {
                Ok::<Addr<ServerStorage>, ()>(ServerStorage::from_registry())
            })
            .route("/subscribe", web::post().to(subscribe_route))
    })
    .bind(("127.0.0.1", 8585))?
    .workers(1)
    .run()
    .await
}
