use crate::context::Context;
use crate::domain::{LogIn, Password, SessionId, Username};

#[async_trait::async_trait]
impl LogIn for Context {
    async fn log_in(&self, username: &Username, password: &Password) -> Option<SessionId> {
        let account = sqlx::query!(
            r#"
SELECT
    A.id AS id,
    AP.password_hash AS password_hash
FROM account AS A
INNER JOIN account_password AS AP ON A.id = AP.account_id
WHERE A.username = $1
  AND AP.deleted IS NULL
"#,
            username.value()
        )
        .fetch_optional(&self.postgres)
        .await
        .expect("TODO")?;

        let matches = argon2::verify_encoded(&account.password_hash, password.value().as_bytes())
            .expect("TODO");

        if !matches {
            todo!()
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
        .expect("TODO");

        Some(session_id)
    }
}
