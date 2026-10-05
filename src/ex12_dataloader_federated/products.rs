use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use async_graphql::{
    Context, ID, InputObject, Interface, MergedObject, Object, OneofObject, Result, SimpleObject,
    dataloader::DataLoader,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::db::products::{
    DbDangerousProduct, DbExpiringProduct, DbProduct, DbProductKind, insert_product,
    retrieve_product_by_code, retrieve_products_by_ids,
};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, ctx: &Context<'_>, code: String) -> Result<Option<ProductKind>> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let product = retrieve_product_by_code(pool, &code).await?;
        Ok(product.map(ProductKind::from))
    }

    #[graphql(entity)]
    async fn find_product_kind_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<ProductKind> {
        load_product(ctx, id).await.map(ProductKind::from)
    }

    #[graphql(entity)]
    async fn find_product_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<Product> {
        match load_product(ctx, id).await? {
            DbProductKind::Product(p) => Ok(p.into()),
            _ => Err(async_graphql::Error::new("Standard product not found")),
        }
    }

    #[graphql(entity)]
    async fn find_expiring_product_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<ExpiringProduct> {
        match load_product(ctx, id).await? {
            DbProductKind::Expiring(p) => Ok(p.into()),
            _ => Err(async_graphql::Error::new("Expiring product not found")),
        }
    }

    #[graphql(entity)]
    async fn find_dangerous_product_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<DangerousProduct> {
        match load_product(ctx, id).await? {
            DbProductKind::Dangerous(p) => Ok(p.into()),
            _ => Err(async_graphql::Error::new("Dangerous product not found")),
        }
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
            id: ID::from(db.id.to_string()),
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
            id: ID::from(db.id.to_string()),
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
            id: ID::from(db.id.to_string()),
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

// ============================================================================
// DataLoader (this is the only part that ex09 doesn't have)
// ============================================================================

pub struct ProductLoader {
    pub pool: Pool<Postgres>,
    pub load_calls: Arc<AtomicUsize>,
}

impl async_graphql::dataloader::Loader<String> for ProductLoader {
    type Value = DbProductKind;
    type Error = Arc<sqlx::Error>;

    async fn load(
        &self,
        keys: &[String],
    ) -> std::result::Result<HashMap<String, Self::Value>, Self::Error> {
        self.load_calls.fetch_add(1, Ordering::SeqCst);
        let products = retrieve_products_by_ids(&self.pool, keys)
            .await
            .map_err(Arc::new)?;
        Ok(products
            .into_iter()
            .map(|p| (p.id().to_string(), p))
            .collect())
    }
}

// Every entity resolver goes through the loader, so N representations cost 1 query.
async fn load_product(ctx: &Context<'_>, id: ID) -> Result<DbProductKind> {
    let loader = ctx.data::<DataLoader<ProductLoader>>()?;
    loader
        .load_one(id.to_string())
        .await?
        .ok_or_else(|| async_graphql::Error::new("Product not found"))
}
