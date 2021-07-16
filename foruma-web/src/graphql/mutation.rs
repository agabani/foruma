use async_graphql::{Context, InputObject, Object};

use crate::domain::{
    AccountSession, ChangePassword, ChangePasswordError, GetAccount, GetAccountSessions, Logout,
    Password, SessionId,
};

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
