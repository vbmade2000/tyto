// use crate::endpoints;
use actix_web::{web, App, HttpResponse, HttpServer};

mod endpoints;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    // Read config file
    let cfg = state::Config {
        filename: "test.toml".to_owned(),
    };

    let shared_state = web::Data::new(state::State::new(cfg.clone()));

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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
