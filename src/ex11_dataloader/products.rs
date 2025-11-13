use async_graphql::{
    Context, ID, InputObject, Interface, Object, OneofObject, Result, SimpleObject,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

use crate::db::products::{
    DbDangerousProduct, DbExpiringProduct, DbProduct, DbProductKind, insert_product,
    retrieve_product_by_code, retrieve_products_by_ids,
};

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
#[allow(clippy::duplicated_attributes)]
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
