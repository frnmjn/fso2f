use async_graphql::{EmptyMutation, EmptySubscription, SDLExportOptions, Schema};
use fso2f::{
    ex01_simple_object, ex02_complex_object, ex03_merged_object, ex04_real_graph, ex05_modeling,
    ex06_union, ex07_interface, ex08_mutation, ex09_one_of,
    ex10_federation::{orders, products},
    ex11_federated_subgraph::{
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
            "schemas/04_real_graph.graphql",
            Schema::new(
                ex04_real_graph::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/05_modeling.graphql",
            Schema::new(
                ex05_modeling::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/06_union.graphql",
            Schema::new(
                ex06_union::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/07_interface.graphql",
            Schema::new(
                ex07_interface::schema::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/08_mutation.graphql",
            Schema::new(
                ex08_mutation::schema::Query::default(),
                ex08_mutation::schema::Mutation::default(),
                EmptySubscription,
            )
            .sdl(),
        ),
        (
            "schemas/09_one_of.graphql",
            Schema::new(
                ex09_one_of::schema::Query::default(),
                ex09_one_of::schema::Mutation::default(),
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
        (
            "schemas/11_products.graphql",
            Schema::new(
                fs_products::Query::default(),
                fs_products::Mutation::default(),
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/11_orders.graphql",
            Schema::new(
                fs_orders::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/11_customers.graphql",
            Schema::new(
                fs_customers::Query::default(),
                EmptyMutation,
                EmptySubscription,
            )
            .sdl_with_options(SDLExportOptions::new().federation()),
        ),
        (
            "schemas/11_fake_customers.graphql",
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
