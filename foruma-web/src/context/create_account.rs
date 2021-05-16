use crate::context::Context;
use crate::domain::{Account, AccountId, CreateAccount, Username};

#[async_trait::async_trait]
impl CreateAccount for Context {
    async fn create_account(&self, username: &Username) -> Account {
        let account_id = AccountId::new(&uuid::Uuid::new_v4().to_string());
        let created = time::OffsetDateTime::now_utc();

        sqlx::query!(
            r#"
INSERT INTO account (public_id, created, username)
VALUES ($1, $2, $3);
"#,
            account_id.value(),
            created,
            username.value()
        )
        .execute(&self.postgres)
        .await
        .expect("TODO");

        Account::new(&account_id, username)
    }
}
