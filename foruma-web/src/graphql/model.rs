use crate::graphql::query::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema, SchemaBuilder};

pub type GraphQlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema() -> SchemaBuilder<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(
        QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
}
