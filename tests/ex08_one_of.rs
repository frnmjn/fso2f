use async_graphql::{EmptySubscription, Request, Schema, Variables};
use fso2f::ex08_one_of::schema::{Mutation, Query};
use fso2f::db::products::retrieve_product_by_id;
use serde_json::{Value, json};
use sqlx::PgPool;

const MUTATION: &str = r#"
    mutation ($product: CreateProductKind!) {
        createProduct (product: $product) {
            ... on Product {
                id
                code
                description
            }
            ... on DangerousProduct {
                id
                code
                description
                maxTemperature
            }
            ... on ExpiringProduct {
                id
                code
                description
                expirationDate
            }
        }
    }
"#;

#[tokio::test]
async fn ex_8() {
    let pool = PgPool::connect("postgres://fso2f:fso2f@localhost:5432/fso2f")
        .await
        .unwrap();
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool.clone())
        .finish();

    let request = Request::new(MUTATION).variables(Variables::from_json(json!({
        "product": {
            "dangerousProduct": {
                "kind": "dangerous",
                "code": "DANGER-101",
                "description": "Dangerous chemical",
                "maxTemperature": 45.5
            }
        }
    })));

    let response = schema.execute(request).await;

    assert!(
        response.errors.is_empty(),
        "GraphQL errors: {:?}",
        response.errors
    );
    let data: Value = response.data.into_json().unwrap();
    let product = &data["createProduct"];
    assert!(product["id"].is_string());
    assert_eq!(product["code"], "DANGER-101");
    assert_eq!(product["description"], "Dangerous chemical");
    assert_eq!(product["maxTemperature"], 45.5);

    // Verify the record exists in the database
    let id = product["id"].as_str().unwrap();
    let db_product = retrieve_product_by_id(&pool, id).await.unwrap();
    assert!(db_product.is_some());
    let db_product = db_product.unwrap();
    match db_product {
        fso2f::db::products::DbProductKind::Dangerous(p) => {
            assert_eq!(p.code, "DANGER-101");
            assert_eq!(p.description, "Dangerous chemical");
            assert_eq!(p.max_temperature, 45.5);
        }
        _ => panic!("Expected a dangerous product in the database"),
    }
}
