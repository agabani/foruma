use crate::context::Context;
use crate::domain::{IpAddress, Login, LoginError, Password, SessionId, UserAgent, Username};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl Login for Context {
    #[tracing::instrument(
        skip(self, username, password, ip_address, user_agent),
        fields(
            context.username = username.value(),
            context.ip_address = tracing::field::Empty,
            context.user_agent = tracing::field::Empty,
        )
    )]
    async fn login(
        &self,
        username: &Username,
        password: &Password,
        ip_address: &Option<IpAddress>,
        user_agent: &Option<UserAgent>,
    ) -> Result<SessionId, LoginError> {
        if let Some(ip_address) = ip_address {
            tracing::Span::current().record(
                "context.ip_address",
                &ip_address.value().to_string().as_str(),
            );
        }
        if let Some(user_agent) = user_agent {
            tracing::Span::current().record("context.user_agent", &user_agent.value());
        }

        let record = sqlx::query!(
            r#"
SELECT A.id              AS account_id,
       AAP.password_hash AS "account_authentication_password_hash?"
FROM account AS A
         LEFT JOIN account_authentication_password AS AAP ON A.id = AAP.account_id
WHERE A.username = $1;
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

        let password_hash = match &record.account_authentication_password_hash {
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

        let now = time::OffsetDateTime::now_utc();
        let session_id = SessionId::new(&uuid::Uuid::new_v4().to_string());

        let ip_address = ip_address.as_ref().map(IpAddress::value);
        let user_agent = user_agent.as_ref().map(UserAgent::value);

        sqlx::query!(
            r#"
INSERT INTO account_authentication_session (public_id, created, account_id, last_active, ip_address, user_agent)
VALUES ($1, $2, $3, $4, $5, $6);
"#,
            session_id.value(),
            now,
            record.account_id,
            now,
            ip_address,
            user_agent
        )
        .execute(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error");

        Ok(session_id)
    }
}
