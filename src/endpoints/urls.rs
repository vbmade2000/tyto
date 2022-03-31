use crate::error::Error;
use crate::state::State;
use crate::types::{self, CreateURLRequest, Url};
use actix_web::{
    http::StatusCode,
    web::{self, Path},
    HttpResponse,
};
use serde::Serialize;
use serde_json::{self, json};

/// Web handler - /urls/{id} - DELETE
/// Deletes a URL record with {id}
pub async fn delete_url(id: Path<i64>, state: web::Data<State>) -> Result<HttpResponse, Error> {
    let state = state.clone();
    let db_connection = &state.db_connection;
    let id = id.into_inner();

    let _ = sqlx::query!(r#"DELETE FROM tyto.urls WHERE id=$1"#, id)
        .execute(db_connection)
        .await?;

    let response = types::Response {
        status: types::Status::Success,
        message: None,
        data: serde_json::to_value("{}").unwrap(),
    };

    Ok(HttpResponse::build(StatusCode::OK).json(response))
}

/// Web handler - /urls/{id}- GET
/// Returns a shortened URL record for a supplied {id}
pub async fn get_shortened_url(
    id: Path<i64>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    // Record { id: 1, address: "0a137b375cc3881a70e186ce2172c8d1", description: None, banned: false, target: "www.google.com", visit_count: 0, created_at: 2022-02-26T15:01:42.112443Z, updated_at: 2022-02-26T15:01:42.112443Z }

    let state = state.clone();
    let db_connection = &state.db_connection;
    let id = id.into_inner();

    let url_data = sqlx::query!(r#"SELECT * FROM tyto.urls WHERE id=$1"#, id)
        .fetch_one(db_connection)
        .await?;

    let found_url = Url {
        id: url_data.id,
        user_id: url_data.user_id,
        address: url_data.address,
        description: url_data.description,
        banned: url_data.banned,
        target: url_data.target,
        visit_count: url_data.visit_count,
        created_at: url_data.created_at,
        updated_at: url_data.updated_at,
    };
    // Prepare response
    let response = types::Response {
        status: types::Status::Success,
        message: None,
        data: serde_json::to_value(found_url).unwrap(),
    };
    Ok(HttpResponse::build(StatusCode::OK).json(response))
}

/// Web handler - /urls- POST
/// Creates a new shortened URL for supplied longer URL
pub async fn post_url(
    input: web::Json<CreateURLRequest>,
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
        r#"INSERT INTO tyto.urls (address,target,description,banned,user_id) VALUES ($1,$2,$3,$4,$5) RETURNING id"#,
        short_url.clone(),
        input.target.clone(),
        input.description,
        input.banned,
        input.user_id,
    )
    .fetch_one(db_connection)
    .await?;

    let output = json!({
        "url": format!("{}/{}", &state.config.domain_name, short_url),
    });

    let response = types::Response {
        status: types::Status::Success,
        message: None,
        data: output,
    };

    Ok(HttpResponse::build(StatusCode::CREATED).json(response))
}

/// Returns a shortened version of a URL
pub async fn shorten_url_md5(long_url: String) -> String {
    format!("{:?}", md5::compute(long_url))
}

/// Web handler - /urls - GET
/// Returns all the URL records from database
pub async fn get_urls(state: web::Data<State>) -> Result<HttpResponse, Error> {
    let state = state.clone();
    let db_connection = &state.db_connection;

    let urls = sqlx::query!(r#"SELECT * FROM tyto.urls ORDER BY created_at ASC"#,)
        .fetch_all(db_connection)
        .await?;

    let mut output = Vec::new();
    for url in urls {
        output.push(Url {
            id: url.id,
            user_id: url.user_id,
            address: url.address,
            description: url.description,
            banned: url.banned,
            target: url.target,
            visit_count: url.visit_count,
            created_at: url.created_at,
            updated_at: url.updated_at,
        });
    }

    // Prepare response
    let response = types::Response {
        status: types::Status::Success,
        message: None,
        data: serde_json::to_value(output).unwrap(),
    };

    Ok(HttpResponse::build(StatusCode::OK).json(response))
}
