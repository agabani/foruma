use crate::context::Context;
use crate::domain::{Account, ChangePassword, ChangePasswordError, Password};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl ChangePassword for Context {
    #[tracing::instrument(skip(self, current_password, new_password))]
    async fn change_password(
        &self,
        account: &Account,
        current_password: &Password,
        new_password: &Password,
    ) -> Result<(), ChangePasswordError> {
        let record = sqlx::query!(
            r#"
SELECT A.public_id      AS account_public_id,
       AP.public_id     AS "account_password_public_id?",
       AP.password_hash AS "account_password_hash?"
FROM account AS A
         LEFT JOIN account_authentication_password AS AP ON A.id = AP.account_id
WHERE A.public_id = $1;
"#,
            account.account_id().value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error")
        .ok_or_else(|| {
            tracing::warn!("Account does not exist");
            ChangePasswordError::AccountDoesNotExist
        })?;

        let password_hash = match &record.account_password_hash {
            Some(password_hash) => password_hash,
            None => {
                tracing::warn!("Account has no password");
                return Err(ChangePasswordError::AccountHasNoPassword);
            }
        };

        let matches = argon2::verify_encoded(&password_hash, current_password.value().as_bytes())
            .trace_err()
            .expect("TODO: handle password hashing error");

        if !matches {
            tracing::warn!("Account provided incorrect password");
            return Err(ChangePasswordError::IncorrectPassword);
        }

        let password_hash = argon2::hash_encoded(
            new_password.value().as_bytes(),
            uuid::Uuid::new_v4().as_bytes(),
            &argon2::Config::default(),
        )
        .trace_err()
        .expect("TODO: handle password hashing error");

        sqlx::query!(
            r#"
UPDATE account_authentication_password
SET password_hash = $1
WHERE public_id = $2;
"#,
            password_hash,
            record.account_password_public_id
        )
        .execute(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error");

        Ok(())
    }
}
