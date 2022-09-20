use crate::schema::todo_items;

use chrono::NaiveDateTime;

use diesel::{Queryable, Identifiable, Insertable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "todo_items"]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    // Has default constraint on database side (FALSE)
    pub done: Option<bool>,
    // Has default constraint on database side (CURRENT_TIMESTAMP)
    pub created: Option<NaiveDateTime>
}

impl TodoItem {

    pub fn merge(self, other: UpdatedTodoItem) -> Self {

        Self {
            id: self.id,
            title: other.title.unwrap_or(&self.title).to_string(),
            content: other.content.map(|content| { content.to_string() }).or(self.content),
            done: other.done.or(self.done),
            created: self.created
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "todo_items"]
pub struct NewTodoItem<'a> {
    pub title: &'a str,
    pub content: Option<&'a str>
}

#[derive(Insertable, Deserialize)]
#[table_name = "todo_items"]
pub struct UpdatedTodoItem<'a> {
    pub title: Option<&'a str>,
    pub content: Option<&'a str>,
    pub done: Option<bool>
}
