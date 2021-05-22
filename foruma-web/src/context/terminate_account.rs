use crate::context::Context;
use crate::domain::{Account, TerminateAccount};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl TerminateAccount for Context {
    #[tracing::instrument(skip(self))]
    async fn terminate_account(&self, account: &Account) {
        sqlx::query!(
            r#"
DELETE
FROM account
WHERE account.public_id = $1
RETURNING id;
            "#,
            account.account_id().value()
        )
        .fetch_one(&self.postgres)
        .await
        .trace_err()
        .expect("TODO");
    }
}
