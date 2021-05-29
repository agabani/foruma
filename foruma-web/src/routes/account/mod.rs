mod terminate;
use actix_web::{http::Method, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/terminate").route(web::method(Method::POST).to(terminate::post)));
}
