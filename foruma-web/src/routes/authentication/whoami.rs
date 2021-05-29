use crate::context::Context;
use crate::domain::{GetAccount, SessionId};
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
    let extensions = http_request.extensions();
    let session_id = match extensions.get::<SessionId>() {
        Some(session_id) => session_id,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let account = match context.get_account(session_id).await {
        Some(account) => account,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    return Ok(HttpResponse::Ok().json(Response {
        id: account.account_id().value().to_string(),
        username: account.username().value().to_string(),
    }));
}
