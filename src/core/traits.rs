use crate::types::CreateUserRequest;
use crate::{error, types::User};
use async_trait::async_trait;

#[async_trait()]
pub trait UserManager {
    async fn create(&self, user: CreateUserRequest) -> Result<i64, error::Error>;
    async fn get(&self, user_id: i64) -> Result<User, error::Error>;
    async fn get_all(&self) -> Result<Vec<User>, error::Error>;
    async fn delete(&self, user_id: i64) -> Result<(), error::Error>;
}

/// A trait that must be implemented by all the concrete types used to notify people
/// in some way.
#[async_trait()]
pub trait Notifier {
    /// Sends an email
    async fn send(&self) -> Result<(), error::Error>;
}
