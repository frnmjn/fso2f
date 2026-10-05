use fso2f::ex13_cynic::{
    CreateDangerousProduct, CreateProductKind, OrderStatus, ProductKind, create_product,
    fetch_order,
};
use uuid::Uuid;

const URL: &str = "http://localhost:5000/graphql";

#[tokio::test]
async fn ex_13_query_order() {
    let client = reqwest::Client::new();

    let order = fetch_order(&client, URL, "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa")
        .await
        .unwrap()
        .expect("order should exist");

    assert_eq!(order.id.inner(), "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa");
    assert_eq!(order.status, OrderStatus::Confirmed);
    assert_eq!(order.total_amount.amount, 150.0);
    assert_eq!(order.total_amount.currency, "EUR");
    assert_eq!(order.customer.name, "Acme Srl (From Order)");

    assert_eq!(order.lines.len(), 2);

    let first = &order.lines[0];
    assert_eq!(first.quantity, 5);
    assert_eq!(first.discount.as_ref().unwrap().amount, 2.0);
    assert!(matches!(&first.product, ProductKind::Product(p) if p.code == "WIDGET-001"));

    let second = &order.lines[1];
    assert_eq!(second.quantity, 10);
    assert!(second.discount.is_none());
    assert!(matches!(
        &second.product,
        ProductKind::DangerousProduct(p) if p.code == "PISTOL-003" && p.max_temperature == 75.0
    ));
}

#[tokio::test]
async fn ex_13_create_product() {
    let client = reqwest::Client::new();
    let code = format!("CYNIC-{}", Uuid::new_v4());

    let created = create_product(
        &client,
        URL,
        CreateProductKind::DangerousProduct(CreateDangerousProduct {
            code: code.clone(),
            description: "Created with cynic".to_string(),
            max_temperature: 120.5,
        }),
    )
    .await
    .unwrap();

    match created {
        ProductKind::DangerousProduct(p) => {
            assert_eq!(p.code, code);
            assert_eq!(p.description, "Created with cynic");
            assert_eq!(p.max_temperature, 120.5);
        }
        _ => panic!("expected a DangerousProduct"),
    }
}
