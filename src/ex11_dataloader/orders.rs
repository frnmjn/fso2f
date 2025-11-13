use async_graphql::{
    ComplexObject, Context, Enum, ID, Object, Result, SimpleObject, dataloader::DataLoader,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::db::orders::{DbOrder, DbOrderLine, get_order_by_id, get_order_lines_by_order_id};

use super::products::{ProductKind, ProductLoader};

#[derive(Default)]
pub struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn order(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Order>> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_order = get_order_by_id(pool, id.as_str()).await?;
        Ok(db_order.map(Order::from))
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Order {
    id: ID,
    customer: Customer,
    total_amount: Money,
    status: OrderStatus,
}

impl From<DbOrder> for Order {
    fn from(db: DbOrder) -> Self {
        Self {
            id: ID::from(db.id),
            customer: Customer {
                id: ID::from(db.customer_id),
                name: db.customer_name,
                vat: db.customer_vat,
            },
            total_amount: Money {
                amount: db.total_amount,
                currency: Currency::EUR,
            },
            status: match db.status.as_str() {
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
    #[graphql(name = "CANCELLED")]
    Deleted,
}

#[ComplexObject]
impl Order {
    async fn lines(&self, ctx: &Context<'_>) -> Result<Vec<OrderLine>> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_lines = get_order_lines_by_order_id(pool, self.id.as_str()).await?;
        Ok(db_lines.into_iter().map(OrderLine::from).collect())
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct OrderLine {
    pub id: ID,
    #[graphql(skip)]
    pub product_id: String,
    pub quantity: i32,
    pub price: Money,
    pub discount: Option<Money>,
}

impl From<DbOrderLine> for OrderLine {
    fn from(l: DbOrderLine) -> Self {
        Self {
            id: ID::from(l.id),
            product_id: l.product_id,
            quantity: l.quantity,
            price: l.price.into(),
            discount: l.discount.map(|d| d.into()),
        }
    }
}

#[ComplexObject]
impl OrderLine {
    async fn product(&self, ctx: &Context<'_>) -> Result<ProductKind> {
        let loader = ctx.data::<DataLoader<ProductLoader>>()?;
        loader
            .load_one(self.product_id.clone())
            .await?
            .ok_or_else(|| "Product not found".into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency,
}

impl From<f64> for Money {
    fn from(amount: f64) -> Self {
        Self {
            amount,
            currency: Currency::EUR,
        }
    }
}

async_graphql::scalar!(Money);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    EUR,
    USD,
    GBP,
}
