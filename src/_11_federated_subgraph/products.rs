use async_graphql::{
    ComplexObject, Context, ID, InputObject, Interface, MergedObject, Object, OneofObject, Result,
    SimpleObject,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

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
    async fn product(&self, code: String) -> ProductKind {
        if code.starts_with("D") {
            ProductKind::DangerousProduct(DangerousProduct {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                code,
                description: "A dangerous product".to_string(),
                max_temperature: 100.0,
            })
        } else if code.starts_with("E") {
            ProductKind::ExpiringProduct(ExpiringProduct {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                code,
                description: "An expiring product".to_string(),
                expiration_date: Utc::now() + chrono::Duration::days(30),
            })
        } else {
            ProductKind::Product(Product {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                code,
                description: "A regular product".to_string(),
            })
        }
    }

    #[graphql(entity)]
    async fn find_product_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<Product> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_product = get_product_by_id(pool, id.as_str())
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
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_product = get_expiring_product_by_id(pool, id.as_str())
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
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_product = get_dangerous_product_by_id(pool, id.as_str())
            .await?
            .ok_or_else(|| async_graphql::Error::new("Dangerous product not found"))?;
        Ok(db_product.into())
    }

    #[graphql(entity)]
    async fn find_product_kind_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<ProductKind> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_product = retrieve_product_by_id(pool, id.as_str())
            .await?
            .ok_or_else(|| async_graphql::Error::new("Product not found"))?;
        Ok(db_product.into())
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
            id: ID::from(db.id.to_string()),
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
