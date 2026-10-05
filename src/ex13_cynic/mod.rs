use cynic::{MutationBuilder, QueryBuilder, http::ReqwestExt};

#[cynic::schema("fso2f")]
mod schema {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] cynic::http::CynicReqwestError),
    #[error("GraphQL errors: {0:?}")]
    GraphQl(Vec<cynic::GraphQlError>),
    #[error("response contains no data")]
    NoData,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Money {
    pub amount: f64,
    pub currency: String,
}

cynic::impl_scalar!(Money, schema::Money);
cynic::impl_scalar!(chrono::DateTime<chrono::Utc>, schema::DateTime);

#[derive(cynic::Enum, Debug, PartialEq)]
pub enum OrderStatus {
    Draft,
    Confirmed,
    #[cynic(rename = "Cancelled")]
    Cancelled,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Customer {
    pub id: cynic::Id,
    pub name: String,
    pub vat: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Product {
    pub id: cynic::Id,
    pub code: String,
    pub description: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct DangerousProduct {
    pub id: cynic::Id,
    pub code: String,
    pub description: String,
    pub max_temperature: f64,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ExpiringProduct {
    pub id: cynic::Id,
    pub code: String,
    pub description: String,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}

#[derive(cynic::InlineFragments, Debug)]
pub enum ProductKind {
    Product(Product),
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
    #[cynic(fallback)]
    Unknown,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct OrderLine {
    pub quantity: i32,
    pub product: ProductKind,
    pub discount: Option<Money>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Order {
    pub id: cynic::Id,
    pub status: OrderStatus,
    pub total_amount: Money,
    pub customer: Customer,
    pub lines: Vec<OrderLine>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct OrderQueryVariables {
    pub order_id: cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "OrderQueryVariables")]
pub struct OrderQuery {
    #[arguments(id: $order_id)]
    pub order: Option<Order>,
}

#[derive(cynic::InputObject, Debug)]
pub struct CreateProduct {
    pub code: String,
    pub description: String,
}

#[derive(cynic::InputObject, Debug)]
pub struct CreateDangerousProduct {
    pub code: String,
    pub description: String,
    pub max_temperature: f64,
}

#[derive(cynic::InputObject, Debug)]
pub struct CreateExpiringProduct {
    pub code: String,
    pub description: String,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}

#[derive(cynic::InputObject, Debug)]
pub enum CreateProductKind {
    Product(CreateProduct),
    DangerousProduct(CreateDangerousProduct),
    ExpiringProduct(CreateExpiringProduct),
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateProductVariables {
    pub product: CreateProductKind,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "CreateProductVariables")]
pub struct CreateProductMutation {
    #[arguments(product: $product)]
    pub create_product: ProductKind,
}

fn into_data<T>(response: cynic::GraphQlResponse<T>) -> Result<T, Error> {
    if let Some(errors) = response.errors.filter(|e| !e.is_empty()) {
        return Err(Error::GraphQl(errors));
    }
    response.data.ok_or(Error::NoData)
}

pub async fn fetch_order(
    client: &reqwest::Client,
    url: &str,
    order_id: &str,
) -> Result<Option<Order>, Error> {
    let operation = OrderQuery::build(OrderQueryVariables {
        order_id: cynic::Id::new(order_id),
    });
    let response = client.post(url).run_graphql(operation).await?;
    Ok(into_data(response)?.order)
}

pub async fn create_product(
    client: &reqwest::Client,
    url: &str,
    product: CreateProductKind,
) -> Result<ProductKind, Error> {
    let operation = CreateProductMutation::build(CreateProductVariables { product });
    let response = client.post(url).run_graphql(operation).await?;
    Ok(into_data(response)?.create_product)
}
