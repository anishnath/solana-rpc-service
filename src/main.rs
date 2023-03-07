use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::Arc;

mod handlers;
mod rpc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/epoch")
                    .route(web::post().to(handlers::get_epoch_info))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}