use crate::db_connection;

use crate::schema::todo_items;
use crate::models::{TodoItem, NewTodoItem};

use diesel::prelude::*;
use diesel::result::Error;


pub fn create_todo_item(new_todo_item: NewTodoItem) -> Result<TodoItem, Error> {
    let connection = db_connection::create_connection();

    let transaction_result: Result<TodoItem, Error> = connection.transaction(|| {
        diesel::insert_into(todo_items::table).values(
            &new_todo_item
        ).get_result(&connection)
    });

    transaction_result
}

pub fn get_todo_item(item_id: i32) -> Option<TodoItem> {
    let connection = db_connection::create_connection();

    match todo_items::table.find(item_id).get_result::<TodoItem>(&connection) {
        Ok(todo_item) => Some(todo_item),

        Err(_) => None
    }
}

pub fn check_if_todo_item_exists(title: &String) -> bool {
    let connection = db_connection::create_connection();

    match todo_items::table.filter(todo_items::title.eq(title)).count().get_result(&connection) {
        Ok(count) => {

            match count {
                1 => true,
                0 => false,
                _ => true
            }
        },

        Err(_) => {
            false
        }
    }
}

pub fn update_todo_item(updated_todo_item: TodoItem) -> Result<TodoItem, Error> {
    let connection = db_connection::create_connection();

    let transaction_result: Result<TodoItem, Error> = diesel::update(
        todo_items::table.find(updated_todo_item.id)
    ).set(
        (
            todo_items::columns::title.eq(updated_todo_item.title),
            todo_items::columns::content.eq(updated_todo_item.content),
        )
    ).get_result(&connection);

    transaction_result
}