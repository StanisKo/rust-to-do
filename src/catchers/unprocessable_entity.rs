use rocket::serde::json::Json;
use rocket::response::status::Custom;
use rocket::http::Status;

use crate::dtos::Response;

/*
This typing is irrelevant, yet, creating another DTO just for the sake
of catching miss-typed input feels like an overkill
*/
#[catch(422)]
pub fn unprocessable_entity_catcher() -> Custom<Json<Response<String>>> {
    Custom(
        Status::BadRequest,
        Json(Response::failure("One or more of the provided fields is of wrong type"))
    )
}