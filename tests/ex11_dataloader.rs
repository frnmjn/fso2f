use async_graphql::{
    EmptyMutation, EmptySubscription, Request, Schema, Variables, dataloader::DataLoader,
};
use fso2f::ex11_dataloader::{ProductLoader, schema::Query};
use serde_json::{Value, json};
use sqlx::PgPool;

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
                }
                quantity
                price
                discount
            }
        }
    }
"#;

#[tokio::test]
async fn ex_11() {
    let pool = PgPool::connect("postgres://fso2f:fso2f@localhost:5432/fso2f")
        .await
        .unwrap();
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .data(DataLoader::new(ProductLoader { pool }, tokio::spawn))
        .finish();

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
    assert_eq!(product["code"], "WIDGET-001");
    assert_eq!(product["description"], "Widget");
    assert_eq!(lines[0]["quantity"], 5);
    assert_eq!(lines[0]["discount"]["amount"], 2.0);

    let product = &lines[1]["product"];
    assert!(product["id"].is_string());
    assert_eq!(product["code"], "PISTOL-003");
    assert_eq!(product["description"], "Pistol");
    assert_eq!(product["maxTemperature"], 75.0);
    assert_eq!(lines[1]["quantity"], 10);
    assert!(lines[1]["discount"].is_null());

    let order = &data["order"];
    assert_eq!(order["id"], "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa");
    assert_eq!(order["status"], "CONFIRMED");
    assert_eq!(order["customer"]["name"], "Acme Srl (From Order)");
}
