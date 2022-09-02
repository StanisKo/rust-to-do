#[get("/")]
pub fn index_controller() -> &'static str {
    "Hello world!"
}