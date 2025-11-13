//! Exercise 05 (union) on http://localhost:4005/graphql, started by `cargo make servers`.

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use fso2f::{ex05_union::schema::Query, server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);
    server::serve(4005, "05 union", schema).await;
}
