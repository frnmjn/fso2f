//! Exercise 02 (complex object) on http://localhost:4002/graphql, started by `cargo make servers`.

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use fso2f::{ex02_complex_object::schema::Query, server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    server::serve(4002, "02 complex object", schema).await;
}
