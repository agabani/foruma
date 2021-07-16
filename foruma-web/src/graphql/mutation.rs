use async_graphql::{Context, InputObject, Object};

use crate::domain::{AccountSession, GetAccount, GetAccountSessions, Logout, SessionId};

#[allow(clippy::module_name_repetitions)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
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
pub struct DeleteAccountAuthenticationSessionInput {
    session_id: String,
}
