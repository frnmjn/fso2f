use async_graphql::{ComplexObject, Context, Enum, ID, MergedObject, Object, Result, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::db::{
    customers::{DbCustomer, get_customer_by_id},
    orders::{DbOrder, DbOrderLine, get_order_by_id, get_order_lines_by_order_id},
};

#[derive(MergedObject, Default)]
pub struct Query(OrderQuery);

#[derive(Default)]
struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn order(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Order>> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_order = get_order_by_id(pool, id.as_str()).await?;
        Ok(db_order.map(Order::from))
    }

    #[graphql(entity)]
    async fn find_product_kind_by_id(&self, #[graphql(key)] id: ID) -> ProductKind {
        ProductKind { id }
    }

    #[graphql(entity)]
    async fn find_customer_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<Option<Customer>> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_customer = get_customer_by_id(pool, id.as_str()).await?;
        Ok(db_customer.map(Customer::from))
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
            id: ID::from(o.id),
            customer: Customer {
                id: ID::from(o.customer_id),
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

impl From<DbCustomer> for Customer {
    fn from(c: DbCustomer) -> Self {
        Self {
            id: ID::from(c.id),
            name: c.name,
            vat: c.vat,
        }
    }
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
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_lines = get_order_lines_by_order_id(pool, self.id.as_str()).await?;
        Ok(db_lines.into_iter().map(OrderLine::from).collect())
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

impl From<DbOrderLine> for OrderLine {
    fn from(l: DbOrderLine) -> Self {
        Self {
            id: ID::from(l.id),
            product: ProductKind {
                id: ID::from(l.product_id),
            },
            quantity: l.quantity,
            price: l.price.into(),
            discount: l.discount.map(|d| d.into()),
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
