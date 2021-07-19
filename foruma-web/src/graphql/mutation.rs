use async_graphql::{Context, InputObject, Object};

use crate::domain::{
    AccountSession, ChangePassword, ChangePasswordError, CreateAccount, CreateAccountError,
    CreatePassword, GetAccount, GetAccountSessions, IpAddress, Login, LoginError, Logout,
    LogoutError, Password, SessionId, UserAgent, Username,
};
use actix_web::http::header::SET_COOKIE;

#[allow(clippy::module_name_repetitions)]
pub struct MutationRoot;

pub enum GraphQLError {
    Unauthenticated,
    BadRequest,
}

impl std::fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            GraphQLError::Unauthenticated => write!(f, "unauthenticated"),
            GraphQLError::BadRequest => write!(f, "bad_request"),
        }
    }
}

#[Object]
impl MutationRoot {
    async fn change_account_authentication_password<'a>(
        &self,
        ctx: &'a Context<'a>,
        input: ChangeAccountAuthenticationPassword,
    ) -> Result<bool, String> {
        let context = ctx
            .data::<actix_web::web::Data<crate::context::Context>>()
            .expect("Database not in context");

        let session_id = ctx
            .data_opt::<SessionId>()
            .ok_or_else(|| GraphQLError::Unauthenticated.to_string())?;

        let current_password = Password::new(&input.current_password);
        let new_password = Password::new(&input.new_password);

        let account = context
            .get_account(&session_id)
            .await
            .ok_or_else(|| GraphQLError::Unauthenticated.to_string())?;

        match context
            .change_password(&account, &current_password, &new_password)
            .await
        {
            Ok(_) => Ok(true),
            Err(
                ChangePasswordError::AccountDoesNotExist
                | ChangePasswordError::AccountHasNoPassword,
            ) => Err(GraphQLError::BadRequest.to_string()),
            Err(ChangePasswordError::IncorrectPassword) => Err("incorrect_password".to_string()),
        }
    }

    async fn delete_account_authentication_session<'a>(
        &self,
        ctx: &'a Context<'a>,
        input: DeleteAccountAuthenticationSessionInput,
    ) -> Option<AccountSession> {
        let context = ctx
            .data::<actix_web::web::Data<crate::context::Context>>()
            .expect("Database not in context");

        let session_id = ctx.data_opt::<SessionId>()?;

        let account = context.get_account(&session_id).await?;

        let vec = context
            .get_account_sessions(account.get_account_id())
            .await
            .ok()?;

        let account_session = vec
            .into_iter()
            .find(|x| x.get_session_id().value() == input.session_id)?;

        context
            .logout(&SessionId::new(&input.session_id))
            .await
            .ok()?;

        Some(account_session)
    }

    async fn login<'a>(&self, ctx: &'a Context<'a>, input: LoginInput) -> Result<bool, String> {
        let context = ctx
            .data::<actix_web::web::Data<crate::context::Context>>()
            .expect("Database not in context");

        let http_session_cookie = ctx
            .data::<crate::http_session_cookie::HttpSessionCookie>()
            .expect("HttpSessionCookie not in context");

        let ip_address = ctx.data::<IpAddress>().ok().cloned();

        let user_agent = ctx.data::<UserAgent>().ok().cloned();

        let username = Username::new(&input.username);
        let password = Password::new(&input.password);

        let session_id = match context
            .login(&username, &password, &ip_address, &user_agent)
            .await
        {
            Ok(session_id) => session_id,
            Err(
                LoginError::AccountDoesNotExist
                | LoginError::AccountHasNoPassword
                | LoginError::IncorrectPassword,
            ) => return Err(GraphQLError::BadRequest.to_string()),
        };

        let cookie = http_session_cookie.encrypt_session_id(&session_id);
        ctx.append_http_header(SET_COOKIE, cookie.to_str().unwrap());

        Ok(true)
    }

    async fn logout_current_account<'a>(&self, ctx: &'a Context<'a>) -> Result<bool, String> {
        let context = ctx
            .data::<actix_web::web::Data<crate::context::Context>>()
            .expect("Database not in context");

        let session_id = ctx
            .data_opt::<SessionId>()
            .ok_or_else(|| GraphQLError::Unauthenticated.to_string())?;

        match context.logout(&session_id).await {
            Ok(()) => Ok(true),
            Err(LogoutError::SessionDoesNotExist) => Err(GraphQLError::Unauthenticated.to_string()),
        }
    }

    async fn signup<'a>(&self, ctx: &'a Context<'a>, input: SignupInput) -> Result<bool, String> {
        let context = ctx
            .data::<actix_web::web::Data<crate::context::Context>>()
            .expect("Database not in context");

        let http_session_cookie = ctx
            .data::<crate::http_session_cookie::HttpSessionCookie>()
            .expect("HttpSessionCookie not in context");

        let ip_address = ctx.data::<IpAddress>().ok().cloned();

        let user_agent = ctx.data::<UserAgent>().ok().cloned();

        let username = Username::new(&input.username);
        let password = Password::new(&input.password);

        let account = match context.create_account(&username).await {
            Ok(account) => account,
            Err(CreateAccountError::AccountAlreadyExists) => {
                return Err(GraphQLError::BadRequest.to_string())
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
            ) => return Err(GraphQLError::BadRequest.to_string()),
        };

        let cookie = http_session_cookie.encrypt_session_id(&session_id);
        ctx.append_http_header(SET_COOKIE, cookie.to_str().unwrap());

        Ok(true)
    }
}

#[derive(InputObject)]
pub struct ChangeAccountAuthenticationPassword {
    current_password: String,
    new_password: String,
}

#[derive(InputObject)]
pub struct DeleteAccountAuthenticationSessionInput {
    session_id: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    username: String,
    password: String,
}

#[derive(InputObject)]
pub struct SignupInput {
    username: String,
    password: String,
}
