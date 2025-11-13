use async_graphql::{ID, Object, SimpleObject, Union};
use chrono::{DateTime, Utc};

use crate::db::fixtures;
use crate::db::products::{DbDangerousProduct, DbExpiringProduct, DbProduct, DbProductKind};

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, code: String) -> Option<ProductKind> {
        fixtures::retrieve_product_by_code(&code).map(ProductKind::from)
    }
}

#[derive(SimpleObject)]
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

#[derive(Union)]
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
