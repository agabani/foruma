mod login;
mod signup;

use actix_web::{http::Method, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::method(Method::POST).to(login::post)))
        .service(web::resource("/signup").route(web::method(Method::POST).to(signup::post)));
}
