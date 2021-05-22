use crate::configuration::Configuration;
use crate::context::Context;
use crate::routes::{account, authentication, health};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub fn run(overrides: &[(&str, &str)]) -> (Server, u16, Configuration) {
    let configuration = Configuration::load(overrides).expect("Failed to load configuration");

    let listener = configuration
        .http_server
        .tcp_listener()
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();

    let postgres_pool = configuration.postgres.database_pool();

    let context = Context::new(postgres_pool.clone());

    let data_configuration = web::Data::new(configuration.clone());
    let data_context = web::Data::new(context);
    let data_key = web::Data::new(cookie::Key::generate());
    let data_postgres_pool = web::Data::new(postgres_pool);

    let server = HttpServer::new(move || {
        App::new()
            .service(web::scope("/health").configure(health::config))
            .service(
                web::scope("/api")
                    .service(web::scope("/account").configure(account::config))
                    .service(web::scope("/authentication").configure(authentication::config)),
            )
            .app_data(data_configuration.clone())
            .app_data(data_context.clone())
            .app_data(data_key.clone())
            .app_data(data_postgres_pool.clone())
    })
    .listen(listener)
    .expect("Failed to bind address")
    .run();

    (server, port, configuration)
}
