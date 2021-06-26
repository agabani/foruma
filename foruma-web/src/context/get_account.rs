use crate::context::Context;
use crate::domain::{Account, AccountId, GetAccount, SessionId, Username};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl GetAccount for Context {
    #[tracing::instrument(skip(self))]
    async fn get_account(&self, session_id: &SessionId) -> Option<Account> {
        let record = sqlx::query!(
            r#"
SELECT A.public_id AS account_public_id,
       A.username  AS account_username
FROM account AS A
         INNER JOIN account_authentication_session AS AAS ON A.id = AAS.account_id
WHERE AAS.public_id = $1;
"#,
            session_id.value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error")?;

        Some(Account::new(
            &AccountId::new(&record.account_public_id),
            &Username::new(&record.account_username),
        ))
    }
}
