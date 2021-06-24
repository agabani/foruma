use crate::{
    configuration,
    configuration::Configuration,
    context::Context,
    middleware::SessionId,
    routes::{account, authentication, health},
};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{
    http::{header, Method},
    web, App, HttpServer,
};
use tracing_actix_web::TracingLogger;

/// # Panics
///
/// Will panic if configuration cannot be fully loaded due to missing environment variables
pub fn run(overrides: &[(&str, &str)]) -> (Server, u16, Configuration) {
    let configuration = Configuration::load(overrides).expect("Failed to load configuration");

    let listener = configuration
        .http_server
        .tcp_listener()
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();

    let postgres_pool = configuration.postgres.database_pool();

    let context = Context::new(postgres_pool.clone());

    let data_context = web::Data::new(context);

    let key = actix_web::cookie::Key::generate();

    let data_key = web::Data::new(key.clone());
    let data_postgres_pool = web::Data::new(postgres_pool);

    let origins = configuration.cors.as_ref().map_or_else(
        || "".to_string(),
        configuration::Cors::comma_separated_origins,
    );

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&origins)
                    .allowed_methods(vec![Method::DELETE, Method::GET, Method::POST])
                    .allowed_headers(vec![header::CONTENT_TYPE])
                    .supports_credentials(),
            )
            .wrap(TracingLogger::default())
            .wrap(SessionId::new(key.clone()))
            .service(web::scope("/health").configure(health::config))
            .service(
                web::scope("/api/v1")
                    .service(web::scope("/account").configure(account::config))
                    .service(web::scope("/authentication").configure(authentication::config)),
            )
            .app_data(data_context.clone())
            .app_data(data_key.clone())
            .app_data(data_postgres_pool.clone())
    })
    .listen(listener)
    .expect("Failed to bind address")
    .run();

    (server, port, configuration)
}
