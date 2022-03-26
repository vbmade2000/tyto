extern crate serde_json;

use crate::config::Config;
use crate::core::traits::Notifier;
use crate::emailer::EmailNotifier;
use actix_web::{web, App, HttpResponse, HttpServer};
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
async fn main() -> std::io::Result<()> {
    // Read config file
    let config_file_path = Path::new("./config.toml");
    let cfg = read_config(config_file_path).await;

    let db_connection_string = db::get_db_conn_string(cfg.clone()).await;

    // Database migration
    let db_connection = db::get_database_connection(db_connection_string)
        .await
        .unwrap();
    sqlx::migrate!("./migrations")
        .run(&db_connection)
        .await
        .unwrap();

    let state = state::State::new(cfg.clone(), db_connection);
    let shared_state = web::Data::new(state);
    let user_manager = web::Data::new(TytoUserManager::new(shared_state.clone()));

    let ip_port = format!("{}:{}", cfg.ip, cfg.port);
    println!("Starting server at: {}", ip_port);

    // Send an email
    let emailer = EmailNotifier::new(
        cfg,
        "mlvora.2010@gmail.com".to_string(),
        "vbmade2000@gmail.com".to_string(),
        "Test Subject".to_string(),
        "Test body".to_string(),
    );

    let a = emailer.send().await;
    println!("{:?}", a);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .app_data(user_manager.clone())
            .service(
                web::scope("/api/v1")
                    .route("/urls/{id}", web::get().to(endpoints::get_shortened_url))
                    .route("/urls", web::post().to(endpoints::post_url))
                    .route("/urls", web::get().to(endpoints::get_urls))
                    .route("/urls/{id}", web::delete().to(endpoints::delete_url))
                    .service(web::scope("admin").route("", web::get().to(HttpResponse::Ok))),
            )
            .service(web::scope("").route("/health", web::get().to(endpoints::health)))
    })
    .bind(ip_port)?
    .run()
    .await
}

/// Reads configuration file and returns an instance of [Config] struct
async fn read_config<P: AsRef<Path>>(config_file_path: P) -> Config {
    // TODO: Handle error properly
    let toml_data = fs::read_to_string(config_file_path)
        .expect("Error in reading configuration from {config_file_path}");

    // TODO: Handle error properly
    toml::from_str::<Config>(&toml_data)
        .expect("Error in parsing configuration file {config_file_path}")
}
