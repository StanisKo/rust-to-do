use crate::schema::todo_items;

use diesel::{Queryable, Identifiable};

use serde_json;
use rocket::serde::{Deserialize, Serialize};


#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "todo_items"]
pub struct TodoItem {
    pub id: i32,
}