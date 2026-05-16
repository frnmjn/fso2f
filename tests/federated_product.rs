use async_graphql::{Request, Variables};
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($productId: ID!) {
        productById (id: $productId) {
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
    let request = Request::new(QUERY).variables(Variables::from_json(json!({
        "productId": "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e",
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

    assert_eq!(
        payload["data"]["productById"],
        json!({
            "id": "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e",
            "kind": "expiring",
            "code": "CHEESE-002",
            "description": "Cheese",
            "expirationDate": "2027-06-15T00:00:00+00:00",
        }),
    );
}
