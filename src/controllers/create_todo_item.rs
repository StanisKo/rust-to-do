use rocket::serde::json::Json;

use crate::models::{TodoItem, NewTodoItem};
use crate::services::todo_item_service;

#[post("/todo-item/create", data="<new_todo_item>")]
pub fn create_todo_item_controller(new_todo_item: Json<NewTodoItem>) -> Json<TodoItem> {
    let todo_item_to_insert = new_todo_item.into_inner();

    let created_todo_item = todo_item_service::create_todo_item(todo_item_to_insert).unwrap();

    Json(created_todo_item)
}