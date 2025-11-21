use async_graphql::{
    ComplexObject, Enum, InputObject, Interface, MergedObject, Object, OneofObject, SimpleObject,
    scalar,
};
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

#[derive(Interface, OneofObject)]
#[graphql(
    field(name = "id", ty = "Uuid"),
    field(name = "code", ty = "String"),
    field(name = "description", ty = "String"),
    input_name = "ProductInput"
)]
pub enum Product {
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
}

#[derive(SimpleObject, InputObject)]
#[graphql(complex, input_name = "DangerousProductInput")]
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

#[derive(SimpleObject, InputObject)]
#[graphql(complex, input_name = "ExpiringProductInput")]
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
            customer: Customer {
                id: Uuid::new_v4(),
                name: "John Doe".to_string(),
                vat: "VAT123456".to_string(),
            },
            total_amount: 99.99,
            status: OrderStatus::Confirmed,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct Order {
    id: OrderId,
    customer: Customer,
    total_amount: f64,
    status: OrderStatus,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "CustomerInput")]
pub struct Customer {
    #[graphql(skip)]
    id: Uuid,
    name: String,
    vat: String,
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

#[derive(MergedObject, Default)]
pub struct MutationRoot(OrderMutation);

#[derive(Default)]
struct OrderMutation;

#[Object]
impl OrderMutation {
    async fn create_order(&self, order: CreateOrder) -> Order {
        tracing::info!("Creating order for customer vat: {}", order.customer.vat);
        Order {
            id: OrderId(Uuid::new_v4()),
            customer: Customer {
                id: Uuid::new_v4(),
                name: "New Customer".to_string(),
                vat: "VAT123456".to_string(),
            },
            total_amount: 0.0,
            status: OrderStatus::Draft,
        }
    }
}

#[derive(InputObject)]
pub struct CreateOrder {
    pub customer: Customer,
    pub items: Vec<CreateOrderLine>,
}

#[derive(InputObject)]
pub struct CreateOrderLine {
    pub product: Product,
    pub quantity: i32,
}
