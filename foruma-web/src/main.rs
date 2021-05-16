use foruma_web::{run, telemetry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init(telemetry::configure("info"));

    let (server, _, configuration) = run(&[]);

    configuration.postgres.migrate().await;

    server.await
}
