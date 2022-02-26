use chrono::{DateTime, Utc};
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

#[derive(Serialize)]
pub struct Link {
    pub id: i32,
    pub address: String,
    pub description: Option<String>,
    pub banned: bool,
    pub target: String,
    pub visit_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
