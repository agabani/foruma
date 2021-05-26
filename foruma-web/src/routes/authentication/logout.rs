use crate::configuration::Configuration;
use crate::context::Context;
use crate::cookie::{SessionCookie, SessionCookieHttpRequest, SessionCookieHttpResponseBuilder};
use crate::cors::Cors;
use crate::domain::{LogOut, LogoutError};
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

pub async fn option(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .insert_access_control_headers(&configuration, &http_request)
        .insert_preflight_access_control_headers(&[Method::POST])
        .finish())
}

pub async fn post(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
    context: web::Data<Context>,
    key: web::Data<cookie::Key>,
) -> Result<HttpResponse, actix_web::Error> {
    let cookie = http_request.decrypt_session_cookie(&key);
    if cookie.is_none() {
        return Ok(HttpResponse::Unauthorized()
            .insert_access_control_headers(&configuration, &http_request)
            .finish());
    }

    let mut cookie = cookie.unwrap();
    let result = context.log_out(&cookie.session_id()).await;

    match result {
        Ok(()) => Ok(HttpResponse::Ok()
            .insert_access_control_headers(&configuration, &http_request)
            .delete_session_cookie(&mut cookie)
            .finish()),
        Err(LogoutError::SessionDoesNotExist) => Ok(HttpResponse::NotFound()
            .insert_access_control_headers(&configuration, &http_request)
            .delete_session_cookie(&mut cookie)
            .finish()),
    }
}
