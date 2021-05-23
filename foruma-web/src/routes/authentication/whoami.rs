use crate::configuration::Configuration;
use crate::context::Context;
use crate::cookie::{SessionCookie, SessionCookieHttpRequest, SessionCookieHttpResponseBuilder};
use crate::cors::Cors;
use crate::domain::GetAccount;
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

#[derive(serde::Serialize)]
struct Response {
    id: String,
    username: String,
}

#[tracing::instrument(skip(configuration))]
pub async fn option(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .insert_access_control_headers(&configuration, &http_request)
        .insert_preflight_access_control_headers(&[Method::GET])
        .finish())
}

#[tracing::instrument(skip(configuration, context, key))]
pub async fn get(
    http_request: HttpRequest,
    context: web::Data<Context>,
    configuration: web::Data<Configuration>,
    key: web::Data<cookie::Key>,
) -> Result<HttpResponse, actix_web::Error> {
    let cookie = http_request.decrypt_session_cookie(&key);
    if cookie.is_none() {
        return Ok(HttpResponse::Unauthorized()
            .insert_access_control_headers(&configuration, &http_request)
            .finish());
    }

    let mut cookie = cookie.unwrap();
    let session_id = cookie.session_id();

    let account = context.get_account(&session_id).await;
    if account.is_none() {
        return Ok(HttpResponse::Unauthorized()
            .insert_access_control_headers(&configuration, &http_request)
            .delete_session_cookie(&mut cookie)
            .finish());
    }

    let account = account.unwrap();

    return Ok(HttpResponse::Ok()
        .insert_access_control_headers(&configuration, &http_request)
        .json(Response {
            id: account.account_id().value().to_string(),
            username: account.username().value().to_string(),
        }));
}
