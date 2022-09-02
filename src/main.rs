#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;

mod services;
mod controllers;

mod db_connection;

use controllers::{create_todo_item_controller, get_todo_item_controller};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/todo-item", routes![
        create_todo_item_controller,
        get_todo_item_controller
    ])
}