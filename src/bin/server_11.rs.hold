//! Exercise 11 (dataloader) on http://localhost:4011/graphql, started by `cargo make servers`.

use async_graphql::{EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader};
use fso2f::{
    ex11_dataloader::schema::{ProductLoader, Query},
    server,
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    let pool = server::pool().await?;
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(DataLoader::new(
            ProductLoader { pool: pool.clone() },
            tokio::spawn,
        ))
        .data(pool)
        .finish();
    server::serve(4011, "11 dataloader", schema).await;
    Ok(())
}
