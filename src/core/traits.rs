use crate::types::{CreateUserRequest, LoginRequest};
use crate::{error, types::User};
use async_trait::async_trait;

#[async_trait()]
pub trait UserManager {
    async fn create(&self, user: CreateUserRequest) -> Result<(i64, String), error::Error>;
    async fn get(&self, user_id: i64) -> Result<User, error::Error>;
    async fn get_all(&self) -> Result<Vec<User>, error::Error>;
    async fn delete(&self, user_id: i64) -> Result<(), error::Error>;
    async fn activate(&self, activation_code: String) -> Result<(), error::Error>;
    async fn login(&self, login_request: LoginRequest) -> Result<String, error::Error>;
    async fn logout(&self, token: String) -> Result<String, error::Error>;
}

/// A trait that must be implemented by all the concrete types used to notify people
/// in some way.
#[async_trait()]
pub trait Notifier {
    /// Sends an email
    async fn send(&self) -> Result<(), error::Error>;
}
