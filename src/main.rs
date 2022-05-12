extern crate serde_json;

use crate::config::Config;
use actix_web::{web, App, HttpResponse, HttpServer};
use error::Error;
use sqlx::{self};
use std::{fs, path::Path};
use user_management::TytoUserManager;

mod config;
mod core;
mod db;
mod emailer;
mod endpoints;
mod error;
mod state;
mod types;
mod user_management;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // Read config file
    // TODO: Read config file path from cmdline.
    let config_file_path = Path::new("./config.toml");
    let cfg = read_config(config_file_path).await?;

    let db_connection_string = db::get_db_conn_string(cfg.clone()).await;

    // Database migration
    let db_connection_pool = db::get_database_connection(db_connection_string).await?;
    sqlx::migrate!("./migrations")
        .run(&db_connection_pool)
        .await?;

    let state = state::State::new(cfg.clone(), db_connection_pool);
    let shared_state = web::Data::new(state);
    let user_manager = web::Data::new(TytoUserManager::new(shared_state.clone()));
    let confg = web::Data::new(cfg.clone());

    let ip_port = format!("{}:{}", cfg.ip, cfg.port);
    println!("Starting server at: {}", ip_port);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .app_data(user_manager.clone())
            .app_data(confg.clone())
            .service(
                web::scope("/api/v1")
                    .route(
                        "/urls/{id}",
                        web::get().to(endpoints::urls::get_shortened_url),
                    )
                    .route("/urls", web::post().to(endpoints::urls::post_url))
                    .route("/urls", web::get().to(endpoints::urls::get_urls))
                    .route("/urls/{id}", web::delete().to(endpoints::urls::delete_url))
                    .route("/users", web::post().to(endpoints::users::create_user))
                    .route(
                        "/users/activate/{code}",
                        web::patch().to(endpoints::users::activate),
                    )
                    .route("/users", web::get().to(endpoints::users::get_all_users))
                    .service(web::scope("admin").route("", web::get().to(HttpResponse::Ok))),
            )
            .service(web::scope("").route("/health", web::get().to(endpoints::health::health)))
    })
    .bind(ip_port)?
    .run()
    .await?;
    Ok(())
}

/// Reads configuration file and returns an instance of [Config] struct
async fn read_config<P: AsRef<Path>>(config_file_path: P) -> Result<Config, Error> {
    // TODO: Handle error properly
    // let toml_text = fs::read_to_string(config_file_path)
    //     .expect("Error in reading configuration from {config_file_path}");

    // TODO: Handle error properly
    // toml::from_str::<Config>(&toml_text)
    //     .expect("Error in parsing configuration file {config_file_path}")

    let toml_text = fs::read_to_string(config_file_path)?;

    let c: Config = toml::from_str::<Config>(&toml_text)?;
    Ok(c)
}
