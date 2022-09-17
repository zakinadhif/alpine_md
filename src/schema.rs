table! {
    notes (id) {
        id -> Int4,
        owner -> Varchar,
        title -> Varchar,
        body -> Text,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}
