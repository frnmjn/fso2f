use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Variables};
use fso2f::_04_real_graph::schema::Query;
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($orderId: ID!) {
        order(id: $orderId) {
            id
            totalAmount
            lines {
                product {
                    id
                    code
                    description
                    salesCount
                }
                quantity
            }
        }
    }
"#;

#[tokio::test]
async fn ex_4() {
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

    let product = &lines[1]["product"];
    assert!(product["id"].is_string());
    assert_eq!(product["code"], "P002");
    assert_eq!(product["description"], "Sample product 2");
    assert_eq!(product["salesCount"], 42);

    let order = &data["order"];
    assert_eq!(order["id"], "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa");
    assert_eq!(order["totalAmount"], 99.99);
}
