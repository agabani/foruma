mod change_password;
mod login;
mod logout;
mod signup;
mod whoami;

use actix_web::{http::Method, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/change-password")
            .route(web::method(Method::OPTIONS).to(change_password::option))
            .route(web::method(Method::POST).to(change_password::post)),
    )
    .service(
        web::resource("/login")
            .route(web::method(Method::OPTIONS).to(login::option))
            .route(web::method(Method::POST).to(login::post)),
    )
    .service(
        web::resource("/logout")
            .route(web::method(Method::OPTIONS).to(logout::option))
            .route(web::method(Method::POST).to(logout::post)),
    )
    .service(
        web::resource("/signup")
            .route(web::method(Method::OPTIONS).to(signup::option))
            .route(web::method(Method::POST).to(signup::post)),
    )
    .service(
        web::resource("/whoami")
            .route(web::method(Method::OPTIONS).to(whoami::option))
            .route(web::method(Method::GET).to(whoami::get)),
    );
}
