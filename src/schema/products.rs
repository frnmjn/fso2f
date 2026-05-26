use std::time::Duration;

use async_graphql::{
    Context, ID, InputObject, Interface, MergedObject, Object, OneofObject, SimpleObject,
    Subscription,
};
use chrono::{DateTime, Utc};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

use crate::db::products::{
    DbDangerousProduct, DbExpiringProduct, DbProduct, DbProductKind, get_dangerous_product_by_id,
    get_expiring_product_by_id, get_product_by_id, insert_product, retrieve_product_by_id,
};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product_by_id(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> async_graphql::Result<Option<ProductKind>> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let uuid = Uuid::parse_str(id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid product ID"))?;
        if let Some(db_product) = retrieve_product_by_id(pool, uuid).await? {
            Ok(Some(db_product.into()))
        } else {
            Ok(None)
        }
    }

    #[graphql(entity)]
    async fn find_product_kind_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<ProductKind> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let uuid = Uuid::parse_str(id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid product ID"))?;
        let db_product = retrieve_product_by_id(pool, uuid)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Product not found"))?;
        Ok(db_product.into())
    }

    #[graphql(entity)]
    async fn find_product_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<Product> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let uuid = Uuid::parse_str(id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid product ID"))?;
        let db_product = get_product_by_id(pool, uuid)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Standard product not found"))?;
        Ok(db_product.into())
    }

    #[graphql(entity)]
    async fn find_expiring_product_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<ExpiringProduct> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let uuid = Uuid::parse_str(id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid product ID"))?;
        let db_product = get_expiring_product_by_id(pool, uuid)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Expiring product not found"))?;
        Ok(db_product.into())
    }

    #[graphql(entity)]
    async fn find_dangerous_product_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<DangerousProduct> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let uuid = Uuid::parse_str(id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid product ID"))?;
        let db_product = get_dangerous_product_by_id(pool, uuid)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Dangerous product not found"))?;
        Ok(db_product.into())
    }
}

#[allow(clippy::duplicated_attributes)]
#[derive(Interface)]
#[graphql(
    field(name = "id", ty = "&ID"),
    field(name = "kind", ty = "String"),
    field(name = "code", ty = "String"),
    field(name = "description", ty = "String")
)]
pub enum ProductKind {
    Product(Product),
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
}

#[derive(SimpleObject)]
pub struct Product {
    #[graphql(shareable)]
    pub id: ID,
    #[graphql(shareable)]
    pub kind: String,
    #[graphql(shareable)]
    pub code: String,
    #[graphql(shareable)]
    pub description: String,
}

impl From<DbProduct> for Product {
    fn from(db: DbProduct) -> Self {
        Self {
            id: ID::from(db.id.to_string()),
            kind: "standard".to_string(),
            code: db.code,
            description: db.description,
        }
    }
}

#[derive(SimpleObject)]
pub struct DangerousProduct {
    #[graphql(shareable)]
    pub id: ID,
    #[graphql(shareable)]
    pub kind: String,
    #[graphql(shareable)]
    pub code: String,
    #[graphql(shareable)]
    pub description: String,
    pub max_temperature: f64,
}

impl From<DbDangerousProduct> for DangerousProduct {
    fn from(db: DbDangerousProduct) -> Self {
        Self {
            id: ID::from(db.id.to_string()),
            kind: "dangerous".to_string(),
            code: db.code,
            description: db.description,
            max_temperature: db.max_temperature,
        }
    }
}

#[derive(SimpleObject)]
pub struct ExpiringProduct {
    #[graphql(shareable)]
    pub id: ID,
    #[graphql(shareable)]
    pub kind: String,
    #[graphql(shareable)]
    pub code: String,
    #[graphql(shareable)]
    pub description: String,
    pub expiration_date: DateTime<Utc>,
}

impl From<DbExpiringProduct> for ExpiringProduct {
    fn from(db: DbExpiringProduct) -> Self {
        Self {
            id: ID::from(db.id.to_string()),
            kind: "expiring".to_string(),
            code: db.code,
            description: db.description,
            expiration_date: db.expiration_date,
        }
    }
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
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;
        let id = Uuid::new_v4();
        let persisted = insert_product(pool, product.into_write_model(id)).await?;
        Ok(persisted.into())
    }
}

#[allow(clippy::duplicated_attributes)]
#[derive(OneofObject)]
pub enum CreateProductKind {
    Product(CreateProduct),
    DangerousProduct(CreateDangerousProduct),
    ExpiringProduct(CreateExpiringProduct),
}

impl CreateProductKind {
    pub fn into_write_model(self, id: Uuid) -> DbProductKind {
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
    pub fn into_write_model(self, id: Uuid) -> DbProduct {
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
    pub fn into_write_model(self, id: Uuid) -> DbDangerousProduct {
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
    pub fn into_write_model(self, id: Uuid) -> DbExpiringProduct {
        DbExpiringProduct {
            id,
            code: self.code,
            description: self.description,
            expiration_date: self.expiration_date,
        }
    }
}

#[derive(Default)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn new_products(
        &self,
        #[graphql(default = 2)] interval: u64,
    ) -> impl Stream<Item = Vec<ProductKind>> {
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(
            interval,
        )))
        .map(move |_| {
            let new_product_added = rand::random::<u8>() % 3 + 1;
            let mut products = Vec::new();
            for i in 0..new_product_added {
                if i % 2 == 0 {
                    products.push(ProductKind::ExpiringProduct(ExpiringProduct {
                        id: ID::from(Uuid::new_v4().to_string()),
                        kind: "expiring".to_string(),
                        code: format!("EXP-{}", Uuid::new_v4()),
                        description: "A newly added expiring product".to_string(),
                        expiration_date: Utc::now() + chrono::Duration::days(30),
                    }));
                } else {
                    products.push(ProductKind::DangerousProduct(DangerousProduct {
                        id: ID::from(Uuid::new_v4().to_string()),
                        kind: "dangerous".to_string(),
                        code: format!("DANG-{}", Uuid::new_v4()),
                        description: "A newly added dangerous product".to_string(),
                        max_temperature: 100.0,
                    }));
                }
            }
            products
        })
    }
}
