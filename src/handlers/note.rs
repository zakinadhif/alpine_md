use actix_web::{web, HttpResponse, Error};

use crate::Pool;
use crate::models::NotePayload;
use crate::actions::note as actions;

pub async fn create_note(db: web::Data<Pool>, payload: web::Json<NotePayload>) -> Result<HttpResponse, Error> {
    let mut db = db.get().unwrap();
    let payload = payload.into_inner();

    let note = web::block(move || actions::create_note(&mut db, &payload))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(note))
}

pub async fn get_note(db: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut db = db.get().unwrap();
    let id = id.into_inner();

    let note = web::block(move || actions::get_note(&mut db, id))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(note))
}

pub async fn index_notes(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut db = db.get().unwrap();

    let notes = web::block(move || actions::index_notes(&mut db))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

pub async fn update_note(db: web::Data<Pool>, id: web::Path<i32>, payload: web::Json<NotePayload>) -> Result<HttpResponse, Error> {
    let mut db = db.get().unwrap();
    let id = id.into_inner();
    let payload = payload.into_inner();

    let note = web::block(move || actions::update_note(&mut db, id, &payload))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(note))
}

pub async fn delete_note(db: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut db = db.get().unwrap();
    let id = id.into_inner();

    let count_deleted = web::block(move || actions::delete_note(&mut db, id))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(count_deleted))
}
