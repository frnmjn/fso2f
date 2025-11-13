//! Exercise 01 (simple object) on http://localhost:4001/graphql, started by `cargo make servers`.

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use fso2f::{ex01_simple_object::schema::Query, server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    server::serve(4001, "01 simple object", schema).await;
}
