use async_graphql::{EmptyMutation, EmptySubscription, SDLExportOptions, Schema};
use fso2f::ex09_federation;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas: Vec<(&str, String)> = vec![
        (
            "schemas/09_products.graphql",
            Schema::new(
                ex09_federation::products::Query::default(),
                ex09_federation::products::Mutation::default(),
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/09_orders.graphql",
            Schema::new(
                ex09_federation::orders::Query::default(),
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
