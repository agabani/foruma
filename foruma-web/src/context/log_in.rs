use crate::context::Context;
use crate::domain::{LogIn, LogInError, Password, SessionId, Username};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl LogIn for Context {
    #[tracing::instrument(skip(self, password))]
    async fn log_in(
        &self,
        username: &Username,
        password: &Password,
    ) -> Result<SessionId, LogInError> {
        let account = sqlx::query!(
            r#"
SELECT
    A.id AS id,
    AP.password_hash AS "password_hash?"
FROM account AS A
LEFT JOIN account_password AS AP ON A.id = AP.account_id
WHERE A.username = $1
  AND AP.deleted IS NULL
"#,
            username.value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error")
        .ok_or({
            tracing::warn!("Account does not exist");
            LogInError::AccountDoesNotExist
        })?;

        let password_hash = match &account.password_hash {
            Some(password_hash) => password_hash,
            None => {
                tracing::warn!("Account has no password");
                return Err(LogInError::AccountHasNoPassword);
            }
        };

        let matches = argon2::verify_encoded(&password_hash, password.value().as_bytes())
            .trace_err()
            .expect("TODO: handle password hashing error");

        if !matches {
            tracing::warn!("Account provided incorrect password");
            return Err(LogInError::IncorrectPassword);
        }

        let created = time::OffsetDateTime::now_utc();
        let session_id = SessionId::new(&uuid::Uuid::new_v4().to_string());

        sqlx::query!(
            r#"
INSERT INTO account_session (public_id, created, account_id)
VALUES ($1, $2, $3);
"#,
            session_id.value(),
            created,
            account.id
        )
        .execute(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error");

        Ok(session_id)
    }
}
