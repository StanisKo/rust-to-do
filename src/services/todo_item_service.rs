use crate::db_connection;

use crate::schema::todo_items;
use crate::models::{TodoItem, NewTodoItem, UpdatedTodoItem};

use diesel::prelude::*;
use diesel::result::Error;

pub struct TodoItemService {
    connection: PgConnection
}

impl TodoItemService {

    pub fn new() -> Self {

        Self { connection: db_connection::create_connection() }
    }

    pub fn create_todo_item(&self, new_todo_item: NewTodoItem) -> Result<TodoItem, Error> {
    
        let transaction_result: Result<TodoItem, Error> = self.connection.transaction(|| {
            diesel::insert_into(todo_items::table).values(
                &new_todo_item
            ).get_result(&self.connection)
        });

        transaction_result
    }

    pub fn get_todo_item(&self, item_id: i32) -> Option<TodoItem> {

        match todo_items::table.find(item_id).get_result::<TodoItem>(&self.connection) {
            Ok(todo_item) => Some(todo_item),

            Err(_) => None
        }
    }

    pub fn check_if_todo_item_exists_by_title(&self, title: &String) -> bool {

        match todo_items::table.filter(todo_items::title.eq(title)).count().get_result::<i64>(&self.connection) {
            Ok(count) => {

                if count >= 1 { return true } else { return false }
            },

            Err(_) => {
                false
            }
        }
    }

    pub fn check_if_todo_item_exists_by_id(&self, id: i32) -> bool {

        match todo_items::table.filter(todo_items::id.eq(id)).count().get_result::<i64>(&self.connection) {
            Ok(count) => {

                if count >= 1 { return true } else { return false }
            },

            Err(_) => {
                false
            }
        }
    }

    pub fn update_todo_item(&self, updated_todo_item: UpdatedTodoItem) -> Result<TodoItem, Error> {

        let transaction_result: Result<TodoItem, Error> = diesel::update(
            todo_items::table.find(updated_todo_item.id)
        ).set(
            (
                todo_items::columns::title.eq(updated_todo_item.title),
                todo_items::columns::content.eq(updated_todo_item.content),
            )
        ).get_result(&self.connection);

        transaction_result
    }
}