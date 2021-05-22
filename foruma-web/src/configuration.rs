use config::Config;
use serde::{Deserialize, Deserializer};
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::Pool;
use std::net::TcpListener;

#[derive(Clone, serde::Deserialize)]
pub struct Configuration {
    pub http_server: HttpServer,
    pub cors: Option<Cors>,
    pub postgres: Postgres,
}

impl Configuration {
    pub fn load(overrides: &[(&str, &str)]) -> Result<Configuration, config::ConfigError> {
        let mut config = Config::default();

        config.merge(config::Environment::with_prefix("APP").separator("__"))?;

        for &(key, value) in overrides {
            config.set(key, value)?;
        }

        config.try_into()
    }
}

#[derive(Clone, serde::Deserialize)]
pub struct HttpServer {
    pub host: String,
    pub port: u16,
}

impl HttpServer {
    pub fn tcp_listener(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.host, self.port))
    }
}

#[derive(Clone, serde::Deserialize)]
pub struct Cors {
    #[serde(deserialize_with = "comma_separated_values_deserializer")]
    pub origins: Vec<String>,
}

fn comma_separated_values_deserializer<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    Ok(string.split(',').map(|item| item.to_owned()).collect())
}

#[derive(Clone, serde::Deserialize)]
pub struct Postgres {
    pub database_name: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub require_ssl: bool,
    pub username: String,
    pub migration: Option<PostgresMigration>,
}

#[derive(Clone, serde::Deserialize)]
pub struct PostgresMigration {
    pub create_database: bool,
    pub path: String,
}

impl Postgres {
    pub fn server_pool(&self) -> Pool<sqlx::Postgres> {
        PgPoolOptions::new().connect_lazy_with(self.server_connect_options())
    }

    pub fn database_pool(&self) -> Pool<sqlx::Postgres> {
        PgPoolOptions::new().connect_lazy_with(self.database_connect_options())
    }

    fn server_connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .ssl_mode(match self.require_ssl {
                true => PgSslMode::Require,
                false => PgSslMode::Prefer,
            })
    }

    fn database_connect_options(&self) -> PgConnectOptions {
        self.server_connect_options().database(&self.database_name)
    }

    pub async fn migrate(&self) {
        if let Some(migration) = &self.migration {
            let migrator = sqlx::migrate::Migrator::new(std::path::Path::new(&migration.path))
                .await
                .expect("TODO");

            let database_pool = self.database_pool();

            let uri = format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database_name
            );

            if migration.create_database
                && !sqlx::Postgres::database_exists(&uri).await.expect("TODO")
            {
                sqlx::Postgres::create_database(&uri).await.expect("TODO");
            }

            migrator.run(&database_pool).await.expect("TODO");
        }
    }
}
