//! Exercise 06 (interface) on http://localhost:4006/graphql, started by `cargo make servers`.

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use fso2f::{ex06_interface::schema::Query, server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);
    server::serve(4006, "06 interface", schema).await;
}
