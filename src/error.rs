use crate::types;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use base64::DecodeError;
use snafu::prelude::*;
use sqlx::migrate::MigrateError;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Database Error: {}", source))]
    Database { source: sqlx::Error },

    #[snafu(display("Configuration Error: {}", source))]
    ConfigFile { source: std::io::Error },

    #[snafu(display("Configuration Error: {}", source))]
    ConfigRead { source: toml::de::Error },

    #[snafu(display("Email sending Error: {}", source))]
    Email {
        source: lettre::transport::smtp::Error,
    },

    #[snafu(display("Invalid email"))]
    InvalidEmail,

    #[snafu(display("Account is already activated."))]
    AccountAlreadyActivated,

    #[snafu(display("Invalid activation token."))]
    InvalidActivationToken,

    #[snafu(display("Database migration failed."))]
    MigrationFailed { source: MigrateError },

    #[snafu(display("User not found."))]
    UserNotFound,

    #[snafu(display("Error in base64 decoding a string"))]
    Base64Decode { source: DecodeError },

    #[snafu(display("Invalid or expired token. Please login again obtain new token"))]
    InvalidToken { source: jwt_simple::Error },

    #[snafu(display("Token time must be between 1 to 60 minutes"))]
    InvalidTokenExpirationTime,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        use Error::*;
        let status = match self {
            Database { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ConfigFile { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ConfigRead { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Email { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            MigrationFailed { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEmail => StatusCode::BAD_REQUEST,
            InvalidActivationToken => StatusCode::BAD_REQUEST,
            AccountAlreadyActivated => StatusCode::CONFLICT,
            UserNotFound => StatusCode::NOT_FOUND,
            Base64Decode { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidToken { source: _ } => StatusCode::UNAUTHORIZED,
            InvalidTokenExpirationTime => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let response = types::Response {
            status: types::Status::Failure,
            message: Some(self.to_string()),
            data: serde_json::from_str("{}").unwrap(),
        };

        HttpResponse::build(status).json(response)
    }
}

impl From<sqlx::Error> for Error {
    fn from(source: sqlx::Error) -> Error {
        Error::Database { source }
    }
}

impl From<toml::de::Error> for Error {
    fn from(source: toml::de::Error) -> Error {
        Error::ConfigRead { source }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Error {
        Error::ConfigFile { source }
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(source: lettre::transport::smtp::Error) -> Error {
        Error::Email { source }
    }
}

impl From<MigrateError> for Error {
    fn from(source: MigrateError) -> Error {
        Error::MigrationFailed { source }
    }
}

impl From<DecodeError> for Error {
    fn from(source: DecodeError) -> Error {
        Error::Base64Decode { source }
    }
}

impl From<jwt_simple::Error> for Error {
    fn from(source: jwt_simple::Error) -> Error {
        Error::InvalidToken { source }
    }
}
