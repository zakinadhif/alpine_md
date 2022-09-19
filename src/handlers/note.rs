use std::collections::HashSet;

use actix_web::{web, HttpResponse, Error};

use crate::Pool;
use crate::extractors::claims::Claims;
use crate::models::NotePayload;
use crate::actions::note as actions;

pub async fn create_note(db: web::Data<Pool>, payload: web::Json<NotePayload>, claims: Claims) -> Result<HttpResponse, Error> {
    if !claims.validate_permissions(&HashSet::from(["write:notes".to_string()])) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut db = db.get().unwrap();
    let payload = payload.into_inner();

    let note = web::block(move || actions::create_note(&mut db, &payload, &claims.sub))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(note))
}

pub async fn get_note(pool: web::Data<Pool>, id: web::Path<i32>, claims: Claims) -> Result<HttpResponse, Error> {
    if !claims.validate_permissions(&HashSet::from(["read:notes".to_string()])) {
        return Ok(HttpResponse::Forbidden().finish())
    }

    let mut db = pool.get().unwrap();
    let id = id.into_inner();

    let is_owned_by_user = web::block(move || actions::user_owns_note(&mut db, id, &claims.sub))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if is_owned_by_user {
        let mut db = pool.get().unwrap();

        let note = web::block(move || actions::get_note(&mut db, id))
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(note))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn index_notes(pool: web::Data<Pool>, claims: Claims) -> Result<HttpResponse, Error> {
    if !claims.validate_permissions(&HashSet::from(["read:notes".to_string()])) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut db = pool.get().unwrap();

    let notes = web::block(move || actions::index_notes(&mut db, &claims.sub))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

pub async fn update_note(pool: web::Data<Pool>, id: web::Path<i32>, payload: web::Json<NotePayload>, claims: Claims) -> Result<HttpResponse, Error> {
    if !claims.validate_permissions(&HashSet::from(["update:notes".to_string()])) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut db = pool.get().unwrap();
    let id = id.into_inner();
    let payload = payload.into_inner();

    let is_owned_by_user = web::block(move || actions::user_owns_note(&mut db, id, &claims.sub))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if is_owned_by_user {
        let mut db = pool.get().unwrap();

        let note = web::block(move || actions::update_note(&mut db, id, &payload))
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(note))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn delete_note(pool: web::Data<Pool>, id: web::Path<i32>, claims: Claims) -> Result<HttpResponse, Error> {
    if !claims.validate_permissions(&HashSet::from(["delete:notes".to_string()])) {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut db = pool.get().unwrap();
    let id = id.into_inner();

    let is_owned_by_user = web::block(move || actions::user_owns_note(&mut db, id, &claims.sub))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if is_owned_by_user {
        let mut db = pool.get().unwrap();

        let count_deleted = web::block(move || actions::delete_note(&mut db, id))
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(count_deleted))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
