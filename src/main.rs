#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;
mod dtos;

mod enums;
mod catchers;
mod services;
mod controllers;

mod db_connection;

use controllers::todo_item_controller;
use catchers::unprocessable_entity_catcher;

#[launch]
fn rocket() -> _ {
    rocket::build().register("/", catchers![
        unprocessable_entity_catcher
    ]).mount("/todo-item", routes![
        todo_item_controller::get_todo_item,
        todo_item_controller::create_todo_item,
        todo_item_controller::update_todo_item,
        todo_item_controller::delete_todo_item
    ])
}