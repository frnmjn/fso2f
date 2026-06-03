use async_graphql::{ID, Object, SimpleObject};


pub struct Query;

#[Object]
impl Query {
    async fn product(&self, code: String) -> Product {
        Product {
            id: ID::from(uuid::Uuid::new_v4().to_string()),
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
