use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Variables};
use fso2f::ex03_merged_object::schema::Query;
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($code: String!, $orderId: ID!) {
        product(code: $code) {
            id
            code
            description
        }
        order(id: $orderId) {
            id
            totalAmount
        }
    }
"#;

#[tokio::test]
async fn ex_3() {
    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);

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

    let order = &data["order"];
    assert_eq!(order["id"], "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa");
    assert_eq!(order["totalAmount"], 99.99);
}
