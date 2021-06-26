use uuid::Uuid;

pub struct TestServer {
    pub address: String,
}

impl TestServer {
    pub async fn spawn(overrides: &[(&str, &str)]) -> Self {
        let defaults = &[
            ("cors.origins", "http://localhost:8080"),
            ("geo_ip.path", "../geoip"),
            ("http_server.host", "127.0.0.1"),
            ("http_server.port", "0"),
            (
                "postgres.database_name",
                &format!("test-{}", Uuid::new_v4()),
            ),
            ("postgres.host", "127.0.0.1"),
            ("postgres.password", "password"),
            ("postgres.port", "5432"),
            ("postgres.require_ssl", "false"),
            ("postgres.username", "postgres"),
        ];

        let (server, port, configuration) = foruma_web::run(&[defaults, overrides].concat());

        let server_pool = configuration.postgres.server_pool();

        sqlx::query(&format!(
            r#"CREATE DATABASE "{}""#,
            configuration.postgres.database_name
        ))
        .execute(&server_pool)
        .await
        .unwrap();

        let database_pool = configuration.postgres.database_pool();

        sqlx::migrate!("../migrations")
            .run(&database_pool)
            .await
            .unwrap();

        let _ = tokio::spawn(server);

        Self {
            address: format!("http://127.0.0.1:{}", port),
        }
    }
}
