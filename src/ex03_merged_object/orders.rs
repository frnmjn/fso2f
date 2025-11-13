use async_graphql::{ID, Object, SimpleObject};

use crate::db::fixtures;
use crate::db::orders::DbOrder;

#[derive(Default)]
pub struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn order(&self, id: ID) -> Option<Order> {
        fixtures::get_order_by_id(id.as_str()).map(Order::from)
    }
}

#[derive(SimpleObject)]
pub struct Order {
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
