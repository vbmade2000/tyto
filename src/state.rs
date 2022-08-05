use crate::config::Config;
use jwt_simple::prelude::HS256Key;
use sqlx::{self, Pool, Postgres};

#[derive(Clone)]
pub struct State {
    pub config: Config,
    pub db_connection: sqlx::Pool<Postgres>,
    pub jwt_key: HS256Key,
}

impl State {
    pub fn new(config: Config, db_connection: Pool<Postgres>) -> State {
        let key = config.auth.key.clone();
        let key = base64::decode(key).expect("Failed to base64-decode JWT key");
        let key = HS256Key::from_bytes(&key);

        State {
            config,
            db_connection,
            jwt_key: key,
        }
    }
}
