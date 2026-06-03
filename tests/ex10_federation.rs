use async_graphql::{Request, Variables};
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($orderId: ID!) {
        order (id: $orderId) {
            id
            status
            totalAmount
            lines {
                quantity
                product {
                    id
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
            customer {
                id
                name
                vat
            }
        }
    }
"#;

#[tokio::test]
async fn ex_10() {
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
    let data = &payload["data"]["order"];

    assert!(data != &Value::Null, "order should not be null");

    // Order fields
    assert_eq!(data["id"], "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa");
    assert_eq!(data["status"], "CONFIRMED");
    assert_eq!(data["totalAmount"]["amount"], 150.0);
    assert_eq!(data["totalAmount"]["currency"], "EUR");

    // Customer
    let customer = &data["customer"];
    assert_eq!(customer["id"], "11111111-1111-4111-8111-111111111111");
    assert_eq!(customer["name"], "Acme Srl (From Order)");
    assert_eq!(customer["vat"], "IT01234567890");

    // Lines
    let lines = data["lines"].as_array().unwrap();
    assert_eq!(lines.len(), 2);

    assert_eq!(lines[0]["quantity"], 5);
    assert_eq!(
        lines[0]["product"]["id"],
        "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d"
    );
    assert_eq!(lines[0]["product"]["code"], "WIDGET-001");
    assert_eq!(lines[0]["product"]["description"], "Widget");

    assert_eq!(lines[1]["quantity"], 10);
    assert_eq!(
        lines[1]["product"]["id"],
        "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f"
    );
    assert_eq!(lines[1]["product"]["code"], "PISTOL-003");
    assert_eq!(lines[1]["product"]["description"], "Pistol");
    assert_eq!(lines[1]["product"]["maxTemperature"], 75.0);
}
