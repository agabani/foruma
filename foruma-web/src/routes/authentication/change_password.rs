use crate::{
    context::Context,
    domain::{ChangePassword, ChangePasswordError, GetAccount, Password},
    http_request_ext::HttpRequestExt,
};
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

    let session_id = match http_request.session_id() {
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
        Err(ChangePasswordError::AccountDoesNotExist) => Ok(HttpResponse::Unauthorized().finish()),
        Err(ChangePasswordError::IncorrectPassword) => Ok(HttpResponse::BadRequest().finish()),
        Err(ChangePasswordError::AccountHasNoPassword) => Ok(HttpResponse::Forbidden().finish()),
    }
}
