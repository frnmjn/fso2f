use async_graphql::{
    ComplexObject, Context, Enum, ID, InputObject, Interface, MergedObject, Object, Result,
    SimpleObject,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::products::{DbProduct, insert_standard_product};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery, OrderQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, code: String) -> ProductKind {
        if code.starts_with("D") {
            ProductKind::DangerousProduct(DangerousProduct {
                id: ID::from(Uuid::new_v4().to_string()),
                code,
                description: "A dangerous product".to_string(),
                max_temperature: 100.0,
            })
        } else if code.starts_with("E") {
            ProductKind::ExpiringProduct(ExpiringProduct {
                id: ID::from(Uuid::new_v4().to_string()),
                code,
                description: "An expiring product".to_string(),
                expiration_date: Utc::now() + chrono::Duration::days(30),
            })
        } else {
            ProductKind::Product(Product {
                id: ID::from(Uuid::new_v4().to_string()),
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

#[derive(SimpleObject)]
pub struct ExpiringProduct {
    pub id: ID,
    pub code: String,
    pub description: String,
    pub expiration_date: DateTime<Utc>,
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

#[derive(Default)]
struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn order(&self, id: ID) -> Order {
        Order {
            id,
            customer: Customer {
                id: ID::from(Uuid::new_v4().to_string()),
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
                    id: ID::from(Uuid::new_v4().to_string()),
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
                    id: ID::from(Uuid::new_v4().to_string()),
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
        product: CreateProduct,
    ) -> async_graphql::Result<Product> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;

        let id = Uuid::new_v4();
        let persisted = insert_standard_product(
            pool,
            DbProduct {
                id,
                code: product.code,
                description: product.description,
            },
        )
        .await?;
        Ok(persisted.into())
    }
}

#[derive(InputObject)]
pub struct CreateProduct {
    pub code: String,
    pub description: String,
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
