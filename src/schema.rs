use async_graphql::*;
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
    #[graphql(skip)]
    id: Uuid,
    code: String,
    description: String,
}

#[ComplexObject]
impl Product {
    async fn sales_count(&self) -> i32 {
        42
    }
}
