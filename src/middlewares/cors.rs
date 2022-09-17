use actix_cors::Cors;
use actix_web::http::{header, Method};

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:5173")
        .allowed_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(86_400)
}
