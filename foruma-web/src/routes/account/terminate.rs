use crate::context::Context;
use crate::domain::{GetAccount, SessionId, TerminateAccount, TerminateAccountError};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn post(
    http_request: HttpRequest,
    context: web::Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
    let extensions = http_request.extensions();
    let session_id = match extensions.get::<SessionId>() {
        Some(session_id) => session_id,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let account = match context.get_account(&session_id).await {
        Some(account) => account,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    match context.terminate_account(&account).await {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(TerminateAccountError::AccountDoesNotExist) => Ok(HttpResponse::BadRequest().finish()),
    }
}
