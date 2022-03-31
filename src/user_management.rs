use crate::error;
use actix_web::web;
use async_trait::async_trait;

use crate::{core::traits::UserManager, state::State, types::User};

pub struct TytoUserManager {
    state: web::Data<State>,
}

#[async_trait()]
impl UserManager for TytoUserManager {
    /// Creates a new user
    async fn create(&self, user: User) -> Result<i64, error::Error> {
        let db_connection = &self.state.db_connection;
        let rec = sqlx::query!(
            r#"INSERT INTO tyto.users (banned,email,password) VALUES ($1,$2,$3) RETURNING id"#,
            false,
            user.email,
            user.password,
        )
        .fetch_one(db_connection)
        .await?;
        Ok(rec.id)
    }

    /// Returns an instance of [User] for a user with supplied id
    async fn get(&self, user_id: i64) -> Result<User, error::Error> {
        // TODO: Return error if user is deleted.
        let db_connection = &self.state.db_connection;
        let user = sqlx::query!(r#"SELECT * FROM tyto.users WHERE id=$1"#, user_id)
            .fetch_one(db_connection)
            .await?;

        Ok(User {
            id: Some(user.id),
            apikey: user.apikey,
            email: user.email,
            banned: user.banned,
            password: user.password,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    /// Returns all the users in database. At the moment, it doesn't differentiate between
    /// live users and deleted users.
    async fn get_all(&self) -> Result<Vec<User>, error::Error> {
        // TODO: Add support to get only live users. Currently it returns all the users.
        let db_connection = &self.state.db_connection;
        let found_users = sqlx::query!(r#"SELECT * FROM tyto.users ORDER BY created_at ASC"#)
            .fetch_all(db_connection)
            .await?;

        let users: Vec<User> = found_users
            .into_iter()
            .map(|user| User {
                id: Some(user.id),
                apikey: user.apikey,
                email: user.email,
                banned: user.banned,
                password: user.password,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
            .collect::<Vec<User>>();

        Ok(users)
    }

    /// Deletes a user with supplied id
    async fn delete(&self, user_id: i64) -> Result<(), error::Error> {
        // TODO: Return error if user is already deleted.
        let db_connection = &self.state.db_connection;
        sqlx::query!(r#"UPDATE tyto.users set deleted=true where id=$1"#, user_id)
            .execute(db_connection)
            .await?;
        Ok(())
    }
}

impl TytoUserManager {
    /// Creates a new instance of concrete [TytoUserManager]
    pub fn new(state: web::Data<State>) -> Self {
        TytoUserManager { state }
    }
}
