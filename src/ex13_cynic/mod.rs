pub mod create_product;
pub mod order;

#[cynic::schema("fso2f")]
mod schema {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
