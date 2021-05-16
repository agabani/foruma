use config::Config;
use serde::{Deserialize, Deserializer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub http_server: HttpServer,
    pub cors: Option<Cors>,
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

#[derive(serde::Deserialize)]
pub struct HttpServer {
    pub host: String,
    pub port: u16,
}

impl HttpServer {
    pub fn tcp_listener(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.host, self.port))
    }
}

#[derive(serde::Deserialize)]
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
