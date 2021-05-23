use crate::context::Context;
use crate::domain::{Account, ChangePassword, Password, PasswordId};
use crate::telemetry::TraceErrorExt;

#[async_trait::async_trait]
impl ChangePassword for Context {
    #[tracing::instrument(skip(self, old_password, new_password))]
    async fn change_password(
        &self,
        account: &Account,
        old_password: &Password,
        new_password: &Password,
    ) -> Result<(), ()> {
        let password = sqlx::query!(
            r#"
SELECT
    AP.public_id AS public_id,
    AP.password_hash AS password_hash
FROM account AS A
INNER JOIN account_password AS AP ON A.id = AP.account_id
WHERE A.public_id = $1
  AND AP.deleted IS NULL;
"#,
            account.account_id().value()
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("TODO")
        .unwrap();

        let matches =
            argon2::verify_encoded(&password.password_hash, old_password.value().as_bytes())
                .trace_err()
                .expect("TODO");

        if !matches {
            todo!()
        }

        let created = time::OffsetDateTime::now_utc();
        let password_id = PasswordId::new(&uuid::Uuid::new_v4().to_string());

        let password_hash = argon2::hash_encoded(
            new_password.value().as_bytes(),
            uuid::Uuid::new_v4().as_bytes(),
            &argon2::Config::default(),
        )
        .trace_err()
        .expect("TODO");

        let mut tx = self.postgres.begin().await.expect("TODO");

        sqlx::query!(
            r#"
UPDATE account_password
    SET deleted = $1
    WHERE account_password.public_id = $2;
"#,
            created,
            password.public_id
        )
        .execute(&mut tx)
        .await
        .trace_err()
        .expect("TODO");

        sqlx::query!(
            r#"
INSERT INTO account_password(public_id, created, account_id, password_hash)
VALUES ($1,
        $2,
        (SELECT id FROM account WHERE account.public_id = $3),
        $4)
"#,
            password_id.value(),
            created,
            account.account_id().value(),
            password_hash
        )
        .execute(&mut tx)
        .await
        .trace_err()
        .expect("TODO");

        tx.commit().await.expect("TODO");

        Ok(())
    }
}
