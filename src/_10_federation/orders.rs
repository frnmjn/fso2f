use async_graphql::{ComplexObject, Context, Enum, ID, MergedObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::orders::{DbOrder, DbOrderLine, get_order_by_id, get_order_lines_by_order_id};

#[derive(MergedObject, Default)]
pub struct Query(OrderQuery);

#[derive(Default)]
struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn order(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Option<Order>> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let uuid = Uuid::parse_str(id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid order ID"))?;
        let db_order = get_order_by_id(pool, uuid).await?;
        Ok(db_order.map(Order::from))
    }

    #[graphql(entity)]
    async fn find_product_kind_by_id(&self, #[graphql(key)] id: ID) -> ProductKind {
        ProductKind { id }
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

impl From<DbOrder> for Order {
    fn from(o: DbOrder) -> Self {
        Self {
            id: ID::from(o.id.to_string()),
            customer: Customer {
                id: ID::from(o.customer_id.to_string()),
                name: o.customer_name,
                vat: o.customer_vat,
            },
            total_amount: Money {
                amount: o.total_amount,
                currency: Currency::EUR,
            },
            status: match o.status.as_str() {
                "Confirmed" => OrderStatus::Confirmed,
                "Deleted" => OrderStatus::Deleted,
                _ => OrderStatus::Draft,
            },
        }
    }
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
    async fn lines(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<OrderLine>> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let uuid = Uuid::parse_str(self.id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid order ID"))?;
        let db_lines = get_order_lines_by_order_id(pool, uuid).await?;
        Ok(db_lines.into_iter().map(OrderLine::from).collect())
    }
}

#[derive(SimpleObject)]
pub struct OrderLine {
    pub product: ProductKind,
    pub quantity: i32,
    pub price: f64,
    pub discount: Option<f64>,
}

impl From<DbOrderLine> for OrderLine {
    fn from(l: DbOrderLine) -> Self {
        Self {
            product: ProductKind {
                id: ID::from(l.product_id.to_string()),
            },
            quantity: l.quantity,
            price: l.price,
            discount: l.discount,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(interface_object)]
pub struct ProductKind {
    pub id: ID,
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
