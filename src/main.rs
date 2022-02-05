// use crate::endpoints;
use crate::config::Config;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::{fs, path::Path};

mod config;
mod endpoints;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read config file
    let config_file_path = Path::new("./config.toml");
    let cfg = read_config(config_file_path).await;

    let shared_state = web::Data::new(state::State::new(cfg.clone()));

    let ip_port = format!("{}:{}", cfg.ip, cfg.port);
    println!("Starting server at: {}", ip_port);

    HttpServer::new(move || {
        App::new().app_data(shared_state.clone()).service(
            web::scope("/api/v1")
                .route(
                    "/urls/{urlcode}",
                    web::get().to(endpoints::get_shortened_url),
                )
                .route("/urls", web::post().to(endpoints::post_url))
                .service(web::scope("admin").route("", web::get().to(|| HttpResponse::Ok()))),
        )
    })
    .bind(ip_port)?
    .run()
    .await
}

/// Reads configuration file and returns an instance of [Config] struct
async fn read_config<P: AsRef<Path>>(config_file_path: P) -> Config {
    // TODO: Handle error properly
    let toml_data = fs::read_to_string(config_file_path).unwrap();

    // TODO: Handle error properly
    let config = toml::from_str(&toml_data).unwrap();
    config
}
