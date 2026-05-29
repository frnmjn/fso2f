use async_graphql::{
    ComplexObject, Context, Enum, ID, InputObject, Interface, MergedObject, Object, OneofObject,
    Result, SimpleObject,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::products::{
    DbDangerousProduct, DbExpiringProduct, DbProduct, DbProductKind, insert_product,
};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery, OrderQuery);

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
    async fn order(&self, id: ID) -> Order {
        Order {
            id,
            customer: Customer {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                name: "John Doe".to_string(),
                vat: "123456789".to_string(),
            },
            total_amount: Money {
                amount: 99.99,
                currency: Currency::EUR,
            },
            status: OrderStatus::Draft,
        }
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
    async fn lines(&self) -> Vec<OrderLine> {
        vec![
            OrderLine {
                product: Product {
                    id: ID::from(uuid::Uuid::new_v4().to_string()),
                    code: "P001".to_string(),
                    description: "Sample product 1".to_string(),
                },
                quantity: 2,
                price: Money {
                    amount: 67.19,
                    currency: Currency::EUR,
                },
                discount: Some(Money {
                    amount: 10.0,
                    currency: Currency::EUR,
                }),
            },
            OrderLine {
                product: Product {
                    id: ID::from(uuid::Uuid::new_v4().to_string()),
                    code: "P002".to_string(),
                    description: "Sample product 2".to_string(),
                },
                quantity: 1,
                price: Money {
                    amount: 49.99,
                    currency: Currency::EUR,
                },
                discount: None,
            },
        ]
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
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;

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
