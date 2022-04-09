use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct EmailConfig {
    /// Sender email address
    pub sender: String,
    /// Username for SMTP server
    pub username: String,
    /// Password for SMTP server
    pub password: String,
    /// SMTP server address
    pub server: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    // Application settings
    pub domain_name: String,
    /// Account activation URL. Account activation email will use this link.
    pub activation_url: String,
    /// IP address to be used for HTTP Server.
    pub ip: String,
    /// Port to be used for HTTP Server.
    pub port: u16,

    // Database settings
    pub db_host: String,
    pub db_name: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,

    // Email settings
    pub email: EmailConfig,
}
