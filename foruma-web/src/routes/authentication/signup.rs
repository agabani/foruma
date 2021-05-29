use crate::{
    context::Context,
    domain::{
        CreateAccount, CreateAccountError, CreatePassword, Login, LoginError, Password, Username,
    },
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

    let session_id = match context.login(&username, &password).await {
        Ok(session_id) => session_id,
        Err(LoginError::AccountDoesNotExist)
        | Err(LoginError::AccountHasNoPassword)
        | Err(LoginError::IncorrectPassword) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let mut response = HttpResponse::Ok();
    response.extensions_mut().insert(session_id);
    Ok(response.finish())
}
