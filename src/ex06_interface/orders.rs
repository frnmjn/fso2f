use async_graphql::{ComplexObject, Enum, ID, Object, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::db::fixtures;
use crate::db::orders::{DbOrder, DbOrderLine};

use super::products::ProductKind;

#[derive(Default)]
pub struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn order(&self, id: ID) -> Option<Order> {
        fixtures::get_order_by_id(id.as_str()).map(Order::from)
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
    async fn lines(&self) -> Vec<OrderLine> {
        fixtures::get_order_lines_by_order_id(self.id.as_str())
            .into_iter()
            .map(OrderLine::from)
            .collect()
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
    async fn product(&self) -> Option<ProductKind> {
        fixtures::retrieve_product_by_id(&self.product_id).map(ProductKind::from)
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
