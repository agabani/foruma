use crate::context::Context;
use crate::domain::{Account, CreatePassword, Password, PasswordId};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl CreatePassword for Context {
    #[tracing::instrument(
        skip(self, account, password),
        fields(
            context.account_id = account.get_account_id().value(),
            context.username = account.get_username().value()
        )
    )]
    async fn create_password(&self, account: &Account, password: &Password) {
        let created = time::OffsetDateTime::now_utc();
        let password_id = PasswordId::new(&uuid::Uuid::new_v4().to_string());

        let password_hash = argon2::hash_encoded(
            password.value().as_bytes(),
            uuid::Uuid::new_v4().as_bytes(),
            &argon2::Config::default(),
        )
        .trace_err()
        .expect("TODO: handle password hashing error");

        sqlx::query!(
            r#"
INSERT INTO account_authentication_password(public_id, created, account_id, password_hash)
VALUES ($1,
        $2,
        (SELECT id FROM account WHERE account.public_id = $3),
        $4);
"#,
            password_id.value(),
            created,
            account.get_account_id().value(),
            password_hash
        )
        .execute(&self.postgres)
        .await
        .trace_err()
        .expect("TODO: handle database error");
    }
}
