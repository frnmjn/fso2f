use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

impl async_graphql::ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
    }
}
use std::time::Duration;

use async_graphql::{
    ComplexObject, Context, Enum, InputObject, Interface, MergedObject, Object, OneofObject,
    SimpleObject, Subscription,
    connection::{Connection, EmptyFields},
    scalar,
};

// DataLoader per OrderLine

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, TimeZone, Utc};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

use crate::db::{customers::get_customer_by_id, orders::get_order_by_id};

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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct OrderId(Uuid);

scalar!(OrderId);

#[Object]
impl OrderQuery {
    async fn order(&self, ctx: &Context<'_>, id: OrderId) -> async_graphql::Result<Order> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let db_order = get_order_by_id(pool, id.0)
            .await?
            .ok_or(AppError::NotFound("Order not found".to_string()))?;
        let db_customer = get_customer_by_id(pool, db_order.customer_id)
            .await?
            .ok_or(AppError::NotFound("Customer not found".to_string()))?;
        Ok(Order {
            id: OrderId(db_order.id),
            customer: Customer {
                id: db_customer.id,
                name: db_customer.name,
                vat: db_customer.vat,
            },
            total_amount: db_order.total_amount,
            status: match db_order.status.as_str() {
                "Draft" => OrderStatus::Draft,
                "Confirmed" => OrderStatus::Confirmed,
                "Cancelled" | "Deleted" => OrderStatus::Deleted,
                _ => OrderStatus::Draft,
            },
        })
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
    async fn lines(
        &self,
        _ctx: &Context<'_>,
        _after: Option<String>,
        _first: Option<i32>,
    ) -> async_graphql::Result<Connection<i32, OrderLine, EmptyFields, EmptyFields>> {
        // TODO: Integrare caricamento reale delle order line
        Ok(Connection::new(false, false))
    }
}

#[derive(SimpleObject, Clone)]
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

#[derive(Default)]
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn new_products(
        &self,
        #[graphql(default = 2)] interval: u64,
    ) -> impl Stream<Item = Vec<Product>> {
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(
            interval,
        )))
        .map(move |_| {
            let new_product_added = rand::random::<u8>() % 3 + 1;
            let mut products = Vec::new();
            for i in 0..new_product_added {
                if i % 2 == 0 {
                    products.push(Product::ExpiringProduct(ExpiringProduct {
                        expiration_date: Utc::now() + chrono::Duration::days(i.into()),
                    }));
                } else {
                    products.push(Product::DangerousProduct(DangerousProduct {
                        max_temperature: 60.0 + (rand::random::<u32>() as f64),
                    }));
                }
            }
            products
        })
    }
}
