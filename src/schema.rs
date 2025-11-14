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
        tracing::info!("Fetching product with code: {}", code);
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
        tracing::info!("Calculating sales count for product code: {}", self.code);
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
        tracing::info!("Fetching order with id: {:?}", id.0);
        Order {
            id,
            total_amount: 99.99,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct Order {
    id: OrderId,
    total_amount: f64,
}

#[ComplexObject]
impl Order {
    async fn lines(&self) -> Vec<OrderLine> {
        tracing::info!("Fetching order lines for order id: {:?}", self.id.0);
        vec![
            OrderLine {
                line_number: 10,
                quantity: 2,
            },
            OrderLine {
                line_number: 20,
                quantity: 5,
            },
        ]
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct OrderLine {
    line_number: i32,
    quantity: i32,
}

#[ComplexObject]
impl OrderLine {
    async fn product(&self) -> Product {
        tracing::info!(
            "Fetching product for order line number: {}",
            self.line_number
        );
        Product {
            id: Uuid::new_v4(),
            code: format!("PROD-{}", rand::random::<u32>()),
            description: "A sample product".to_string(),
        }
    }
}
