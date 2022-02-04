use crate::state::State;
use actix_web::{http, web::Path};
use actix_web::{web, Responder};
use md5;
use serde::{Deserialize, Serialize};

/// A struct used to represent request input for /urls POST
#[derive(Deserialize)]
pub struct URLRequest {
    target: String,
}

/// A struct used to represent response output for /urls GET
#[derive(Serialize)]
pub struct URLResponse {
    status_code: u16,
    target: String,
}

/// Adds URL into a data structure
pub fn add_url(long_url: String, short_url: String, state: &State) -> Option<String> {
    let mut urls = state.urls.lock().unwrap();
    urls.insert(short_url.clone(), long_url.clone())
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
        web::Json(URLResponse {
            status_code: http::StatusCode::OK.as_u16(),
            target: v.clone(),
        })
    } else {
        web::Json(URLResponse {
            status_code: http::StatusCode::NOT_FOUND.as_u16(),
            target: "".to_owned(),
        })
    }
}

/// Web handler - POST
/// Creates a new shortened URL for supplied longer URL
pub async fn post_url(input: web::Json<URLRequest>, state: web::Data<State>) -> impl Responder {
    let short_url = shorten_url_md5(input.target.clone()).await;
    add_url(input.target.clone(), short_url.clone(), &state);

    short_url
}

/// Returns a shortened version of a URL
pub async fn shorten_url_md5(long_url: String) -> String {
    println!("shorten_url called");
    format!("{:?}", md5::compute(long_url))
}
