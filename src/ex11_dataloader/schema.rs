use async_graphql::{
    ComplexObject, Context, Enum, ID, InputObject, Interface, MergedObject, Object, OneofObject,
    Result, SimpleObject, dataloader::DataLoader,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

use crate::db::orders::{DbOrder, DbOrderLine, get_order_by_id, get_order_lines_by_order_id};
use crate::db::products::{
    DbDangerousProduct, DbExpiringProduct, DbProduct, DbProductKind, insert_product,
    retrieve_product_by_code, retrieve_products_by_ids,
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

#[derive(SimpleObject, Clone)]
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

#[derive(SimpleObject, Clone)]
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

#[derive(SimpleObject, Clone)]
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

#[derive(Interface, Clone)]
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
    #[graphql(name = "Cancelled")]
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
        let db_product = product.into_write_model(id);
        let persisted = insert_product(pool, db_product).await?;
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

pub struct ProductLoader {
    pub pool: Pool<Postgres>,
}

impl async_graphql::dataloader::Loader<String> for ProductLoader {
    type Value = ProductKind;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let db_products = retrieve_products_by_ids(&self.pool, keys).await?;
        Ok(db_products
            .into_iter()
            .map(|p| (p.id().to_string(), p.into()))
            .collect())
    }
}
