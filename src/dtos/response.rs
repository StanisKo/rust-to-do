use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub sucess: bool,
    pub message: Option<String>,
    pub data: Option<T>
}

impl<T> Response<T> {

    pub fn new() -> Self {
        Response { sucess: false, message: None, data: None }
    }
}