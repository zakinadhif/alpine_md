use actix_web::Responder;

pub async fn create_note() -> impl Responder {
    "Hello from create_note()!"
}

pub async fn get_note() -> impl Responder {
    "Hello from get_note()!"
}

pub async fn index_notes() -> impl Responder {
    "Hello from index_notes()"
}

pub async fn update_note() -> impl Responder {
    "Hello from update_note"
}

pub async fn delete_note() -> impl Responder {
    "Hello from delete_note()"
}
