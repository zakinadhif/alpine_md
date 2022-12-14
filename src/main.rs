#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use awc::Client;
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use env_logger::Env;

use handlers::note;

mod extractors;
mod types;
mod models;
mod middlewares;
mod schema;
mod actions;
mod handlers;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn api_banner() -> impl Responder {
    "AlpineMD API"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let bind_ip = std::env::var("BIND_IP").expect("BIND_IP must be defined");
    let bind_port = std::env::var("BIND_PORT")
        .expect("BIND_PORT must be defined")
        .parse()
        .expect("BIND_PORT must be a valid port");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Client::default()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(middlewares::cors())
            .wrap(Logger::default())
            .route("/", web::get().to(api_banner))
            .service(
                web::scope("notes")
                    .route("", web::post().to(note::create_note))
                    .route("", web::get().to(note::index_notes))
                    .route("{id}", web::get().to(note::get_note))
                    .route("{id}", web::put().to(note::update_note))
                    .route("{id}", web::delete().to(note::delete_note))
            )
    })
    .bind((bind_ip, bind_port))?
    .run()
    .await
}
