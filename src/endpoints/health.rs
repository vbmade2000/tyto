use crate::error::Error;
use crate::types;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

/// Web handler - /health
/// Returns health of the system
pub async fn health() -> Result<HttpResponse, Error> {
    // TODO: Add verbose query string parameter. When `verbose` is supplied,
    // endpoint will health of other external components too, like Database server etc.
    // Also, `verbose` requires Admin level permissions.
    let response = types::Response {
        status: types::Status::Success,
        message: None,
        data: serde_json::to_value("{}").unwrap(),
    };
    Ok(HttpResponse::build(StatusCode::OK).json(response))
}
