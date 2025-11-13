use async_graphql::{ID, Object, SimpleObject};

use crate::db::fixtures;
use crate::db::orders::DbOrder;
use crate::db::products::DbProduct;

pub struct Query;

#[Object]
impl Query {
    async fn product(&self, code: String) -> Option<Product> {
        fixtures::get_product_by_code(&code).map(Product::from)
    }
    async fn order(&self, id: ID) -> Option<Order> {
        fixtures::get_order_by_id(id.as_str()).map(Order::from)
    }
}

#[derive(SimpleObject)]
struct Product {
    id: ID,
    code: String,
    description: String,
}

impl From<DbProduct> for Product {
    fn from(db: DbProduct) -> Self {
        Self {
            id: ID::from(db.id),
            code: db.code,
            description: db.description,
        }
    }
}

#[derive(SimpleObject)]
struct Order {
    id: ID,
    total_amount: f64,
}

impl From<DbOrder> for Order {
    fn from(db: DbOrder) -> Self {
        Self {
            id: ID::from(db.id),
            total_amount: db.total_amount,
        }
    }
}
