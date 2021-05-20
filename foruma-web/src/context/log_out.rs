use crate::context::Context;
use crate::domain::{LogOut, SessionId};

#[async_trait::async_trait]
impl LogOut for Context {
    async fn log_out(&self, session_id: &SessionId) {
        let deleted = time::OffsetDateTime::now_utc();

        sqlx::query!(
            r#"
UPDATE account_session
SET deleted = $2
WHERE public_id = $1
  AND deleted IS NULL;
"#,
            session_id.value(),
            deleted
        )
        .execute(&self.postgres)
        .await
        .expect("TODO");
    }
}
