use std::time::Duration;

use crate::config;
use crate::error::Error;
use sqlx::{self, postgres::PgPoolOptions, Pool, Postgres};

/// Returns database connection strings using parameters received from config
pub async fn get_db_conn_string(cfg: &config::Config) -> String {
    // postgresql://[user[:password]@][netloc][:port][/dbname][?param1=value1&...]

    // let database_urll = "postgres://tyto@localhost:5432/tyto";
    if !cfg.db_password.is_empty() {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            cfg.db_user, cfg.db_password, cfg.db_host, cfg.db_port, cfg.db_name
        )
    } else {
        format!(
            "postgres://{}@{}:{}/{}",
            cfg.db_user, cfg.db_host, cfg.db_port, cfg.db_name
        )
    }
}

pub async fn get_database_connection(conn_string: &str) -> Result<Pool<Postgres>, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect(conn_string)
        .await?;
    Ok(pool)
}
