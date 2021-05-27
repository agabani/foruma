use crate::configuration::Configuration;
use crate::context::Context;
use crate::cookie::{SessionCookie, SessionCookieHttpResponseBuilder};
use crate::cors::Cors;
use crate::domain::{
    CreateAccount, CreateAccountError, CreatePassword, LogIn, LogInError, Password, Username,
};
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    username: String,
    password: String,
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
    let username = Username::new(&request.username);
    let password = Password::new(&request.password);

    let account = context.create_account(&username).await;
    let account = match account {
        Ok(account) => account,
        Err(CreateAccountError::AccountAlreadyExists) => {
            tracing::warn!("TODO: gracefully handle account creation");
            return Ok(HttpResponse::Unauthorized()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
    };

    context.create_password(&account, &password).await;

    let session_id = context.log_in(&username, &password).await;
    let session_id = match session_id {
        Ok(session_id) => session_id,
        Err(LogInError::AccountDoesNotExist) => {
            return Ok(HttpResponse::Unauthorized()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
        Err(LogInError::IncorrectPassword) => {
            return Ok(HttpResponse::Unauthorized()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
        Err(LogInError::AccountHasNoPassword) => {
            return Ok(HttpResponse::Unauthorized()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
    };

    let cookie = SessionCookie::new(&session_id);

    Ok(HttpResponse::Ok()
        .encrypt_session_cookie(&key, cookie)
        .insert_access_control_headers(&configuration, &http_request)
        .finish())
}
