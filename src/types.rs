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
pub struct Url {
    pub id: i64,
    pub user_id: i64,
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
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    /// Email address of a user.
    pub email: String,
    /// Password of a user.
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    /// Unique ID of a user.
    pub id: Option<i64>,
    /// API key user can use to programatically use Tyto APIs. Not in use currently.
    pub apikey: Option<String>,
    /// Shows if user is banned.
    pub banned: bool,
    /// Email address of a user.
    pub email: String,
    /// Password of a user.
    pub password: String,
    /// Timestamp when user is created in database.
    pub created_at: DateTime<Utc>,
    /// Timestamp when user is last updated in database.
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    /// Username of a user
    pub email: String,
    /// Password of a user in plain text
    pub password: String,
}

/// A structure to represent JWT Claim
#[derive(Deserialize, Serialize)]
pub struct UserClaim {
    /// Email of a user
    pub email: String,
    /// Role of a user.
    pub role: String,
}

#[derive(Deserialize, Serialize)]
pub enum UserRole {
    Normal,
    Admin,
}
