use crate::http_request_ext::HttpRequestExt;
use crate::{
    context::Context,
    domain::{
        CreateAccount, CreateAccountError, CreatePassword, Login, LoginError, Password, UserAgent,
        Username,
    },
};
use actix_web::{web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

pub async fn post(
    context: web::Data<Context>,
    http: HttpRequest,
    request: web::Json<Request>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = Username::new(&request.username);
    let password = Password::new(&request.password);
    let ip_address = http.client_ip();

    let user_agent = http
        .headers()
        .get("User-Agent")
        .and_then(|value| value.to_str().ok())
        .map(|value| UserAgent::new(value));

    let account = match context.create_account(&username).await {
        Ok(account) => account,
        Err(CreateAccountError::AccountAlreadyExists) => {
            tracing::warn!("TODO: gracefully handle account creation");
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    context.create_password(&account, &password).await;

    let session_id = match context
        .login(&username, &password, &ip_address, &user_agent)
        .await
    {
        Ok(session_id) => session_id,
        Err(
            LoginError::AccountDoesNotExist
            | LoginError::AccountHasNoPassword
            | LoginError::IncorrectPassword,
        ) => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let mut response = HttpResponse::Ok();
    response.extensions_mut().insert(session_id);
    Ok(response.finish())
}
