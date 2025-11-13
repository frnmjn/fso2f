pub mod create_product;
pub mod order;

pub use create_product::create_product;
pub use order::fetch_order;

#[cynic::schema("fso2f")]
mod schema {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
