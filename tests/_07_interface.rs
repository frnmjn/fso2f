use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Variables};
use fso2f::_07_interface::schema::QueryRoot;
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($code: String!) {
        product(code: $code) {
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
"#;

#[tokio::test]
async fn ex_7() {
    let schema = Schema::new(QueryRoot::default(), EmptyMutation, EmptySubscription);

    let request = Request::new(QUERY).variables(Variables::from_json(json!({
        "code": "DANGER-001",
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
    assert_eq!(product["code"], "DANGER-001");
    assert_eq!(product["description"], "A dangerous product");
    assert_eq!(product["maxTemperature"], 100.0);
}
