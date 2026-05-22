use async_graphql::{ComplexObject, ID, MergedObject, Object, Result, SimpleObject};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProductQuery, OrderQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, code: String) -> Product {
        Product {
            id: ID::from(Uuid::new_v4().to_string()),
            code,
            description: "A sample product".to_string(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct Product {
    id: ID,
    code: String,
    description: String,
}

#[ComplexObject]
impl Product {
    async fn sales_count(&self /*, ctx: &Context<'_> */) -> Result<i32> {
        // let _ = ctx.data::<Pool<Postgres>>()?;
        Ok(42)
    }
}

#[derive(Default)]
struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn order(&self, id: ID) -> Order {
        Order {
            id,
            total_amount: 99.99,
        }
    }
}

#[derive(SimpleObject)]
struct Order {
    id: ID,
    total_amount: f64,
}
