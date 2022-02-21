use crate::error::Error;
use crate::state::State;
use crate::types;
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
pub async fn get_shortened_url(urlcode: Path<String>, state: web::Data<State>) -> impl Responder {
    let state = state.clone();
    let urls = &state.urls;
    let urls = urls.lock().unwrap();
    let url_found = urls.get(&urlcode.into_inner());
    if let Some(v) = url_found {
        web::Json(Response {
            status: Status::SUCCESS,
            message: None,
            data: serde_json::from_str(v).unwrap(),
        })
    } else {
        web::Json(Response {
            status: Status::FAILURE,
            message: Some("URL not found".to_owned()),
            data: serde_json::from_str("{}").unwrap(),
        })
    }
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

    #[derive(Serialize)]
    pub struct Link {
        pub id: i32,
        pub address: String,
        pub description: Option<String>,
        pub banned: bool,
        pub target: String,
        pub visit_count: i32,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    let mut output = Vec::new();
    // let mut temp_desc: Option<String>;
    for link in links {
        // temp_desc = link.description;
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
