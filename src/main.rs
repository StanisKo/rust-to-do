#[macro_use] extern crate rocket;

mod handlers;

use handlers::index_route;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_route])
}