use crate::context::Context;
use crate::domain::{Logout, LogoutError, SessionId};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl Logout for Context {
    #[tracing::instrument(skip(self))]
    async fn logout(&self, session_id: &SessionId) -> Result<(), LogoutError> {
        let deleted = time::OffsetDateTime::now_utc();

        sqlx::query!(
            r#"
UPDATE account_session
SET deleted = $2
WHERE public_id = $1
  AND deleted IS NULL
RETURNING id;
"#,
            session_id.value(),
            deleted
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error")
        .ok_or_else(|| {
            tracing::warn!("Session does not exist");
            LogoutError::SessionDoesNotExist
        })?;

        Ok(())
    }
}
