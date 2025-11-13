use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema, Variables};
use fso2f::ex05_union::schema::Query;
use serde_json::{Value, json};

const QUERY: &str = r#"
    query ($code: String!) {
        product(code: $code) {
            ... on Product {
                id
                code
                description
            }
            ... on DangerousProduct {
                id
                code
                description
                maxTemperature
            }
            ... on ExpiringProduct {
                id
                code
                description
                expirationDate
            }
        }
    }
"#;

#[tokio::test]
async fn ex_5() {
    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);

    let request = Request::new(QUERY).variables(Variables::from_json(json!({
        "code": "PISTOL-003",
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
    assert_eq!(product["code"], "PISTOL-003");
    assert_eq!(product["description"], "Pistol");
    assert_eq!(product["maxTemperature"], 75.0);
}
