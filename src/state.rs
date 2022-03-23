use crate::config::Config;
use sqlx::{self, Pool, Postgres};

#[derive(Clone)]
pub struct State {
    pub config: Config,
    pub db_connection: sqlx::Pool<Postgres>,
}

impl State {
    pub fn new(config: Config, db_connection: Pool<Postgres>) -> State {
        State {
            config,
            db_connection,
        }
    }
}
