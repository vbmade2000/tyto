use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    // Application settings
    pub domain_name: String,
    pub ip: String,
    pub port: u16,

    // Database settings
    pub db_host: String,
    pub db_name: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
}
