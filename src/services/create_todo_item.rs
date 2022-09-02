use crate::db_connection;

use crate::schema::todo_items;
use crate::models::{TodoItem, NewTodoItem};

use diesel::prelude::*;
use diesel::result::Error;


pub fn create_todo_item_service(new_todo_item: NewTodoItem) -> Result<TodoItem, Error> {
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