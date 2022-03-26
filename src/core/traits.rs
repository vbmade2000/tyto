use crate::{error, state::State, types::User};
use actix_web::web;
use async_trait::async_trait;

#[async_trait()]
pub trait UserManager {
    /// Creates a new instance of concrete [UserManager]
    fn new(state: web::Data<State>) -> Self;
    async fn create(&self, user: User) -> Result<(), error::Error>;
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
