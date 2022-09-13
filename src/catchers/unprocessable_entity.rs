use rocket::serde::json::Json;
use rocket::response::status::Custom;
use rocket::http::Status;

use crate::dtos::ErrorResponse;

#[catch(422)]
pub fn unprocessable_entity_catcher() -> Custom<Json<ErrorResponse>> {

    Custom(
        Status::BadRequest,
        Json(
            ErrorResponse::new("One or more of the provided fields is of wrong type")
        )
    )
}