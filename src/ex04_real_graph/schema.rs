use async_graphql::{ComplexObject, ID, MergedObject, Object, Result, SimpleObject};


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
#[graphql(complex)]
pub struct Product {
    id: ID,
    code: String,
    description: String,
}

#[ComplexObject]
impl Product {
    async fn sales_count(&self) -> Result<i32> {
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
