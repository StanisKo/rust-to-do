#[macro_use] extern crate rocket;

mod handlers;

use handlers::index_handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_handler])
}