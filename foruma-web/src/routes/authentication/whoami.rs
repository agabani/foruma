use crate::configuration::Configuration;
use crate::context::Context;
use crate::cors::Cors;
use crate::domain::{GetAccount, SessionId};
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

#[derive(serde::Serialize)]
struct Response {
    id: String,
    username: String,
}

pub async fn option(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .insert_access_control_headers(&configuration, &http_request)
        .insert_preflight_access_control_headers(&[Method::GET])
        .finish())
}

pub async fn get(
    http_request: HttpRequest,
    context: web::Data<Context>,
    configuration: web::Data<Configuration>,
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

    let account = match context.get_account(session_id).await {
        Some(account) => account,
        None => {
            return Ok(HttpResponse::Unauthorized()
                .insert_access_control_headers(&configuration, &http_request)
                .finish());
        }
    };

    return Ok(HttpResponse::Ok()
        .insert_access_control_headers(&configuration, &http_request)
        .json(Response {
            id: account.account_id().value().to_string(),
            username: account.username().value().to_string(),
        }));
}
