use async_graphql::{ID, Object, SimpleObject};
use uuid::Uuid;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn product(&self, code: String) -> Product {
        Product {
            id: ID::from(Uuid::new_v4().to_string()),
            code,
            description: "A sample product".to_string(),
        }
    }
}

#[derive(SimpleObject)]
struct Product {
    id: ID,
    code: String,
    description: String,
}
