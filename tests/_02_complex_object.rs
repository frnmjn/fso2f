use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Variables};
use fso2f::_02_complex_object::schema::QueryRoot;
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($code: String!) {
        product (code: $code) {
            id
            code
            description
            salesCount
        }
    }
"#;

#[tokio::test]
async fn ex_2() {
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

    let request = Request::new(QUERY).variables(Variables::from_json(json!({
        "code": "WIDGET-001",
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
}
