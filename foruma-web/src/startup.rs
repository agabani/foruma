use crate::configuration::Configuration;
use crate::routes::{authentication, health};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub fn run(overrides: &[(&str, &str)]) -> (Server, u16) {
    let configuration = Configuration::load(overrides).expect("Failed to load configuration");

    let listener = configuration
        .http_server
        .tcp_listener()
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();

    let configuration = web::Data::new(configuration);
    let key = web::Data::new(cookie::Key::generate());

    let server = HttpServer::new(move || {
        App::new()
            .service(web::scope("/health").configure(health::config))
            .service(
                web::scope("/api")
                    .service(web::scope("/authentication").configure(authentication::config)),
            )
            .app_data(configuration.clone())
            .app_data(key.clone())
    })
    .listen(listener)
    .expect("Failed to bind address")
    .run();

    (server, port)
}
