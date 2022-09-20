use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::status::Custom;

use crate::dtos::Response;
use crate::enums::{Lookup, Filter};
use crate::services::TodoItemService;
use crate::models::{TodoItem, NewTodoItem, UpdatedTodoItem};

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
                Status::InternalServerError,
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


#[put("/update/<item_id>", format = "json", data="<updated_todo_item>")]
pub fn update_todo_item(item_id: i32, updated_todo_item: Json<UpdatedTodoItem>) -> Custom<Json<Response<TodoItem>>> {

    let todo_item_service = TodoItemService::new();

    let updated_todo_item = updated_todo_item.into_inner();

    match todo_item_service.get_todo_item(item_id) {

        Some(mut todo_item) => {

            todo_item = todo_item.merge(updated_todo_item);

            match todo_item_service.update_todo_item(&todo_item) {

                Ok(todo_item_after_update) => {
                    Custom(
                        Status::Ok,
                        Json(Response::success(todo_item_after_update))
                    )
                },

                Err(_) => {
                    Custom(
                        Status::InternalServerError,
                        Json(Response::failure("Updating todo item failed due to internal reasons"))
                    )
                }
            }
        },

        None => {
            Custom(
                Status::NotFound,
                Json(Response::failure("Todo item by such id does not exist"))
            )
        }
    }
}

#[delete("/delete/<item_id>")]
pub fn delete_todo_item(item_id: i32) -> Custom<Json<Response<TodoItem>>> {

    let todo_item_service = TodoItemService::new();

    if todo_item_service.check_if_todo_item_exists(Lookup::Id(&item_id)) {

        match todo_item_service.delete_todo_item(item_id) {

            Ok(deleted_todo_item) => {
                return Custom(
                    Status::Ok,
                    Json(Response::success(deleted_todo_item))
                )
            },

            Err(_) => {
                return Custom(
                    Status::InternalServerError,
                    Json(Response::failure("Deleting todo item failed due to internal reasons"))
                );
            }
        }
    }

    Custom(
        Status::NotFound,
        Json(Response::failure("Todo item by such id does not exist"))
    )
}

#[get("/list/<filter>/<page>")]
pub fn get_todo_item_list(filter: &str, page: i32) -> Custom<Json<Response<Vec<TodoItem>>>> {

    let todo_item_service = TodoItemService::new();

    let filter = match filter {
        "done" => Filter::Done,

        "active" => Filter::Active,

        _ => {
            return Custom(
                Status::BadRequest,
                Json(Response::failure("Provided filter is not supported"))
            );
        }
    };

    match todo_item_service.get_todo_items_list(page, filter) {
        Ok(todo_items_list) => {
            Custom(
                Status::Ok,
                Json(Response::success(todo_items_list))
            )
        },

        Err(_) => {
            Custom(
                Status::InternalServerError,
                Json(Response::failure("Fetching todo item list failed due to internal reasons"))
            )
        }
    }
}