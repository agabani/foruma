use crate::context::Context;
use crate::domain::{Login, LoginError, Password, SessionId, UserAgent, Username};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl Login for Context {
    #[tracing::instrument(skip(self, password))]
    async fn login(
        &self,
        username: &Username,
        password: &Password,
        user_agent: &Option<UserAgent>,
    ) -> Result<SessionId, LoginError> {
        let record = sqlx::query!(
            r#"
SELECT A.id             AS account_id,
       AP.password_hash AS "account_password_hash?"
FROM account AS A
         LEFT JOIN account_password AS AP ON A.id = AP.account_id
WHERE A.username = $1
  AND AP.deleted IS NULL;
"#,
            username.value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error")
        .ok_or_else(|| {
            tracing::warn!("Account does not exist");
            LoginError::AccountDoesNotExist
        })?;

        let password_hash = match &record.account_password_hash {
            Some(password_hash) => password_hash,
            None => {
                tracing::warn!("Account has no password");
                return Err(LoginError::AccountHasNoPassword);
            }
        };

        let matches = argon2::verify_encoded(&password_hash, password.value().as_bytes())
            .trace_err()
            .expect("TODO: handle password hashing error");

        if !matches {
            tracing::warn!("Account provided incorrect password");
            return Err(LoginError::IncorrectPassword);
        }

        let created = time::OffsetDateTime::now_utc();
        let session_id = SessionId::new(&uuid::Uuid::new_v4().to_string());

        match user_agent {
            None => {
                sqlx::query!(
                    r#"
INSERT INTO account_session (public_id, created, account_id)
VALUES ($1, $2, $3);
"#,
                    session_id.value(),
                    created,
                    record.account_id
                )
                .execute(&self.postgres)
                .await
                .trace_err()
                .expect("TODO: handle database error");
            }
            Some(user_agent) => {
                sqlx::query!(
                    r#"
INSERT INTO account_session (public_id, created, account_id, user_agent)
VALUES ($1, $2, $3, $4);
"#,
                    session_id.value(),
                    created,
                    record.account_id,
                    user_agent.value()
                )
                .execute(&self.postgres)
                .await
                .trace_err()
                .expect("TODO: handle database error");
            }
        }

        Ok(session_id)
    }
}
