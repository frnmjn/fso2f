use cynic::{MutationBuilder, http::ReqwestExt};

use super::{Error, schema};

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateProductMutationVariables {
    pub product: CreateProductKind,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Mutation",
    variables = "CreateProductMutationVariables"
)]
pub struct CreateProductMutation {
    #[arguments(product: $product)]
    pub create_product: ProductKind,
}

#[derive(cynic::InputObject, Debug)]
pub enum CreateProductKind {
    Product(CreateProduct),
    #[cynic(rename = "dangerousProduct")]
    DangerousProduct(CreateDangerousProduct),
    #[cynic(rename = "expiringProduct")]
    ExpiringProduct(CreateExpiringProduct),
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
    pub expiration_date: DateTime,
}

pub async fn create_product(
    client: &reqwest::Client,
    url: &str,
    product: CreateProductKind,
) -> Result<ProductKind, Error> {
    let operation = CreateProductMutation::build(CreateProductMutationVariables { product });
    let response = client.post(url).run_graphql(operation).await?;

    if let Some(errors) = response.errors {
        return Err(format!("{errors:?}").into());
    }
    response
        .data
        .map(|d| d.create_product)
        .ok_or_else(|| "empty response".into())
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
    pub expiration_date: DateTime,
}

#[derive(cynic::InlineFragments, Debug)]
pub enum ProductKind {
    Product(Product),
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
    #[cynic(fallback)]
    Unknown,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct DateTime(pub String);
