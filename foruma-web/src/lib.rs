#![warn(clippy::pedantic)]

extern crate argon2;

mod configuration;
mod context;
mod domain;
mod geoip;
mod graphql;
mod http_request_ext;
mod middleware;
mod routes;
mod startup;
pub mod telemetry;

pub use startup::run;
