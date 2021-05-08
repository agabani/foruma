use crate::configuration::Configuration;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run(overrides: &[(&str, &str)]) -> (Server, u16) {
    let configuration = Configuration::load(overrides).expect("Failed to load configuration");

    let listener = configuration
        .http_server
        .tcp_listener()
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || {
        App::new()
            .route("/health/liveness", web::get().to(HttpResponse::Ok))
            .route("/health/readiness", web::get().to(HttpResponse::Ok))
    })
    .listen(listener)
    .expect("Failed to bind address")
    .run();

    (server, port)
}
