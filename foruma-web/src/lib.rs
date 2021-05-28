extern crate argon2;

mod configuration;
mod context;
mod cors;
mod domain;
mod middleware;
mod routes;
mod startup;
pub mod telemetry;

pub use startup::run;
