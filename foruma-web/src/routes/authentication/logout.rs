use crate::configuration::Configuration;
use crate::context::Context;
use crate::cookie::{SessionCookie, SessionCookieHttpRequest, SessionCookieHttpResponseBuilder};
use crate::cors::Cors;
use crate::domain::LogOut;
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

#[tracing::instrument(skip(configuration))]
pub async fn option(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("http request");

    Ok(HttpResponse::Ok()
        .insert_access_control_headers(&configuration, &http_request)
        .insert_preflight_access_control_headers(&[Method::POST])
        .finish())
}

#[tracing::instrument(skip(configuration, context, key))]
pub async fn post(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
    context: web::Data<Context>,
    key: web::Data<cookie::Key>,
) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("http request");

    let cookie = http_request.decrypt_session_cookie(&key);
    if cookie.is_none() {
        return Ok(HttpResponse::Unauthorized()
            .insert_access_control_headers(&configuration, &http_request)
            .finish());
    }

    let mut cookie = cookie.unwrap();
    context.log_out(&cookie.session_id()).await;

    return Ok(HttpResponse::Ok()
        .insert_access_control_headers(&configuration, &http_request)
        .delete_session_cookie(&mut cookie)
        .finish());
}
