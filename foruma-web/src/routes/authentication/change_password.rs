use crate::context::Context;
use crate::domain::{ChangePassword, ChangePasswordError, GetAccount, Password, SessionId};
use actix_web::{web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    #[serde(rename = "currentPassword")]
    current_password: String,

    #[serde(rename = "newPassword")]
    new_password: String,
}

pub async fn post(
    http_request: HttpRequest,
    context: web::Data<Context>,
    request: web::Json<Request>,
) -> Result<HttpResponse, actix_web::Error> {
    let current_password = Password::new(&request.current_password);
    let new_password = Password::new(&request.new_password);

    let extensions = http_request.extensions();
    let session_id = match extensions.get::<SessionId>() {
        Some(session_id) => session_id,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let account = match context.get_account(&session_id).await {
        Some(account) => account,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    match context
        .change_password(&account, &current_password, &new_password)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(ChangePasswordError::AccountDoesNotExist) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
        Err(ChangePasswordError::IncorrectPassword) => {
            return Ok(HttpResponse::BadRequest().finish());
        }
        Err(ChangePasswordError::AccountHasNoPassword) => {
            return Ok(HttpResponse::Forbidden().finish());
        }
    }
}
