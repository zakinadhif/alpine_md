/**
 * Models
 * ======
 *
 * Naming Convention
 * -----------------
 * Each table will have four models, each with its own purpose:
 *  - Table: A Queryable struct
 *  - NewTable: An Insertable struct
 *  - TablePayload: A Deserialize struct that is used to contain http request body
 *  - TableForm: A AsChangeset struct that is used to perform update or patch operation
 */

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::AsChangeset;

use crate::schema::notes;

#[derive(Queryable, Serialize)]
pub struct Note {
    pub id: i32,
    pub owner: String,
    pub title: String,
    pub body: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    pub owner: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct NotePayload {
    pub title: String,
    pub body: String
}

#[derive(AsChangeset)]
#[diesel(table_name = notes)]
pub struct NoteForm<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>
}
