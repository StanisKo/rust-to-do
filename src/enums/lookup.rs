pub enum Lookup<'a> {
    Id(&'a i32),

    Title(&'a str)
}