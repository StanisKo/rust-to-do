// @generated automatically by Diesel CLI.

diesel::table! {
    todo_items (id) {
        id -> Int4,
        title -> Varchar,
        content -> Nullable<Text>,
        done -> Nullable<Bool>,
        created -> Nullable<Timestamp>,
    }
}
