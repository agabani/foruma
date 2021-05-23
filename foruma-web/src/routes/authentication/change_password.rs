use crate::configuration::Configuration;
use crate::context::Context;
use crate::cookie::{SessionCookie, SessionCookieHttpRequest, SessionCookieHttpResponseBuilder};
use crate::cors::Cors;
use crate::domain::{ChangePassword, GetAccount, Password};
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    #[serde(rename = "oldPassword")]
    old_password: String,

    #[serde(rename = "newPassword")]
    new_password: String,
}

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
    request: web::Json<Request>,
) -> Result<HttpResponse, actix_web::Error> {
    let old_password = Password::new(&request.old_password);
    let new_password = Password::new(&request.new_password);

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

    context
        .change_password(&account, &old_password, &new_password)
        .await
        .expect("TODO");

    Ok(HttpResponse::Ok()
        .encrypt_session_cookie(&key, cookie)
        .insert_access_control_headers(&configuration, &http_request)
        .finish())
}
