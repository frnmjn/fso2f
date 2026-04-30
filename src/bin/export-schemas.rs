use async_graphql::{EmptyMutation, EmptySubscription, SDLExportOptions, Schema};
use fso2f::schema::{customers, fake_customers, orders, products};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate products subgraph schema
    let products_schema = Schema::new(products::Query::default(), EmptyMutation, EmptySubscription);

    let products_sdl = products_schema.sdl_with_options(SDLExportOptions::new().federation());
    fs::write("schemas/products.graphql", products_sdl)?;
    println!("✅ Products schema exported to schemas/products.graphql");

    // Generate orders subgraph schema
    let orders_schema = Schema::new(
        orders::Query::default(),
        orders::Mutation::default(),
        EmptySubscription,
    );

    let orders_sdl = orders_schema.sdl_with_options(SDLExportOptions::new().federation());
    fs::write("schemas/orders.graphql", orders_sdl)?;
    println!("✅ Orders schema exported to schemas/orders.graphql");

    // Generate fake customers subgraph schema
    let fake_customers_schema = Schema::new(
        fake_customers::FakeCustomerQuery,
        EmptyMutation,
        EmptySubscription,
    );
    let fake_customers_sdl =
        fake_customers_schema.sdl_with_options(SDLExportOptions::new().federation());
    fs::write("schemas/fake_customers.graphql", fake_customers_sdl)?;
    println!("✅ Fake Customers schema exported to schemas/fake_customers.graphql");

    // Generate customers subgraph schema
    let customers_schema = Schema::new(
        customers::Query::default(),
        EmptyMutation,
        EmptySubscription,
    );

    let customers_sdl = customers_schema.sdl_with_options(SDLExportOptions::new().federation());
    fs::write("schemas/customers.graphql", customers_sdl)?;
    println!("✅ Customers schema exported to schemas/customers.graphql");

    Ok(())
}
