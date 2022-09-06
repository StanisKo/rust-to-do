use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub sucess: bool,
    pub message: Option<String>,
    pub data: Option<T>
}

impl<T> Response<T> {

    pub fn success(body: T) -> Self {
        Response { sucess: true, message: None, data: Some(body) }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Response { sucess: false, message: Some(message.into()), data: None }
    }
}