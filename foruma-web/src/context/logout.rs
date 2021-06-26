use crate::context::Context;
use crate::domain::{Logout, LogoutError, SessionId};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl Logout for Context {
    #[tracing::instrument(
        skip(self, session_id),
        fields(
            context.session_id = session_id.value()
        )
    )]
    async fn logout(&self, session_id: &SessionId) -> Result<(), LogoutError> {
        sqlx::query!(
            r#"
DELETE FROM account_authentication_session
WHERE public_id = $1
RETURNING id;
"#,
            session_id.value()
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
