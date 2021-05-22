use crate::context::Context;
use crate::domain::{Account, AccountId, CreateAccount, Username};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl CreateAccount for Context {
    #[tracing::instrument(skip(self))]
    async fn create_account(&self, username: &Username) -> Option<Account> {
        let account_id = AccountId::new(&uuid::Uuid::new_v4().to_string());
        let created = time::OffsetDateTime::now_utc();

        sqlx::query!(
            r#"
INSERT INTO account (public_id, created, username)
VALUES ($1, $2, $3)
ON CONFLICT DO NOTHING
RETURNING id;
"#,
            account_id.value(),
            created,
            username.value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO")?;

        Some(Account::new(&account_id, username))
    }
}
