use crate::error::Error;
use crate::state::State;
use crate::types::{self, Link};
use actix_web::http::StatusCode;
use actix_web::web::Path;
use actix_web::{web, HttpResponse, Responder};
use chrono::DateTime;
use chrono::Utc;
use md5;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{self, value};
use sqlx;

/// A struct used to represent request input for /urls POST
#[derive(Deserialize)]
pub struct URLRequest {
    target: String,
    description: Option<String>,
    banned: bool,
}

/// Represents a response status
#[derive(Serialize)]
enum Status {
    SUCCESS,
    FAILURE,
}

/// A struct used to represent response output for /urls GET
#[derive(Serialize)]
pub struct Response {
    status: Status,
    message: Option<String>,
    data: value::Value,
}

/// Web handler - GET
/// Returns a shortened URL for a longer version
pub async fn get_shortened_url(
    id: Path<i32>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    // Record { id: 1, address: "0a137b375cc3881a70e186ce2172c8d1", description: None, banned: false, target: "www.google.com", visit_count: 0, created_at: 2022-02-26T15:01:42.112443Z, updated_at: 2022-02-26T15:01:42.112443Z }

    let state = state.clone();
    let db_connection = &state.db_connection;
    let id = id.into_inner();

    let link_data = sqlx::query!(r#"SELECT * FROM tyto.links WHERE id=$1"#, id)
        .fetch_one(db_connection)
        .await?;

    let found_link = Link {
        id: link_data.id,
        address: link_data.address,
        description: link_data.description,
        banned: link_data.banned,
        target: link_data.target,
        visit_count: link_data.visit_count,
        created_at: link_data.created_at,
        updated_at: link_data.updated_at,
    };
    // Prepare response
    let response = types::Response {
        status: types::Status::SUCCESS,
        message: None,
        data: serde_json::to_value(found_link).unwrap(),
    };
    Ok(HttpResponse::build(StatusCode::OK).json(response))
}

/// Web handler - POST
/// Creates a new shortened URL for supplied longer URL
pub async fn post_url(
    input: web::Json<URLRequest>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    #[derive(Serialize)]
    struct OutputData {
        pub url: String,
    }

    let state = state.clone();
    let db_connection = &state.db_connection;
    let short_url = shorten_url_md5(input.target.clone()).await;

    // IMP NOTE: DATABASE_URL env var must be set for this to work.
    //           export DATABASE_URL="postgres://tyto@localhost/tyto"
    let _rec = sqlx::query!(
        r#"INSERT INTO tyto.links (address,target,description,banned) VALUES ($1,$2,$3,$4) RETURNING id"#,
        short_url.clone(),
        input.target.clone(),
        input.description,
        input.banned
    )
    .fetch_one(db_connection)
    .await?;

    let output = json!({
        "url": format!("{}/{}", &state.config.domain_name, short_url),
    });

    let response = types::Response {
        status: types::Status::SUCCESS,
        message: None,
        data: output,
    };

    Ok(HttpResponse::build(StatusCode::CREATED).json(response))
}

/// Returns a shortened version of a URL
pub async fn shorten_url_md5(long_url: String) -> String {
    format!("{:?}", md5::compute(long_url))
}

/// Web Handler - GET
/// Returns all the URLs from database
pub async fn get_urls(state: web::Data<State>) -> Result<HttpResponse, Error> {
    let state = state.clone();
    let db_connection = &state.db_connection;

    let links = sqlx::query!(r#"SELECT * FROM tyto.links ORDER BY created_at ASC"#,)
        .fetch_all(db_connection)
        .await?;

    let mut output = Vec::new();
    for link in links {
        output.push(Link {
            id: link.id,
            address: link.address,
            description: link.description,
            banned: link.banned,
            target: link.target,
            visit_count: link.visit_count,
            created_at: link.created_at,
            updated_at: link.updated_at,
        });
    }

    // Prepare response
    let response = types::Response {
        status: types::Status::SUCCESS,
        message: None,
        data: serde_json::to_value(output).unwrap(),
    };

    Ok(HttpResponse::build(StatusCode::OK).json(response))
}
