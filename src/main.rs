#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;
mod dtos;

mod services;
mod controllers;
mod catchers;

mod db_connection;

use controllers::{create_todo_item_controller, get_todo_item_controller};
use catchers::unprocessable_entity_catcher;

#[launch]
fn rocket() -> _ {
    rocket::build().register("/", catchers![
        unprocessable_entity_catcher
    ]).mount("/todo-item", routes![
        create_todo_item_controller,
        get_todo_item_controller
    ])
}