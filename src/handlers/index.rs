#[get("/")]
pub fn index_route() -> &'static str {
    "Hello, world!"
}