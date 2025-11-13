use async_graphql::{ID, Object, SimpleObject};

use crate::db::fixtures;
use crate::db::products::DbProduct;

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, code: String) -> Option<Product> {
        fixtures::get_product_by_code(&code).map(Product::from)
    }
}

#[derive(SimpleObject)]
pub struct Product {
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
