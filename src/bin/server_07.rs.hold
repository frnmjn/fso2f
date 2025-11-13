//! Exercise 07 (mutation) on http://localhost:4007/graphql, started by `cargo make servers`.

use async_graphql::{EmptySubscription, Schema};
use fso2f::{
    ex07_mutation::schema::{Mutation, Query},
    server,
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(server::pool().await?)
        .finish();
    server::serve(4007, "07 mutation", schema).await;
    Ok(())
}
