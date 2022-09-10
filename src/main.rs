use actix_web::{web, App, HttpServer};
use handlers::note;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("notes")
                .route("", web::post().to(note::create_note))
                .route("", web::get().to(note::index_notes))
                .route("{id}", web::get().to(note::get_note))
                .route("{id}", web::put().to(note::update_note))
                .route("{id}", web::delete().to(note::delete_note)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
