use crate::context::Context;
use crate::domain::{
    CreateAccount, CreateAccountError, CreatePassword, LogIn, LogInError, Password, Username,
};
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

    let account = match context.create_account(&username).await {
        Ok(account) => account,
        Err(CreateAccountError::AccountAlreadyExists) => {
            tracing::warn!("TODO: gracefully handle account creation");
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    context.create_password(&account, &password).await;

    let session_id = match context.log_in(&username, &password).await {
        Ok(session_id) => session_id,
        Err(LogInError::AccountDoesNotExist) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
        Err(LogInError::IncorrectPassword) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
        Err(LogInError::AccountHasNoPassword) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let mut response = HttpResponse::Ok();
    response.extensions_mut().insert(session_id);
    Ok(response.finish())
}
