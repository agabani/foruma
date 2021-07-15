use crate::domain::{Account, GetAccount, SessionId, UpdateLastActive, UpdateLastActiveError};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

pub type GraphQlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema() -> GraphQlSchema {
    Schema::build(
        QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn current_account<'a>(&self, ctx: &'a Context<'a>) -> Option<Account> {
        let context = ctx
            .data::<actix_web::web::Data<crate::context::Context>>()
            .expect("Database not in context");

        let session = ctx.data_opt::<SessionId>()?;

        let account = context.get_account(session).await?;

        match context.update_last_active(session).await {
            Ok(_) => (),
            Err(UpdateLastActiveError::SessionDoesNotExist) => return None,
        }

        Some(account)
    }
}

#[Object]
impl Account {
    async fn id(&self) -> &str {
        self.get_account_id().value()
    }

    async fn username(&self) -> &str {
        self.get_username().value()
    }
}
