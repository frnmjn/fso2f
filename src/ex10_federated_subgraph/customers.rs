use async_graphql::{Context, ID, MergedObject, Object, SimpleObject};
use sqlx::{Pool, Postgres};

use crate::db::customers::{DbCustomer, get_customer_by_id};

#[derive(MergedObject, Default)]
pub struct Query(CustomerQuery);

#[derive(Default)]
pub struct CustomerQuery;

#[Object]
impl CustomerQuery {
    async fn customer(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Customer> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_customer = get_customer_by_id(pool, id.as_str())
            .await?
            .ok_or_else(|| async_graphql::Error::new("Customer not found"))?;
        Ok(db_customer.into())
    }

    #[graphql(entity)]
    async fn find_customer_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: ID,
    ) -> async_graphql::Result<Customer> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let db_customer = get_customer_by_id(pool, id.as_str())
            .await?
            .ok_or_else(|| async_graphql::Error::new("Customer not found"))?;
        Ok(db_customer.into())
    }
}

#[derive(SimpleObject)]
pub struct Customer {
    pub id: ID,
    #[graphql(override_from = "orders")]
    pub name: String,
    #[graphql(override_from = "orders")]
    pub vat: String,
    /// New fields owned by customers subgraph
    pub email: String,
    pub phone: String,
}

impl From<DbCustomer> for Customer {
    fn from(value: DbCustomer) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            vat: value.vat,
            email: value.email,
            phone: value.phone,
        }
    }
}
