use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Variables};
use fso2f::ex05_modeling::schema::Query;
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($orderId: ID!) {
        order(id: $orderId) {
            id
            status
            customer {
                id
                name
                vat
            }
            totalAmount
            lines {
                product {
                    id
                    code
                    description
                    salesCount
                }
                quantity
                price
                discount
            }
        }
    }
"#;

#[tokio::test]
async fn ex_5() {
    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);

    let request = Request::new(QUERY).variables(Variables::from_json(json!({
        "orderId": "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa",
    })));

    let response = schema.execute(request).await;

    assert!(
        response.errors.is_empty(),
        "GraphQL errors: {:?}",
        response.errors
    );

    let data: Value = response.data.into_json().unwrap();

    let lines = &data["order"]["lines"];
    assert!(lines.is_array());
    assert_eq!(lines.as_array().unwrap().len(), 2);

    let product = &lines[0]["product"];
    assert!(product["id"].is_string());
    assert_eq!(product["code"], "P001");
    assert_eq!(product["description"], "Sample product 1");
    assert_eq!(product["salesCount"], 42);
    assert_eq!(lines[0]["quantity"], 2);
    assert_eq!(lines[0]["price"]["amount"], 67.19);
    assert_eq!(lines[0]["price"]["currency"], "EUR");
    assert_eq!(lines[0]["discount"]["amount"], 10.0);
    assert_eq!(lines[0]["discount"]["currency"], "EUR");

    let product = &lines[1]["product"];
    assert!(product["id"].is_string());
    assert_eq!(product["code"], "P002");
    assert_eq!(product["description"], "Sample product 2");
    assert_eq!(product["salesCount"], 42);
    assert_eq!(lines[1]["quantity"], 1);
    assert_eq!(lines[1]["price"]["amount"], 49.99);
    assert_eq!(lines[1]["price"]["currency"], "EUR");
    assert!(lines[1]["discount"].is_null());

    let order = &data["order"];
    assert_eq!(order["id"], "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa");
    assert_eq!(order["status"], "DRAFT");
    assert!(order["customer"]["id"].is_string());
    assert_eq!(order["customer"]["name"], "John Doe");
    assert_eq!(order["customer"]["vat"], "123456789");
    assert_eq!(order["totalAmount"]["amount"], 99.99);
    assert_eq!(order["totalAmount"]["currency"], "EUR");
}
