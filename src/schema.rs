use async_graphql::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProductQuery, OrderQuery);

#[derive(Default)]
struct ProductQuery;

#[Object]
impl ProductQuery {
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

#[derive(Default)]
struct OrderQuery;

#[derive(Serialize, Deserialize)]
struct OrderId(Uuid);

scalar!(OrderId);

#[Object]
impl OrderQuery {
    async fn order(&self, id: OrderId) -> Order {
        Order {
            id,
            total_amount: 99.99,
        }
    }
}

#[derive(SimpleObject)]
struct Order {
    id: OrderId,
    total_amount: f64,
}
