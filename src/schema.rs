// @generated automatically by Diesel CLI.

diesel::table! {
    #[sql_name = "todo-items"]
    todo_items (id) {
        id -> Int4,
        title -> Varchar,
        content -> Nullable<Text>,
        created -> Timestamp,
        done -> Bool,
    }
}
