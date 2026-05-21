use async_graphql::{Object, SimpleObject};
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
struct Product {
    id: Uuid,
    code: String,
    description: String,
}
