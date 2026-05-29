use async_graphql::{EmptyMutation, EmptySubscription, SDLExportOptions, Schema};
use fso2f::{
    _01_simple_object, _02_complex_object, _03_merged_object, _04_real_graph, _05_modeling,
    _06_union, _07_interface, _08_mutation, _09_one_of,
    _10_federation::{orders, products},
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas: Vec<(&str, String)> = vec![
        (
            "schemas/01_simple_object.graphql",
            Schema::new(
                _01_simple_object::schema::Query,
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/02_complex_object.graphql",
            Schema::new(
                _02_complex_object::schema::Query,
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/03_merged_object.graphql",
            Schema::new(
                _03_merged_object::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/04_real_graph.graphql",
            Schema::new(
                _04_real_graph::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/05_modeling.graphql",
            Schema::new(
                _05_modeling::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/06_union.graphql",
            Schema::new(
                _06_union::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/07_interface.graphql",
            Schema::new(
                _07_interface::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/08_mutation.graphql",
            Schema::new(
                _08_mutation::schema::Query::default(),
                _08_mutation::schema::Mutation::default(),
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/09_one_of.graphql",
            Schema::new(
                _09_one_of::schema::Query::default(),
                _09_one_of::schema::Mutation::default(),
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/10_products.graphql",
            Schema::new(
                products::Query::default(),
                products::Mutation::default(),
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/10_orders.graphql",
            Schema::new(orders::Query::default(), EmptyMutation, EmptySubscription)
                .sdl_with_options(SDLExportOptions::new().federation()),
        ),
    ];

    for (path, sdl) in &schemas {
        fs::write(path, sdl)?;
        println!("✅ Schema exported to {path}");
    }

    Ok(())
}
