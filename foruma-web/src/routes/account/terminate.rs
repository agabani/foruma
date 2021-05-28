use crate::configuration::Configuration;
use crate::context::Context;
use crate::cors::Cors;
use crate::domain::{GetAccount, SessionId, TerminateAccount, TerminateAccountError};
use actix_web::http::Method;
use actix_web::{web, HttpRequest, HttpResponse};

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
) -> Result<HttpResponse, actix_web::Error> {
    let extensions = http_request.extensions();
    let session_id = match extensions.get::<SessionId>() {
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
                .finish())
        }
    };

    match context.terminate_account(&account).await {
        Ok(()) => Ok(HttpResponse::Ok()
            .insert_access_control_headers(&configuration, &http_request)
            .finish()),
        Err(TerminateAccountError::AccountDoesNotExist) => Ok(HttpResponse::BadRequest()
            .insert_access_control_headers(&configuration, &http_request)
            .finish()),
    }
}
