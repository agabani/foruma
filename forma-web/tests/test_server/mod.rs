pub struct TestServer {
    pub address: String,
}

impl TestServer {
    pub fn spawn(overrides: &[(&str, &str)]) -> Self {
        let defaults = &[("http_server.host", "127.0.0.1"), ("http_server.port", "0")];

        let (server, port) = forma_web::run(&[defaults, overrides].concat());

        let _ = tokio::spawn(server);

        Self {
            address: format!("http://127.0.0.1:{}", port),
        }
    }
}
