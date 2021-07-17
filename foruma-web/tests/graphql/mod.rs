#[derive(serde::Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<Error>>,
}

#[derive(serde::Deserialize)]
pub struct Error {
    pub message: String,
    pub locations: Vec<Location>,
    pub path: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}
