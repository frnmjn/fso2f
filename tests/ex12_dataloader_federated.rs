use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use async_graphql::{EmptySubscription, Request, Schema, Variables, dataloader::DataLoader};
use fso2f::ex12_dataloader_federated::products::{Mutation, ProductLoader, Query};
use serde_json::{Value, json};
use sqlx::PgPool;

const QUERY: &str = r#"
    query ($representations: [_Any!]!) {
        _entities(representations: $representations) {
            ... on Product { id code }
            ... on DangerousProduct { id code maxTemperature }
            ... on ExpiringProduct { id code }
        }
    }
"#;

#[tokio::test]
async fn ex_12() {
    let pool = PgPool::connect("postgres://fso2f:fso2f@localhost:5432/fso2f")
        .await
        .unwrap();
    let load_calls = Arc::new(AtomicUsize::new(0));
    let loader = ProductLoader {
        pool: pool.clone(),
        load_calls: load_calls.clone(),
    };
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool)
        .data(DataLoader::new(loader, tokio::spawn))
        .finish();

    // Same shape the router sends: one _entities call, many representations (with duplicates).
    let representations: Vec<Value> = [
        "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d",
        "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f",
        "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e",
        "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d",
    ]
    .iter()
    .map(|id| json!({ "__typename": "ProductKind", "id": id }))
    .collect();

    let request = Request::new(QUERY)
        .variables(Variables::from_json(json!({ "representations": representations })));
    let response = schema.execute(request).await;

    assert!(
        response.errors.is_empty(),
        "GraphQL errors: {:?}",
        response.errors
    );

    let data: Value = response.data.into_json().unwrap();
    let entities = data["_entities"].as_array().unwrap();
    assert_eq!(entities.len(), 4);
    assert_eq!(entities[0]["code"], "WIDGET-001");
    assert_eq!(entities[1]["code"], "PISTOL-003");
    assert_eq!(entities[1]["maxTemperature"], 75.0);
    assert_eq!(entities[2]["code"], "CHEESE-002");
    assert_eq!(entities[3]["code"], "WIDGET-001");

    assert_eq!(
        load_calls.load(Ordering::SeqCst),
        1,
        "4 representations must be resolved by a single batched load()"
    );
}
