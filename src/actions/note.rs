use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::{NotePayload, NewNote, Note};
use crate::schema::notes::dsl::*;

pub fn create_note(db: &mut PgConnection, payload: &NotePayload) -> Result<Note, diesel::result::Error> {
    let new_note = NewNote {
        title: &payload.title,
        body: &payload.body,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now()
    };

    diesel::insert_into(notes)
        .values(&new_note)
        .get_result(db)
}

pub fn delete_note(db: &mut PgConnection, note_id: i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(notes.find(note_id))
        .execute(db)
}

pub fn update_note(db: &mut PgConnection, note_id: i32, payload: &NotePayload) -> Result<Note, diesel::result::Error> {
    diesel::update(notes.find(note_id))
        .set((
            title.eq(&payload.title),
            body.eq(&payload.body),
            updated_at.eq(chrono::Utc::now())
        ))
        .get_result(db)
}

pub fn get_note(db: &mut PgConnection, note_id: i32) -> Result<Note, diesel::result::Error> {
    notes.find(note_id)
        .get_result(db)
}

pub fn index_notes(db: &mut PgConnection) -> Result<Vec<Note>, diesel::result::Error> {
    notes.load(db)
}
