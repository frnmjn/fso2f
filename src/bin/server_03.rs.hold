//! Exercise 03 (merged object) on http://localhost:4003/graphql, started by `cargo make servers`.

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use fso2f::{ex03_merged_object::schema::Query, server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);
    server::serve(4003, "03 merged object", schema).await;
}
