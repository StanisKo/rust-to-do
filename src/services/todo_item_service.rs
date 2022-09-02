use crate::db_connection;

use crate::schema::todo_items;
use crate::models::{TodoItem, NewTodoItem};

use diesel::prelude::*;
use diesel::result::Error;


pub fn create_todo_item(new_todo_item: NewTodoItem) -> Result<TodoItem, Error> {
    /*
    We explicitly don't handle connection errors since, if any, we want runtime error
    */
    let connection = db_connection::create_connection();

    /*
    If anything goes belly up here, we'd like to resolve it to 500 upstream
    */
    let transaction_result: Result<TodoItem, Error> = connection.transaction(|| {
        diesel::insert_into(todo_items::table).values(
            &new_todo_item
        ).get_result(&connection)
    });

    transaction_result
}

pub fn get_todo_item(item_id: i32) -> Result<TodoItem, Error> {
    let connection = db_connection::create_connection();

    todo_items::table.find(item_id).get_result::<TodoItem>(&connection)
}