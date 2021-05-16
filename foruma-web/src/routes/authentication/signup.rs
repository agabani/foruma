use crate::configuration::Configuration;
use crate::context::Context;
use crate::cookie::{SessionCookie, SessionCookieHttpResponseBuilder};
use crate::cors::Cors;
use crate::domain::{CreateAccount, CreatePassword, LogIn, Password, Username};
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

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

#[tracing::instrument(skip(configuration, context, key, request))]
pub async fn post(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
    context: web::Data<Context>,
    key: web::Data<cookie::Key>,
    request: web::Json<Request>,
) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("http request");

    let username = Username::new(&request.username);
    let password = Password::new(&request.password);

    let account = context.create_account(&username).await;
    context.create_password(&account, &password).await;

    let session_id = context.log_in(&username, &password).await;
    if session_id.is_none() {
        return Ok(HttpResponse::Unauthorized()
            .insert_access_control_headers(&configuration, &http_request)
            .finish());
    }

    let session_id = session_id.unwrap();
    let cookie = SessionCookie::new(&session_id);

    Ok(HttpResponse::Ok()
        .encrypt_session_cookie(&key, cookie)
        .insert_access_control_headers(&configuration, &http_request)
        .finish())
}
