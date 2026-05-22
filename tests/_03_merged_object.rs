use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Variables};
use fso2f::_03_merged_object::schema::QueryRoot;
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($code: String!, $orderId: ID!) {
        product(code: $code) {
            id
            code
            description
            salesCount
        }
        order(id: $orderId) {
            id
            totalAmount
        }
    }
"#;

#[tokio::test]
async fn queries_merged_object() {
    let schema = Schema::new(QueryRoot::default(), EmptyMutation, EmptySubscription);

    let request = Request::new(QUERY).variables(Variables::from_json(json!({
        "code": "WIDGET-001",
        "orderId": "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa",
    })));

    let response = schema.execute(request).await;

    assert!(
        response.errors.is_empty(),
        "GraphQL errors: {:?}",
        response.errors
    );

    let data: Value = response.data.into_json().unwrap();

    let product = &data["product"];
    assert!(product["id"].is_string());
    assert_eq!(product["code"], "WIDGET-001");
    assert_eq!(product["description"], "A sample product");
    assert_eq!(product["salesCount"], 42);

    let order = &data["order"];
    assert_eq!(order["id"], "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa");
    assert_eq!(order["totalAmount"], 99.99);
}
