//! GraphQL schema definition

use crate::resolvers::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

/// Create GraphQL schema
pub fn create_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription).finish()
}
