use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{self, value};

/// Represents a request status
#[derive(Serialize)]
pub enum Status {
    Success,
    Failure,
}

#[derive(Serialize)]
pub struct Response {
    pub status: Status,
    pub message: Option<String>,
    pub data: value::Value,
}

/// A struct to represent a single URL record
#[derive(Serialize)]
pub struct Link {
    pub id: i32,
    pub user_id: i32,
    pub address: String,
    pub description: Option<String>,
    pub banned: bool,
    pub target: String,
    pub visit_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A struct used to represent a request input for /urls POST
#[derive(Deserialize)]
pub struct CreateURLRequest {
    pub target: String,
    pub description: Option<String>,
    pub banned: bool,
    pub user_id: i32,
}
