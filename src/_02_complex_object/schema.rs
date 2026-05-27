use async_graphql::{ComplexObject, ID, Object, SimpleObject};
use uuid::Uuid;

pub struct Query;

#[Object]
impl Query {
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
    async fn sales_count(&self) -> i32 {
        42
    }
}
