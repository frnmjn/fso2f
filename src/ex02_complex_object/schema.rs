use async_graphql::{ComplexObject, ID, Object, SimpleObject};

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
    async fn order(&self, id: ID) -> Order {
        Order {
            id,
            total_amount: 99.99,
        }
    }
}

#[derive(SimpleObject)]
pub struct Product {
    id: ID,
    code: String,
    description: String,
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct Order {
    id: ID,
    total_amount: f64,
}

#[derive(SimpleObject)]
pub struct OrderLine {
    pub product: Product,
    pub quantity: i32,
}

#[ComplexObject]
impl Order {
    async fn lines(&self) -> Vec<OrderLine> {
        vec![
            OrderLine {
                product: Product {
                    id: ID::from(uuid::Uuid::new_v4().to_string()),
                    code: "P001".to_string(),
                    description: "Sample product 1".to_string(),
                },
                quantity: 2,
            },
            OrderLine {
                product: Product {
                    id: ID::from(uuid::Uuid::new_v4().to_string()),
                    code: "P002".to_string(),
                    description: "Sample product 2".to_string(),
                },
                quantity: 1,
            },
        ]
    }
}
