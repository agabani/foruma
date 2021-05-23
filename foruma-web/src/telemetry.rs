use tracing::{subscriber, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn init(subscriber: impl Subscriber + Send + Sync) {
    subscriber::set_global_default(subscriber).expect("setting tracing default failed.");
}

pub fn configure(level: &str) -> impl Subscriber + Send + Sync {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))
        .unwrap();

    let fmt_layer = BunyanFormattingLayer::new("foruma.api".to_string(), std::io::stdout);

    Registry::default()
        .with(filter_layer)
        .with(JsonStorageLayer)
        .with(fmt_layer)
}

pub trait TraceErrorExt<T, E: std::fmt::Display> {
    fn trace_err(self) -> Result<T, E>;
}

impl<T, E: std::fmt::Display> TraceErrorExt<T, E> for Result<T, E> {
    fn trace_err(self) -> Result<T, E> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                tracing::error!(error = %e);
                Err(e)
            }
        }
    }
}
