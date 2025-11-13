//! Exercise 12 (dataloader federated, products subgraph) on http://localhost:4012/graphql, started by `cargo make servers`.

use async_graphql::{EmptySubscription, Schema, dataloader::DataLoader};
use fso2f::{
    ex12_dataloader_federated::products::{Mutation, ProductLoader, Query},
    server,
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    let pool = server::pool().await?;
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(DataLoader::new(
            ProductLoader {
                pool: pool.clone(),
                load_calls: Default::default(),
            },
            tokio::spawn,
        ))
        .data(pool)
        .finish();
    server::serve(4012, "12 dataloader federated (products subgraph)", schema).await;
    Ok(())
}
