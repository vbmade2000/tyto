use crate::state::State;
use actix_web::{http, web::Path};
use actix_web::{web, Responder};
use md5;
use serde::{Deserialize, Serialize};
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
    data: String,
}

/// Removes URL from a data structure
pub fn _remove_url(url: String) {
    println!("Remove {} from a data structure", url);
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
            data: v.clone(),
        })
    } else {
        web::Json(Response {
            status: Status::FAILURE,
            message: Some("URL not found".to_owned()),
            data: "".to_owned(),
        })
    }
}

/// Web handler - POST
/// Creates a new shortened URL for supplied longer URL
pub async fn post_url(input: web::Json<URLRequest>, state: web::Data<State>) -> impl Responder {
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
    .await
    .unwrap();
    format!("{}/{}", &state.config.domain_name, short_url)
}

/// Returns a shortened version of a URL
pub async fn shorten_url_md5(long_url: String) -> String {
    format!("{:?}", md5::compute(long_url))
}
