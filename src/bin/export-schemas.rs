use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use fso2f::{orders, products};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate products subgraph schema
    let products_schema = Schema::new(
        products::QueryRoot::default(),
        EmptyMutation,
        products::SubscriptionRoot::default(),
    );

    let products_sdl = products_schema.sdl();
    fs::write("schemas/products.graphql", products_sdl)?;
    println!("✅ Products schema exported to schemas/products.graphql");

    // Generate orders subgraph schema
    let orders_schema = Schema::new(
        orders::QueryRoot::default(),
        orders::MutationRoot::default(),
        EmptySubscription,
    );

    let orders_sdl = orders_schema.sdl();
    fs::write("schemas/orders.graphql", orders_sdl)?;
    println!("✅ Orders schema exported to schemas/orders.graphql");

    Ok(())
}
