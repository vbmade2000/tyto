use crate::types;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Database Error: {}", source))]
    DatabaseError { source: sqlx::Error },

    #[snafu(display("Configuration Error: {}", source))]
    ConfigFileError { source: std::io::Error },

    #[snafu(display("Configuration Error: {}", source))]
    ConfigReadError { source: toml::de::Error },
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        use Error::*;
        let status = match self {
            DatabaseError { source } => StatusCode::INTERNAL_SERVER_ERROR,
            ConfigFileError { source } => StatusCode::INTERNAL_SERVER_ERROR,
            ConfigReadError { source } => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let response = types::Response {
            status: types::Status::FAILURE,
            message: Some(self.to_string()),
            data: None,
        };

        HttpResponse::build(status).json(response)
    }
}

impl From<sqlx::Error> for Error {
    fn from(source: sqlx::Error) -> Error {
        Error::DatabaseError { source }
    }
}

impl From<toml::de::Error> for Error {
    fn from(source: toml::de::Error) -> Error {
        Error::ConfigReadError { source }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Error {
        Error::ConfigFileError { source }
    }
}
