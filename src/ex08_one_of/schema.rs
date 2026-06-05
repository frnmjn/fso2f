use async_graphql::{
    ComplexObject, Context, Enum, ID, InputObject, Interface, MergedObject, Object, OneofObject,
    Result, SimpleObject,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::db::orders::{DbOrder, get_order_by_id, get_order_lines_by_order_id};
use crate::db::products::{
    DbDangerousProduct, DbExpiringProduct, DbProduct, DbProductKind, get_product_by_id,
    insert_product, retrieve_product_by_code,
};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery, OrderQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, ctx: &Context<'_>, code: String) -> Result<Option<ProductKind>> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let product = retrieve_product_by_code(pool, &code).await?;
        Ok(product.map(ProductKind::from))
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Product {
    id: ID,
    code: String,
    description: String,
}

impl From<DbProduct> for Product {
    fn from(db: DbProduct) -> Self {
        Self {
            id: ID::from(db.id),
            code: db.code,
            description: db.description,
        }
    }
}

#[ComplexObject]
impl Product {
    async fn sales_count(&self) -> Result<i32> {
        Ok(42)
    }
}

#[derive(SimpleObject)]
pub struct DangerousProduct {
    pub id: ID,
    pub code: String,
    pub description: String,
    pub max_temperature: f64,
}

impl From<DbDangerousProduct> for DangerousProduct {
    fn from(db: DbDangerousProduct) -> Self {
        Self {
            id: ID::from(db.id),
            code: db.code,
            description: db.description,
            max_temperature: db.max_temperature,
        }
    }
}

#[derive(SimpleObject)]
pub struct ExpiringProduct {
    pub id: ID,
    pub code: String,
    pub description: String,
    pub expiration_date: DateTime<Utc>,
}

impl From<DbExpiringProduct> for ExpiringProduct {
    fn from(db: DbExpiringProduct) -> Self {
        Self {
            id: ID::from(db.id),
            code: db.code,
            description: db.description,
            expiration_date: db.expiration_date,
        }
    }
}

#[derive(Interface)]
#[graphql(
    field(name = "id", ty = "&ID"),
    field(name = "code", ty = "String"),
    field(name = "description", ty = "String")
)]
pub enum ProductKind {
    Product(Product),
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
}

impl From<DbProductKind> for ProductKind {
    fn from(db: DbProductKind) -> Self {
        match db {
            DbProductKind::Product(p) => ProductKind::Product(Product::from(p)),
            DbProductKind::Dangerous(p) => ProductKind::DangerousProduct(DangerousProduct::from(p)),
            DbProductKind::Expiring(p) => ProductKind::ExpiringProduct(ExpiringProduct::from(p)),
        }
    }
}

#[derive(Default)]
struct OrderQuery;

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
    async fn lines(&self, ctx: &Context<'_>) -> Result<Vec<OrderLine>> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_lines = get_order_lines_by_order_id(pool, self.id.as_str()).await?;
        let mut lines = Vec::new();
        for line in db_lines {
            let product = get_product_by_id(pool, &line.product_id)
                .await?
                .map(Product::from)
                .unwrap_or(Product {
                    id: ID::from(line.product_id),
                    code: "UNKNOWN".to_string(),
                    description: "Unknown product".to_string(),
                });
            lines.push(OrderLine {
                product,
                quantity: line.quantity,
                price: Money {
                    amount: line.price,
                    currency: Currency::EUR,
                },
                discount: line.discount.map(|d| Money {
                    amount: d,
                    currency: Currency::EUR,
                }),
            });
        }
        Ok(lines)
    }
}

#[derive(SimpleObject)]
pub struct OrderLine {
    pub product: Product,
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

#[derive(MergedObject, Default)]
pub struct Mutation(ProductMutation);

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductMutation {
    async fn create_product(
        &self,
        ctx: &Context<'_>,
        product: CreateProductKind,
    ) -> async_graphql::Result<ProductKind> {
        let pool = ctx.data::<Pool<Postgres>>()?;

        let id = uuid::Uuid::new_v4().to_string();
        let persisted = insert_product(pool, product.into_write_model(id)).await?;
        Ok(persisted.into())
    }
}

#[derive(OneofObject)]
pub enum CreateProductKind {
    Product(CreateProduct),
    DangerousProduct(CreateDangerousProduct),
    ExpiringProduct(CreateExpiringProduct),
}

impl CreateProductKind {
    pub fn into_write_model(self, id: String) -> DbProductKind {
        match self {
            CreateProductKind::Product(p) => DbProductKind::Product(p.into_write_model(id)),
            CreateProductKind::DangerousProduct(p) => {
                DbProductKind::Dangerous(p.into_write_model(id))
            }
            CreateProductKind::ExpiringProduct(p) => {
                DbProductKind::Expiring(p.into_write_model(id))
            }
        }
    }
}

#[derive(InputObject)]
pub struct CreateProduct {
    pub kind: String,
    pub code: String,
    pub description: String,
}

impl CreateProduct {
    pub fn into_write_model(self, id: String) -> DbProduct {
        DbProduct {
            id,
            code: self.code,
            description: self.description,
        }
    }
}

#[derive(InputObject)]
pub struct CreateDangerousProduct {
    pub kind: String,
    pub code: String,
    pub description: String,
    pub max_temperature: f64,
}

impl CreateDangerousProduct {
    pub fn into_write_model(self, id: String) -> DbDangerousProduct {
        DbDangerousProduct {
            id,
            code: self.code,
            description: self.description,
            max_temperature: self.max_temperature,
        }
    }
}

#[derive(InputObject)]
pub struct CreateExpiringProduct {
    pub kind: String,
    pub code: String,
    pub description: String,
    pub expiration_date: DateTime<Utc>,
}

impl CreateExpiringProduct {
    pub fn into_write_model(self, id: String) -> DbExpiringProduct {
        DbExpiringProduct {
            id,
            code: self.code,
            description: self.description,
            expiration_date: self.expiration_date,
        }
    }
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
