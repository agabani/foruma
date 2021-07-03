use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type ForumaSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema() -> ForumaSchema {
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
}
