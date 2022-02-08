use crate::config::Config;
use sqlx::{self, Pool, Postgres};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct State {
    pub urls: Mutex<HashMap<String, String>>,
    pub config: Config,
    pub db_connection: sqlx::Pool<Postgres>,
}

impl State {
    pub fn new(config: Config, db_connection: Pool<Postgres>) -> State {
        State {
            config,
            urls: Mutex::default(),
            db_connection: db_connection,
        }
    }
}
