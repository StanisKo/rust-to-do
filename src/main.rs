#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;

mod services;
mod controllers;

mod db_connection;

use controllers::index_controller;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_controller])
}