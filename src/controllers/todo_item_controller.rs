use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::status::Custom;

use crate::enums::Lookup;
use crate::dtos::Response;
use crate::services::TodoItemService;
use crate::models::{TodoItem, NewTodoItem};

#[post("/create", format = "json", data="<new_todo_item>")]
pub fn create_todo_item(new_todo_item: Json<NewTodoItem>) -> Custom<Json<Response<TodoItem>>> {

    let todo_item_service = TodoItemService::new();

    let todo_item_to_insert = new_todo_item.into_inner();

    if todo_item_to_insert.title.is_empty() {
        return Custom(
            Status::BadRequest,
            Json(Response::failure("Title is missing from the request"))
        );
    }

    let todo_item_already_exists = todo_item_service.check_if_todo_item_exists(
        Lookup::Title(&todo_item_to_insert.title)
    );

    if todo_item_already_exists {
        return Custom(
            Status::BadRequest,
            Json(Response::failure("Todo item already exists"))
        );
    }

    match todo_item_service.create_todo_item(todo_item_to_insert) {
        Ok(todo_item) => {
            Custom(
                Status::Created,
                Json(Response::success(todo_item))
            )
        },

        Err(_) => {
            Custom(
                Status::BadRequest,
                Json(Response::failure("Creating todo item failed due to internal reasons"))
            )
        }
    }
}

#[get("/<item_id>")]
pub fn get_todo_item(item_id: i32) -> Custom<Json<Response<TodoItem>>> {

    let todo_item_service = TodoItemService::new();

    match todo_item_service.get_todo_item(item_id) {
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
