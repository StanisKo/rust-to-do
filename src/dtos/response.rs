use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub sucess: bool,
    pub message: Option<String>,
    pub data: Option<T>
}

impl<T> Response<T> {

    pub fn success(body: T) -> Self {

        Self { sucess: true, message: None, data: Some(body) }
    }

    pub fn failure(message: impl Into<String>) -> Self {

        Self { sucess: false, message: Some(message.into()), data: None }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub sucess: bool,
    pub message: String
}

impl ErrorResponse {

    pub fn new(error_message: impl Into<String>) -> Self {

        Self { sucess: false, message: error_message.into() }
    }
}