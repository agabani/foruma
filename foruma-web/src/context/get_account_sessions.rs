use crate::context::Context;
use crate::domain::{
    AccountId, AccountSession, GetAccountSessions, GetAccountSessionsError, SessionId, UserAgent,
};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl GetAccountSessions for Context {
    #[tracing::instrument(skip(self))]
    async fn get_account_sessions(
        &self,
        account_id: &AccountId,
    ) -> Result<Vec<AccountSession>, GetAccountSessionsError> {
        let records = sqlx::query!(
            r#"
SELECT "A".id          AS account_id,
       "AS".public_id  AS "account_session_public_id?",
       "AS".user_agent AS "account_session_user_agent?"
FROM account AS "A"
         LEFT JOIN account_session AS "AS" ON "A".id = "AS".account_id
WHERE "A".public_id = $1
  AND "A".deleted IS NULL
  AND "AS".deleted IS NULL;
"#,
            account_id.value()
        )
        .fetch_all(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error");

        if records.is_empty() {
            return Err(GetAccountSessionsError::AccountDoesNotExist);
        }

        let account_sessions = records
            .iter()
            .filter(|record| record.account_session_public_id.is_some())
            .map(|record| {
                let session_id = match &record.account_session_public_id {
                    Some(session_id) => SessionId::new(session_id),
                    None => unreachable!(),
                };

                let user_agent = &record
                    .account_session_user_agent
                    .as_ref()
                    .map(|user_agent| UserAgent::new(user_agent));

                AccountSession::new(&session_id, &user_agent)
            })
            .collect();

        Ok(account_sessions)
    }
}
