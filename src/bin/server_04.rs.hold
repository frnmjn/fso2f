//! Exercise 04 (modeling) on http://localhost:4004/graphql, started by `cargo make servers`.

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use fso2f::{ex04_modeling::schema::Query, server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);
    server::serve(4004, "04 modeling", schema).await;
}
