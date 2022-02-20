use serde::Serialize;
use serde_json::{self, value};

#[derive(Serialize)]
pub enum Status {
    SUCCESS,
    FAILURE,
}

#[derive(Serialize)]
pub struct Response {
    pub status: Status,
    pub message: Option<String>,
    pub data: value::Value,
}
