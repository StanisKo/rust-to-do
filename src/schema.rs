// @generated automatically by Diesel CLI.

diesel::table! {
    todo_items (id) {
        id -> Int4,
        title -> Varchar,
        content -> Nullable<Text>,
        created -> Timestamp,
        done -> Bool,
    }
}
