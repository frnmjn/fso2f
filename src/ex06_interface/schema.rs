use async_graphql::MergedObject;

use super::orders::OrderQuery;
use super::products::ProductQuery;

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery, OrderQuery);
