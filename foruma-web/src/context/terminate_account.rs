use crate::context::Context;
use crate::domain::{Account, TerminateAccount, TerminateAccountError};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl TerminateAccount for Context {
    #[tracing::instrument(
        skip(self, account),
        fields(
            context.account_id = account.get_account_id().value(),
            context.username = account.get_username().value()
        )
    )]
    async fn terminate_account(&self, account: &Account) -> Result<(), TerminateAccountError> {
        sqlx::query!(
            r#"
DELETE
FROM account
WHERE account.public_id = $1
RETURNING id;
            "#,
            account.get_account_id().value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error")
        .ok_or_else(|| {
            tracing::warn!("Session does not exist");
            TerminateAccountError::AccountDoesNotExist
        })?;

        Ok(())
    }
}
