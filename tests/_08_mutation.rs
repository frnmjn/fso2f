use async_graphql::{EmptySubscription, Request, Schema, Variables};
use fso2f::_08_mutation::schema::{Mutation, Query};
use fso2f::db::products::get_product_by_id;
use serde_json::{Value, json};

const MUTATION: &str = r#"
    mutation ($product: CreateProduct!) {
        createProduct (product: $product) {
            id
            code
            description
        }
    }
"#;

#[tokio::test]
async fn ex_8() {
    let pool = sqlx::PgPool::connect("postgres://fso2f:fso2f@localhost:5432/fso2f")
        .await
        .unwrap();
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool.clone())
        .finish();

    let request = Request::new(MUTATION).variables(Variables::from_json(json!({
        "product": {
                "code": "BROCCOLI-101",
                "description": "Broccoli",
        }
    })));

    let response = schema.execute(request).await;

    assert!(
        response.errors.is_empty(),
        "GraphQL errors: {:?}",
        response.errors
    );
    let data: Value = response.data.into_json().unwrap();
    let product = &data["createProduct"];
    assert!(product["id"].is_string());
    assert_eq!(product["code"], "BROCCOLI-101");
    assert_eq!(product["description"], "Broccoli");

    // Verify the record exists in the database
    let id = product["id"].as_str().unwrap();
    let db_product = get_product_by_id(&pool, id).await.unwrap();
    assert!(db_product.is_some());
    let db_product = db_product.unwrap();
    assert_eq!(db_product.code, "BROCCOLI-101");
    assert_eq!(db_product.description, "Broccoli");
}
