use crate::domain::{UpdateLastActive, UpdateLastActiveError};
use crate::{context::Context, domain::GetAccount, http_request_ext::HttpRequestExt};
use actix_web::{web, HttpRequest, HttpResponse};

#[derive(serde::Serialize)]
struct Response {
    id: String,
    username: String,
}

pub async fn get(
    http_request: HttpRequest,
    context: web::Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
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

    match context.update_last_active(&session_id).await {
        Ok(_) => (),
        Err(UpdateLastActiveError::SessionDoesNotExist) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    return Ok(HttpResponse::Ok().json(Response {
        id: account.account_id().value().to_string(),
        username: account.username().value().to_string(),
    }));
}
