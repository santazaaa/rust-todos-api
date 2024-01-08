// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Uuid,
        text -> Text,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
