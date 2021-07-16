use crate::domain::{Logout, LogoutError, SessionId};
use crate::{context::Context, http_request_ext::HttpRequestExt};
use actix_web::{web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    #[serde(rename = "id")]
    id: String,
}

pub async fn delete(
    http_request: HttpRequest,
    context: web::Data<Context>,
    request: web::Path<Request>,
) -> Result<HttpResponse, actix_web::Error> {
    /* TODO: Check if the session the user tried to delete belongs to the user.
     *       Return 404 if it does not belong to the user.
     */

    match http_request.session_id() {
        Some(session_id) => session_id,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let session_id = SessionId::new(&request.id);

    match context.logout(&session_id).await {
        Ok(u) => u,
        Err(LogoutError::SessionDoesNotExist) => {
            return Ok(HttpResponse::BadRequest().finish());
        }
    };

    Ok(HttpResponse::NoContent().finish())
}
