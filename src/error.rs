use crate::types;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use snafu::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Database Error: {}", source))]
    Database { source: sqlx::Error },

    #[snafu(display("Already exists"))]
    AlreadyExist { _value: String },

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
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        use Error::*;
        let status = match self {
            Database { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ConfigFile { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ConfigRead { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Email { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEmail => StatusCode::BAD_REQUEST,
            AccountAlreadyActivated => StatusCode::CONFLICT,
            AlreadyExist { _value: _ } => StatusCode::CONFLICT,
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
        match source {
            // Handle Database Errors
            // https://docs.rs/sqlx/latest/sqlx/error/trait.DatabaseError.html

            /* IMP: Here x.code() returns a SQLSTATE. Ex. in 23505, 23 is a class
               that indicates Integrity constraint violation.
            *  Check https://en.wikipedia.org/wiki/SQLSTATE for more info.
            */
            // TODO: Use https://crates.io/crates/sqlstate
            sqlx::Error::Database(ref x) => match x.code() {
                Some(e) => {
                    if e == Cow::Borrowed("23505") {
                        Error::AlreadyExist {
                            _value: "".to_owned(),
                        }
                    } else {
                        Error::Database { source }
                    }
                }
                None => Error::Database { source },
            },
            _ => {
                println!("Some other error");
                Error::Database { source }
            }
        }
        // Error::Database { source}
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
