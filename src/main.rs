use actix_web::{App, HttpServer};

use actix_web::{web, Responder};

async fn hello_world() -> impl Responder {
    "Hello world!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello_world))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
