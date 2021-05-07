use forma_web::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (server, _) = run(&[
        ("http_server.host", "0.0.0.0"),
        ("http_server.port", "8080"),
    ]);

    server.await
}
