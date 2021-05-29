use crate::context::Context;
use crate::domain::{LogOut, LogoutError, SessionId};
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

    match context.log_out(&session_id).await {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(LogoutError::SessionDoesNotExist) => Ok(HttpResponse::NotFound().finish()),
    }
}
