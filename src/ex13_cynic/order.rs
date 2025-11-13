use cynic::{QueryBuilder, http::ReqwestExt};
use serde::{Deserialize, Serialize};

use super::{Error, schema};

#[derive(cynic::QueryVariables, Debug)]
pub struct OrderQueryVariables<'a> {
    pub order_id: &'a cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "OrderQueryVariables")]
pub struct OrderQuery {
    #[arguments(id: $order_id)]
    pub order: Option<Order>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Order {
    pub id: cynic::Id,
    pub status: OrderStatus,
    pub total_amount: Money,
    pub lines: Vec<OrderLine>,
    pub customer: Customer,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Customer {
    pub id: cynic::Id,
    pub name: String,
    pub vat: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct OrderLine {
    pub quantity: i32,
    pub discount: Option<Money>,
    pub product: ProductKind,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ExpiringProduct {
    pub id: cynic::Id,
    pub code: String,
    pub description: String,
    pub expiration_date: DateTime,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct DangerousProduct {
    pub id: cynic::Id,
    pub code: String,
    pub description: String,
    pub max_temperature: f64,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Product {
    pub id: cynic::Id,
    pub code: String,
    pub description: String,
}

#[derive(cynic::InlineFragments, Debug)]
pub enum ProductKind {
    Product(Product),
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
    #[cynic(fallback)]
    Unknown,
}

#[derive(cynic::Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderStatus {
    Draft,
    Confirmed,
    #[cynic(rename = "Cancelled")]
    Cancelled,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct DateTime(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: f64,
    pub currency: String,
}

cynic::impl_scalar!(Money, schema::Money);

pub async fn fetch_order(
    client: &reqwest::Client,
    url: &str,
    order_id: &str,
) -> Result<Option<Order>, Error> {
    let id = cynic::Id::new(order_id);
    let operation = OrderQuery::build(OrderQueryVariables { order_id: &id });
    let response = client.post(url).run_graphql(operation).await?;

    if let Some(errors) = response.errors {
        return Err(format!("{errors:?}").into());
    }
    Ok(response.data.and_then(|d| d.order))
}
