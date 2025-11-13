//! Exercise 00: checks that the toolchain, the crate and the database work.
//! Run by `cargo make check-setup`.

use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Variables};
use fso2f::db::products::{DbProductKind, retrieve_product_by_code};
use fso2f::ex00_init::schema::Query;
use serde_json::{Value, json};
use sqlx::PgPool;

#[tokio::test]
async fn ex_0_schema() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    let request = Request::new("query ($name: String!) { greeting(name: $name) }")
        .variables(Variables::from_json(json!({ "name": "RustLab" })));

    let response = schema.execute(request).await;

    assert!(
        response.errors.is_empty(),
        "GraphQL errors: {:?}",
        response.errors
    );
    let data: Value = response.data.into_json().unwrap();
    assert_eq!(data["greeting"], "Hello, RustLab!");
}

#[tokio::test]
async fn ex_0_database() {
    let pool = PgPool::connect("postgres://fso2f:fso2f@localhost:5432/fso2f")
        .await
        .expect("cannot connect to postgres: run `docker compose up -d --wait postgres`");

    let product = retrieve_product_by_code(&pool, "WIDGET-001")
        .await
        .unwrap()
        .expect("seed data missing: recreate the database with `docker compose down -v`");

    assert!(matches!(product, DbProductKind::Product(p) if p.description == "Widget"));
}
