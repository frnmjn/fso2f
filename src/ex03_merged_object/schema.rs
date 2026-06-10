use async_graphql::{ID, MergedObject, Object, SimpleObject};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery, OrderQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
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
