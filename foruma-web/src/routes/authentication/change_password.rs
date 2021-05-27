use crate::configuration::Configuration;
use crate::context::Context;
use crate::cookie::{SessionCookie, SessionCookieHttpRequest};
use crate::cors::Cors;
use crate::domain::{ChangePassword, ChangePasswordError, GetAccount, Password};
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    #[serde(rename = "currentPassword")]
    current_password: String,

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
    let current_password = Password::new(&request.current_password);
    let new_password = Password::new(&request.new_password);

    let session_id = match http_request
        .decrypt_session_cookie(&key)
        .map(|cookie| cookie.session_id())
    {
        Some(session_id) => session_id,
        None => {
            return Ok(HttpResponse::Unauthorized()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
    };

    let account = match context.get_account(&session_id).await {
        Some(account) => account,
        None => {
            return Ok(HttpResponse::Unauthorized()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
    };

    match context
        .change_password(&account, &current_password, &new_password)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok()
            .insert_access_control_headers(&configuration, &http_request)
            .finish()),
        Err(ChangePasswordError::AccountDoesNotExist) => {
            return Ok(HttpResponse::Unauthorized()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
        Err(ChangePasswordError::IncorrectPassword) => {
            return Ok(HttpResponse::BadRequest()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
        Err(ChangePasswordError::AccountHasNoPassword) => {
            return Ok(HttpResponse::Forbidden()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
    }
}
