use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;

use crate::dtos::Response;
use crate::models::{TodoItem, NewTodoItem};
use crate::services::todo_item_service;

#[post("/create", format = "application/json", data="<new_todo_item>")]
pub fn create_todo_item_controller(new_todo_item: Json<NewTodoItem>) -> Custom<Json<TodoItem>> {
    let todo_item_to_insert = new_todo_item.into_inner();

    let created_todo_item = todo_item_service::create_todo_item(todo_item_to_insert).unwrap();

    Custom(Status::Created, Json(created_todo_item))
}

#[get("/<item_id>")]
pub fn get_todo_item_controller(item_id: i32) -> Custom<Json<Response<TodoItem>>> {

    match todo_item_service::get_todo_item(item_id) {
        Some(todo_item) => {
            Custom(
                Status::Ok,
                Json(Response::success(todo_item))
            )
        },

        None => {
            Custom(
                Status::NotFound,
                Json(Response::failure("Todo item by such id does not exist"))
            )
        }
    }
}