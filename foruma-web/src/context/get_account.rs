use crate::context::Context;
use crate::domain::{Account, AccountId, GetAccount, SessionId, Username};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl GetAccount for Context {
    #[tracing::instrument(skip(self))]
    async fn get_account(&self, session_id: &SessionId) -> Option<Account> {
        let account = sqlx::query!(
            r#"
SELECT
    A.public_id AS public_id,
    A.username AS username
FROM account AS A
INNER JOIN account_session AS "AS" ON A.id = "AS".account_id
WHERE "AS".public_id = $1
  AND "AS".deleted IS NULL;
"#,
            session_id.value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO")?;

        Some(Account::new(
            &AccountId::new(&account.public_id),
            &Username::new(&account.username),
        ))
    }
}
