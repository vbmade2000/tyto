use serde::Serialize;

#[derive(Serialize)]
pub enum Status {
    SUCCESS,
    FAILURE,
}

#[derive(Serialize)]
pub struct Response {
    pub status: Status,
    pub message: String,
    pub data: Option<String>,
}
