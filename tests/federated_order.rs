use async_graphql::{Request, Variables};
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($orderId: OrderId!) {
        order (id: $orderId) {
            id
            status
            totalAmount
            lines {
                lineNumber
                quantity
                product {
                    id
                    kind
                }
            }
        }
    }
"#;

#[tokio::test]
async fn queries_federated_order() {
    let request = Request::new(QUERY).variables(Variables::from_json(json!({
        "orderId": "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa",
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
        payload["data"]["order"],
        json!({
            "id": "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa",
            "status": "CONFIRMED",
            "totalAmount": 150.0,
            "lines": [
                {
                    "lineNumber": 1,
                    "quantity": 5,
                    "product": {
                        "id": "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d",
                        "kind": "lol",
                    },
                },
                {
                    "lineNumber": 2,
                    "quantity": 10,
                    "product": {
                        "id": "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f",
                        "kind": "lol",
                    },
                },
            ],
        }),
    );
}
