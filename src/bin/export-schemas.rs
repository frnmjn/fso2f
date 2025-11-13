use async_graphql::{EmptyMutation, EmptySubscription, SDLExportOptions, Schema};
use fso2f::{
    ex01_simple_object, ex02_complex_object, ex03_merged_object, ex04_modeling, ex05_union,
    ex06_interface, ex07_mutation, ex08_one_of,
    ex09_federation::{orders, products},
    ex10_feature_subgraph::{
        customers as fs_customers, fake_customers as fs_fake_customers, orders as fs_orders,
        products as fs_products,
    },
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas: Vec<(&str, String)> = vec![
        (
            "schemas/01_simple_object.graphql",
            Schema::new(
                ex01_simple_object::schema::Query,
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/02_complex_object.graphql",
            Schema::new(
                ex02_complex_object::schema::Query,
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/03_merged_object.graphql",
            Schema::new(
                ex03_merged_object::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/04_modeling.graphql",
            Schema::new(
                ex04_modeling::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/05_union.graphql",
            Schema::new(
                ex05_union::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/06_interface.graphql",
            Schema::new(
                ex06_interface::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/07_mutation.graphql",
            Schema::new(
                ex07_mutation::schema::Query::default(),
                ex07_mutation::schema::Mutation::default(),
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/08_one_of.graphql",
            Schema::new(
                ex08_one_of::schema::Query::default(),
                ex08_one_of::schema::Mutation::default(),
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/09_products.graphql",
            Schema::new(
                products::Query::default(),
                products::Mutation::default(),
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/09_orders.graphql",
            Schema::new(orders::Query::default(), EmptyMutation, EmptySubscription)
                .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/10_products.graphql",
            Schema::new(
                fs_products::Query::default(),
                fs_products::Mutation::default(),
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/10_orders.graphql",
            Schema::new(
                fs_orders::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/10_customers.graphql",
            Schema::new(
                fs_customers::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/10_fake_customers.graphql",
            Schema::new(
                fs_fake_customers::Query::default(),
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
