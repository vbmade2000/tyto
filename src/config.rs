use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub domain_name: String,
    pub ip: String,
    pub port: u16,
}
