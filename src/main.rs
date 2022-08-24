#[macro_use] extern crate rocket;

mod handlers;

use handlers::index_handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_handler])
}

/*
On diesel:

https://betterprogramming.pub/using-the-diesel-orm-for-a-web-app-with-rocket-90e610f6a6cf
*/