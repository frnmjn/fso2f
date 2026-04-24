use async_graphql::{
    ComplexObject, Context, Enum, ID, InputObject, MergedObject, Object, SimpleObject, scalar,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::orders::{
    DbOrder, DbOrderLine, get_order_by_customer_id, get_order_by_id, get_order_lines_by_order_id,
    insert_order, insert_order_line,
};

#[derive(MergedObject, Default)]
pub struct Query(OrderQuery);

#[derive(Default)]
pub struct OrderQuery;

#[derive(Serialize, Deserialize)]
pub struct OrderId(pub Uuid);

scalar!(OrderId);

#[Object]
impl OrderQuery {
    async fn order(&self, ctx: &Context<'_>, id: OrderId) -> async_graphql::Result<Order> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let db_order = get_order_by_id(pool, id.0)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Order not found"))?;

        Ok(db_order.into())
    }

    #[graphql(entity)]
    async fn find_customer_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<Customer> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let db_order = get_order_by_customer_id(pool, Uuid::parse_str(id.as_str())?)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Order not found"))?;

        Ok(Customer {
            id,
            name: db_order.customer_name,
            vat: db_order.customer_vat,
        })
    }

    #[graphql(entity)]
    async fn find_product_by_id(&self, #[graphql(key)] id: ID) -> Product {
        Product { id }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Order {
    pub id: OrderId,
    pub customer: Customer,
    pub total_amount: f64,
    pub status: OrderStatus,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "CustomerInput")]
pub struct Customer {
    pub id: ID,
    #[graphql(shareable)]
    pub name: String,
    #[graphql(shareable)]
    pub vat: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum OrderStatus {
    Draft,
    Confirmed,
    #[graphql(name = "Cancelled")]
    Deleted,
}

impl From<DbOrder> for Order {
    fn from(db_order: DbOrder) -> Self {
        Self {
            id: OrderId(db_order.id),
            customer: Customer {
                id: ID::from(db_order.customer_id.to_string()),
                name: db_order.customer_name,
                vat: db_order.customer_vat,
            },
            total_amount: db_order.total_amount,
            status: match db_order.status.as_str() {
                "Draft" => OrderStatus::Draft,
                "Confirmed" => OrderStatus::Confirmed,
                "Cancelled" | "Deleted" => OrderStatus::Deleted,
                _ => OrderStatus::Draft,
            },
        }
    }
}

#[ComplexObject]
impl Order {
    async fn lines(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<OrderLine>> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let db_lines = get_order_lines_by_order_id(pool, self.id.0).await?;
        Ok(db_lines.into_iter().map(OrderLine::from).collect())
    }
}

#[derive(SimpleObject)]
pub struct OrderLine {
    pub line_number: i32,
    pub quantity: i32,
    pub product: Product,
}

impl From<DbOrderLine> for OrderLine {
    fn from(db_line: DbOrderLine) -> Self {
        Self {
            line_number: db_line.line_number,
            quantity: db_line.quantity,
            product: Product {
                id: ID::from(db_line.product_id.to_string()),
            },
        }
    }
}

#[derive(SimpleObject)]
#[graphql(interface_object)]
pub struct Product {
    pub id: ID,
}

#[derive(MergedObject, Default)]
pub struct Mutation(OrderMutation);

#[derive(Default)]
pub struct OrderMutation;

#[Object]
impl OrderMutation {
    async fn create_order(
        &self,
        ctx: &Context<'_>,
        order: CreateOrder,
    ) -> async_graphql::Result<Order> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let order_id = Uuid::new_v4();
        let customer_id = Uuid::parse_str(order.customer.id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid customer ID"))?;

        let db_order = insert_order(
            pool,
            order_id,
            customer_id,
            &order.customer.name,
            &order.customer.vat,
            0.0,
            "Draft",
        )
        .await?;

        for (i, item) in order.items.iter().enumerate() {
            let product_id = Uuid::parse_str(item.product_id.as_str())
                .map_err(|_| async_graphql::Error::new("Invalid product ID"))?;
            insert_order_line(
                pool,
                Uuid::new_v4(),
                order_id,
                product_id,
                ((i + 1) * 10) as i32,
                item.quantity,
            )
            .await?;
        }

        Ok(db_order.into())
    }
}

#[derive(InputObject)]
pub struct CreateOrder {
    pub customer: Customer,
    pub items: Vec<CreateOrderLine>,
}

#[derive(InputObject)]
pub struct CreateOrderLine {
    pub product_id: ID,
    pub quantity: i32,
}

#[derive(Default)]
pub struct Subscription;
