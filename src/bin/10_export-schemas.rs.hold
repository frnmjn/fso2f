use async_graphql::{EmptyMutation, EmptySubscription, SDLExportOptions, Schema};
use fso2f::ex10_feature_subgraph;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas: Vec<(&str, String)> = vec![
        (
            "schemas/10_products.graphql",
            Schema::new(
                ex10_feature_subgraph::products::Query::default(),
                ex10_feature_subgraph::products::Mutation::default(),
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/10_orders.graphql",
            Schema::new(
                ex10_feature_subgraph::orders::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/10_customers.graphql",
            Schema::new(
                ex10_feature_subgraph::customers::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/10_fake_customers.graphql",
            Schema::new(
                ex10_feature_subgraph::fake_customers::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
    ];

    for (path, sdl) in &schemas {
        fs::write(path, sdl)?;
        println!("✅ Schema exported to {path}");
    }

    Ok(())
}
