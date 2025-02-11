mod api;
mod matchmaking;
mod redis_handler;

use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(api::config)
    })
    .bind("127.0.0.1:8080")?
        .run()
        .await
}