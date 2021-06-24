use crate::domain::UserAgent;
use crate::{
    context::Context,
    domain::{Login, LoginError, Password, Username},
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

    let user_agent = http
        .headers()
        .get("User-Agent")
        .and_then(|value| value.to_str().ok())
        .map(|value| UserAgent::new(value));

    let session_id = match context.login(&username, &password, &user_agent).await {
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
