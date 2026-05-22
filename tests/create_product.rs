use async_graphql::{Request, Variables};
use serde_json::{Value, json};

const MUTATION: &str = r#"
    mutation ($product: CreateProductKind!) {
        createProduct (product: $product) {
            id
            kind
            code
            description
            ... on DangerousProduct {
                maxTemperature
            }
            ... on ExpiringProduct {
                expirationDate
            }
        }
    }
"#;

#[tokio::test]
async fn queries_federated_product() {
    let request = Request::new(MUTATION).variables(Variables::from_json(json!({
        "product": {
            "expiringProduct": {
                "kind": "expiring",
                "code": "BROCCOLI-101",
                "description": "Broccoli",
                "expirationDate": "2026-07-01T00:00:00+00:00",
            },
        }
    })));

    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:5000/graphql")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let payload: Value = response.json().await.unwrap();
    dbg!(&payload);

    let id = &payload["data"]["createProduct"]["id"];

    assert_eq!(
        payload["data"]["createProduct"],
        json!({
            "id": id,
            "kind": "expiring",
            "code": "BROCCOLI-101",
            "description": "Broccoli",
            "expirationDate": "2026-07-01T00:00:00+00:00",
        }),
    );
}
