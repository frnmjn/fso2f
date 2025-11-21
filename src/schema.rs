use async_graphql::{ComplexObject, Enum, Interface, MergedObject, Object, SimpleObject, scalar};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProductQuery, OrderQuery);

#[derive(Default)]
struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, code: String) -> Product {
        tracing::info!("Fetching product with code: {}", code);
        if code.starts_with("EXP") {
            return Product::ExpiringProduct(ExpiringProduct {
                expiration_date: Utc.with_ymd_and_hms(1988, 6, 8, 19, 30, 00).unwrap(),
            });
        }
        Product::DangerousProduct(DangerousProduct {
            max_temperature: 75.0,
        })
    }
}

#[derive(Interface)]
#[graphql(
    field(name = "id", ty = "Uuid"),
    field(name = "code", ty = "String"),
    field(name = "description", ty = "String")
)]
pub enum Product {
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct DangerousProduct {
    max_temperature: f64,
}

#[ComplexObject]
impl DangerousProduct {
    async fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    async fn code(&self) -> String {
        format!("DANG-{}", rand::random::<u32>())
    }

    async fn description(&self) -> String {
        "A dangerous product".to_string()
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ExpiringProduct {
    expiration_date: DateTime<Utc>,
}

#[ComplexObject]
impl ExpiringProduct {
    async fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    async fn code(&self) -> String {
        format!("EXP-{}", rand::random::<u32>())
    }

    async fn description(&self) -> String {
        "An expiring product".to_string()
    }
}

#[derive(Default)]
struct OrderQuery;

#[derive(Serialize, Deserialize)]
struct OrderId(Uuid);

scalar!(OrderId);

#[Object]
impl OrderQuery {
    async fn order(&self, id: OrderId) -> Order {
        tracing::info!("Fetching order with id: {:?}", id.0);
        Order {
            id,
            total_amount: 99.99,
            status: OrderStatus::Confirmed,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct Order {
    id: OrderId,
    total_amount: f64,
    status: OrderStatus,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum OrderStatus {
    Draft,
    Confirmed,
    #[graphql(name = "Cancelled")]
    Deleted,
}

#[ComplexObject]
impl Order {
    async fn lines(&self) -> Vec<OrderLine> {
        tracing::info!("Fetching order lines for order id: {:?}", self.id.0);
        vec![
            OrderLine {
                line_number: 10,
                quantity: 2,
                discount: Some(5.0),
            },
            OrderLine {
                line_number: 20,
                quantity: 5,
                discount: None,
            },
        ]
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct OrderLine {
    line_number: i32,
    quantity: i32,
    discount: Option<f64>,
}

#[ComplexObject]
impl OrderLine {
    async fn product(&self) -> Product {
        tracing::info!(
            "Fetching product for order line number: {}",
            self.line_number
        );
        if self.line_number % 2 == 0 {
            return Product::ExpiringProduct(ExpiringProduct {
                expiration_date: Utc.with_ymd_and_hms(1988, 6, 8, 19, 30, 00).unwrap(),
            });
        }
        Product::DangerousProduct(DangerousProduct {
            max_temperature: 100.0,
        })
    }
}
