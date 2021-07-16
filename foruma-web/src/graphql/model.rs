use async_graphql::{EmptySubscription, Schema, SchemaBuilder};

use crate::graphql::mutation::MutationRoot;
use crate::graphql::query::QueryRoot;

pub type GraphQlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn schema() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription)
}
