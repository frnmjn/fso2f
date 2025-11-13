use async_graphql::MergedObject;

use super::orders::OrderQuery;
pub use super::products::ProductLoader;
use super::products::{ProductMutation, ProductQuery};

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery, OrderQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ProductMutation);
