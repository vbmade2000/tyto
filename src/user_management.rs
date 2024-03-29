use crate::constants;
use crate::error;
use crate::types::CreateUserRequest;
use crate::types::LoginRequest;
use crate::types::UserClaim;
use crate::utils::validate_token;
use crate::{core::traits::UserManager, state::State, types::User};
use actix_web::web;
use async_trait::async_trait;
use jwt_simple::prelude::*;
use rand::Rng;

pub struct TytoUserManager {
    state: web::Data<State>,
}

/// Generates activation code.
/// How does it work:
/// It generates a random number in the range of u32, concatentes it with md5(email) and returns the
/// resultant string.
fn generate_activation_code(email: &str) -> String {
    // Generate random number to use in activation code to make it difficult to predict/re-create.
    let mut rng = rand::thread_rng();
    let random_number = rng.gen::<u32>();

    // Generate activation code
    // activation_code = md5(email + random_number)
    format!(
        "{:?}",
        md5::compute(email.to_owned() + &random_number.to_string())
    )
}

#[async_trait()]
impl UserManager for TytoUserManager {
    /// Creates a new user.
    /// How does it work:
    /// At the moment is takes email and password of a user and stores in database. It stores password in
    /// plain text which is indeed a bad practice and soon it will be changed to store securely. It then
    /// generates activation code and returns it.
    async fn create(&self, user: CreateUserRequest) -> Result<(i64, String), error::Error> {
        /*
            IDEA: Instead of storing activation timestamp in database, we can have a
            timestamp value here, encrypt it using symmetric encryption and append it
            to the activation code. Something like this.
            activation_code = md5(email + random_number) + encrypt(timestamp)
            This way the tradeoff is speed will be affected as we encrypt data here
            and storage space will be saved because we do not store timestamp in database.
            When we want to check expiration time, we can just extract appended timestamp,
            decrypt it and compare.
        */

        let db_connection = &self.state.db_connection;
        let activation_code = generate_activation_code(&user.email);

        let rec = sqlx::query!(
            r#"INSERT INTO tyto.users (email,password, activation_code) VALUES ($1,$2,$3) RETURNING id"#,
            user.email,
            user.password,
            activation_code
        )
        .fetch_one(db_connection)
        .await?;
        Ok((rec.id, activation_code))
    }

    /// Returns an instance of a [User] for a user with supplied id.
    /// How does it work:
    /// It simply retrieves the record of a supplied user id and returns instance of [User] structure
    /// populated with record values.
    async fn get(&self, user_id: i64) -> Result<User, error::Error> {
        // TODO: Return error if a user is deleted.
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

    /// Returns all the users in database.
    /// How does it work?
    /// It simply retrieves all the user records from the database and returns a list of [User]
    /// instances
    async fn get_all(&self) -> Result<Vec<User>, error::Error> {
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

    /// Deletes a user with supplied id.
    /// How does it work:
    /// It simply deletes a user from database.
    async fn delete(&self, user_id: i64) -> Result<(), error::Error> {
        let db_connection = &self.state.db_connection;
        let result = sqlx::query!(
            r#"DELETE FROM tyto.users where id=$1 RETURNING id"#,
            user_id
        )
        .fetch_optional(db_connection)
        .await?;
        if result.is_none() {
            return Err(error::Error::UserNotFound);
        }

        Ok(())
    }

    /// Activates a user with supplied id.
    /// How does it work:
    /// 1. Check if length is as expected. Return error if not.
    /// 2. Check if user is already activated. Return error if not.
    /// 3. Activate the user.
    async fn activate(&self, activation_code: String) -> Result<(), error::Error> {
        // Activation code must be of fixed and predefined length.
        if activation_code.len() != constants::user::ACTIVATION_CODE_LENGTH {
            return Err(error::Error::InvalidActivationToken);
        }

        // Fetch user record
        let db_connection = &self.state.db_connection;
        let user_record = sqlx::query!(
            r#"SELECT activated from tyto.users WHERE activation_code=$1"#,
            activation_code
        )
        .fetch_one(db_connection)
        .await?;

        // Do not activate already activated account.
        if user_record.activated {
            return Err(error::Error::AccountAlreadyActivated);
        }

        // Activate un-activated account
        sqlx::query!(
            r#"UPDATE tyto.users SET activated=true WHERE activation_code=$1"#,
            activation_code
        )
        .execute(db_connection)
        .await?;

        Ok(())
    }

    /// Logs in the user and returns a JWT on success.
    /// 1. Fetch user record for a matching credentials. Return error if no records found.
    /// 2. Create user claim to be encoded in JWT.
    /// 3. Generate JWT and return it.
    async fn login(&self, login_request: LoginRequest) -> Result<String, error::Error> {
        let db_connection = &self.state.db_connection;
        let _user_record = sqlx::query!(
            r#"SELECT id, email, activated from tyto.users WHERE email=$1 and password=$2"#,
            login_request.email,
            login_request.password
        )
        .fetch_one(db_connection)
        .await?;

        // User claim that we'll encode in token.
        let user_claim = UserClaim {
            email: login_request.email.clone(),
            role: "regular".to_string(),
        };

        // Generate JWT
        let expires_in_minutes = self.state.config.auth.minutes;
        let claim =
            Claims::with_custom_claims(user_claim, Duration::from_mins(expires_in_minutes as u64));
        let token = self.state.jwt_key.authenticate(claim)?;

        Ok(token)
    }

    /// Allows user to logout.
    /// Practically the logout is not possible. Client can just remove existing token from the
    /// local storage. Still we return new token with that belongs to no user or role. Also, that
    /// token would expire in 2 milliseconds.
    async fn logout(&self, token: String) -> Result<String, error::Error> {
        let mut existing_claim = validate_token(&token, &self.state.jwt_key).await?;
        existing_claim.email = "".to_string();
        existing_claim.role = "".to_string();

        let claim = Claims::with_custom_claims(existing_claim, Duration::from_millis(2_u64));
        let token = self.state.jwt_key.authenticate(claim)?;
        Ok(token)
    }
}

impl TytoUserManager {
    /// Creates a new instance of concrete [TytoUserManager]
    pub fn new(state: web::Data<State>) -> Self {
        TytoUserManager { state }
    }
}
