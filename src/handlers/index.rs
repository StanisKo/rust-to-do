#[get("/")]
pub fn index_handler() -> &'static str {
    "Hello, world!"
}