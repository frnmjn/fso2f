use std::time::Duration;

use async_graphql::{Context, ID, Interface, MergedObject, Object, SimpleObject, Subscription};
use chrono::{DateTime, Utc};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

use crate::db::products::{
    DbDangerousProduct, DbExpiringProduct, DbProductKind, get_dangerous_product_by_id,
    get_expiring_product_by_id,
};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
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

#[derive(Interface)]
#[graphql(
    field(name = "id", ty = "&ID"),
    field(name = "code", ty = "String"),
    field(name = "description", ty = "String")
)]
pub enum Product {
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
}

#[derive(SimpleObject)]
pub struct DangerousProduct {
    #[graphql(shareable)]
    pub id: ID,
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
    pub code: String,
    #[graphql(shareable)]
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

impl From<DbProductKind> for Product {
    fn from(db: DbProductKind) -> Self {
        match db {
            DbProductKind::Dangerous(p) => Product::DangerousProduct(DangerousProduct::from(p)),
            DbProductKind::Expiring(p) => Product::ExpiringProduct(ExpiringProduct::from(p)),
        }
    }
}

#[derive(Default)]
pub struct Mutation;

#[derive(Default)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn new_products(
        &self,
        #[graphql(default = 2)] interval: u64,
    ) -> impl Stream<Item = Vec<Product>> {
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(
            interval,
        )))
        .map(move |_| {
            let new_product_added = rand::random::<u8>() % 3 + 1;
            let mut products = Vec::new();
            for i in 0..new_product_added {
                if i % 2 == 0 {
                    products.push(Product::ExpiringProduct(ExpiringProduct {
                        id: ID::from(Uuid::new_v4().to_string()),
                        code: format!("EXP{}", Uuid::new_v4().to_string()),
                        description: "A newly added expiring product".to_string(),
                        expiration_date: Utc::now() + chrono::Duration::days(30),
                    }));
                } else {
                    products.push(Product::DangerousProduct(DangerousProduct {
                        id: ID::from(Uuid::new_v4().to_string()),
                        code: format!("DANG{}", Uuid::new_v4().to_string()),
                        description: "A newly added dangerous product".to_string(),
                        max_temperature: 100.0,
                    }));
                }
            }
            products
        })
    }
}
