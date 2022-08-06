extern crate base64;
extern crate serde_json;

use crate::config::Config;
use actix_web::{web, App, HttpResponse, HttpServer};
use clap::Parser;
use error::Error;
use sqlx::{self};
use std::{fs, path::Path};
use user_management::TytoUserManager;

mod config;
mod constants;
mod core;
mod db;
mod emailer;
mod endpoints;
mod error;
mod state;
mod types;
mod user_management;
pub mod utils;

/// Tiny URL generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct TytoArgs {
    /// Path to the tyto configuration file
    #[clap(short, long, default_value_t = String::from("config.toml"))]
    config: String,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let args = TytoArgs::parse();

    // Read config file path from command line.
    let config_path = fs::canonicalize(args.config)?;
    println!("{:?}", &config_path);
    let cfg = read_config(config_path).await?;

    // Database pool creation
    let db_connection_string = db::get_db_conn_string(&cfg).await;
    let db_connection_pool = db::get_database_connection(&db_connection_string).await?;

    // Database migration
    sqlx::migrate!("./migrations")
        .run(&db_connection_pool)
        .await?;

    let state = state::State::new(cfg.clone(), db_connection_pool);
    let shared_state = web::Data::new(state);
    let shared_user_manager = web::Data::new(TytoUserManager::new(shared_state.clone()));
    let shared_config = web::Data::new(cfg.clone());

    let ip_port = format!("{}:{}", cfg.ip, cfg.port);
    println!("Starting server at: {}", ip_port);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .app_data(shared_user_manager.clone())
            .app_data(shared_config.clone())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/urls")
                            .route("", web::get().to(endpoints::urls::get_urls))
                            .route("", web::post().to(endpoints::urls::post_url))
                            .route("/{id}", web::delete().to(endpoints::urls::delete_url))
                            .route("/{id}", web::get().to(endpoints::urls::get_shortened_url)),
                    )
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(endpoints::users::get_all_users))
                            .route("", web::post().to(endpoints::users::create_user))
                            .route("/{id}", web::delete().to(endpoints::users::delete_user))
                            .route(
                                "/activate/{code}",
                                web::patch().to(endpoints::users::activate),
                            )
                            .route("login", web::post().to(endpoints::users::login))
                            .route("logout", web::post().to(endpoints::users::logout)),
                    )
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
    let toml_text = fs::read_to_string(config_file_path)?;

    let c: Config = toml::from_str::<Config>(&toml_text)?;
    validate_config(&c).await?;
    Ok(c)
}

/// Performs various validations on the values from config file.
async fn validate_config(c: &Config) -> Result<(), Error> {
    if c.auth.minutes < 1 || c.auth.minutes > 60 {
        return Err(error::Error::InvalidTokenExpirationTime);
    }
    Ok(())
}
