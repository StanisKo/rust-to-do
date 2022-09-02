use crate::schema::todo_items;

use diesel::{Queryable, Identifiable, Insertable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "todo_items"]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created: std::time::SystemTime,
    pub done: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "todo_items"]
pub struct NewTodoItem {
    pub title: String,
    pub content: Option<String>,
    pub created: std::time::SystemTime,
}
