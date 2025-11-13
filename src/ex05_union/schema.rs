use async_graphql::{ComplexObject, Enum, ID, MergedObject, Object, SimpleObject, Union};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery, OrderQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, code: String) -> ProductKind {
        if code.starts_with("D") {
            ProductKind::DangerousProduct(DangerousProduct {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                code,
                description: "A dangerous product".to_string(),
                max_temperature: 100.0,
            })
        } else if code.starts_with("E") {
            ProductKind::ExpiringProduct(ExpiringProduct {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                code,
                description: "An expiring product".to_string(),
                expiration_date: Utc::now() + chrono::Duration::days(30),
            })
        } else {
            ProductKind::Product(Product {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                code,
                description: "A regular product".to_string(),
            })
        }
    }
}

#[derive(SimpleObject)]
pub struct Product {
    id: ID,
    code: String,
    description: String,
}

#[derive(SimpleObject)]
pub struct DangerousProduct {
    pub id: ID,
    pub code: String,
    pub description: String,
    pub max_temperature: f64,
}

#[derive(SimpleObject)]
pub struct ExpiringProduct {
    pub id: ID,
    pub code: String,
    pub description: String,
    pub expiration_date: DateTime<Utc>,
}

#[derive(Union)]
pub enum ProductKind {
    Product(Product),
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
}

#[derive(Default)]
struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn order(&self, id: ID) -> Order {
        Order {
            id,
            customer: Customer {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                name: "John Doe".to_string(),
                vat: "123456789".to_string(),
            },
            total_amount: Money {
                amount: 99.99,
                currency: Currency::EUR,
            },
            status: OrderStatus::Draft,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct Order {
    id: ID,
    customer: Customer,
    total_amount: Money,
    status: OrderStatus,
}

#[derive(SimpleObject)]
pub struct Customer {
    pub id: ID,
    pub name: String,
    pub vat: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum OrderStatus {
    Draft,
    Confirmed,
    #[graphql(name = "Cancelled")]
    Deleted,
}

#[ComplexObject]
impl Order {
    async fn lines(&self) -> Vec<OrderLine> {
        vec![
            OrderLine {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                product: ProductKind::ExpiringProduct(ExpiringProduct {
                    id: ID::from(uuid::Uuid::new_v4().to_string()),
                    code: "P001".to_string(),
                    description: "Sample product 1".to_string(),
                    expiration_date: Utc::now() + chrono::Duration::days(30),
                }),
                quantity: 2,
                price: Money {
                    amount: 67.19,
                    currency: Currency::EUR,
                },
                discount: Some(Money {
                    amount: 10.0,
                    currency: Currency::EUR,
                }),
            },
            OrderLine {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                product: ProductKind::DangerousProduct(DangerousProduct {
                    id: ID::from(uuid::Uuid::new_v4().to_string()),
                    code: "P002".to_string(),
                    description: "Sample product 2".to_string(),
                    max_temperature: 100.0,
                }),
                quantity: 1,
                price: Money {
                    amount: 49.99,
                    currency: Currency::EUR,
                },
                discount: None,
            },
        ]
    }
}

#[derive(SimpleObject)]
pub struct OrderLine {
    pub id: ID,
    pub product: ProductKind,
    pub quantity: i32,
    pub price: Money,
    pub discount: Option<Money>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency,
}

async_graphql::scalar!(Money);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    EUR,
    USD,
    GBP,
}
