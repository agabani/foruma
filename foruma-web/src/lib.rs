extern crate argon2;

mod configuration;
mod context;
mod domain;
mod middleware;
mod routes;
mod startup;
pub mod telemetry;

pub use startup::run;
