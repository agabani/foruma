use crate::context::Context;
use crate::domain::{Login, LoginError, Password, Username};
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

pub async fn post(
    context: web::Data<Context>,
    request: web::Json<Request>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = Username::new(&request.username);
    let password = Password::new(&request.password);

    let session_id = match context.login(&username, &password).await {
        Ok(session_id) => session_id,
        Err(LoginError::AccountDoesNotExist) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
        Err(LoginError::IncorrectPassword) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
        Err(LoginError::AccountHasNoPassword) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let mut response = HttpResponse::Ok();
    response.extensions_mut().insert(session_id);
    Ok(response.finish())
}
