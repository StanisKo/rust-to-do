#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod models;
mod schema;
mod handlers;
mod db_connection;

use handlers::index_handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_handler])
}