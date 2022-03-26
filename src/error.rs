use crate::types;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use snafu::prelude::*;

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
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        use Error::*;
        let status = match self {
            Database { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ConfigFile { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ConfigRead { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Email { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
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
