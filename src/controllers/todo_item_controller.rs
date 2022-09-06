use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;

use crate::dtos::Response;
use crate::models::{TodoItem, NewTodoItem};
use crate::services::todo_item_service;

#[post("/create", data="<new_todo_item>")]
pub fn create_todo_item_controller(new_todo_item: Json<NewTodoItem>) -> Custom<Json<TodoItem>> {
    let todo_item_to_insert = new_todo_item.into_inner();

    let created_todo_item = todo_item_service::create_todo_item(todo_item_to_insert).unwrap();

    Custom(Status::Created, Json(created_todo_item))
}

#[get("/<item_id>")]
pub fn get_todo_item_controller(item_id: i32) -> Custom<Json<Response<TodoItem>>> {
    let mut response = Response::<TodoItem>::new();

    let status;

    match todo_item_service::get_todo_item(item_id) {
        Some(todo_item) => {
            response.sucess = true;
            response.data = Some(todo_item);

            status = Status::Ok;
        },

        None => {
            response.message = Some(
                String::from("Todo item by such id does not exist")
            );

            status = Status::NotFound;
        }
    };

    Custom(status, Json(response))
}