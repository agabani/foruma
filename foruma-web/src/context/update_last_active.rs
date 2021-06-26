use crate::context::Context;
use crate::domain::{SessionId, UpdateLastActive, UpdateLastActiveError};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl UpdateLastActive for Context {
    #[tracing::instrument(skip(self))]
    async fn update_last_active(
        &self,
        session_id: &SessionId,
    ) -> Result<(), UpdateLastActiveError> {
        let now = time::OffsetDateTime::now_utc();

        sqlx::query!(
            r#"
UPDATE account_authentication_session
SET last_active = $1
WHERE public_id = $2
RETURNING id;
            "#,
            now,
            session_id.value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error")
        .ok_or_else(|| {
            tracing::warn!("Session does not exists");
            UpdateLastActiveError::SessionDoesNotExist
        })?;

        Ok(())
    }
}
