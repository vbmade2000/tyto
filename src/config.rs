use serde::Deserialize;

/// Email configuration
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

/// Authentication configuration
#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    /// Base64 encoded key to be used in JWT of length 12
    pub key: String,
    /// Minutes token remains valid for. Minimum 1 minute and maximun 60 minutes are allowed.
    pub minutes: u8,
}

/// Tyto configuration
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    // Application settings
    /// Domain name to be used in generated tiny urls
    pub domain_name: String,
    /// Account activation URL. Account activation email will use this link.
    pub activation_url: String,
    /// IP address to be used for HTTP Server.
    pub ip: String,
    /// Port to be used for HTTP Server.
    pub port: u16,

    // Database settings
    /// Database host
    pub db_host: String,
    /// Database name
    pub db_name: String,
    /// Database port
    pub db_port: u16,
    /// Username for the database connection
    pub db_user: String,
    /// Password for the database connection
    pub db_password: String,

    /// Email settings
    pub email: EmailConfig,

    /// Auth settings
    pub auth: AuthConfig,
}
