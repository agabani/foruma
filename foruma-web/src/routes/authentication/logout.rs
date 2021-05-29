use crate::{
    context::Context,
    domain::{Logout, LogoutError},
    http_request_ext::HttpRequestExt,
};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn post(
    http_request: HttpRequest,
    context: web::Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
    let session_id = match http_request.session_id() {
        Some(session_id) => session_id,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    match context.logout(&session_id).await {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(LogoutError::SessionDoesNotExist) => Ok(HttpResponse::NotFound().finish()),
    }
}
