use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn product(&self, code: String) -> Product {
        Product {
            id: Uuid::new_v4(),
            code,
            description: "A sample product".to_string(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct Product {
    id: Uuid,
    code: String,
    description: String,
}

#[ComplexObject]
impl Product {
    async fn sales_count(&self, ctx: &Context<'_>) -> Result<i32> {
        let _ = ctx.data::<Pool<Postgres>>()?;
        Ok(42)
    }
}
