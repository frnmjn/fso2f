//! Exercise 08 (oneOf) on http://localhost:4008/graphql, started by `cargo make servers`.

use async_graphql::{EmptySubscription, Schema};
use fso2f::{
    ex08_one_of::schema::{Mutation, Query},
    server,
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(server::pool().await?)
        .finish();
    server::serve(4008, "08 oneOf", schema).await;
    Ok(())
}
