use std::time::Duration;

use async_graphql::{
    ComplexObject, InputObject, Interface, MergedObject, Object, OneofObject, SimpleObject,
    Subscription,
};
use sqlx::types::chrono::{DateTime, TimeZone, Utc};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProductQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn product(&self, code: String) -> Product {
        tracing::info!("Fetching product with code: {}", code);
        if code.starts_with("EXP") {
            return Product::ExpiringProduct(ExpiringProduct {
                expiration_date: Utc.with_ymd_and_hms(1988, 6, 8, 19, 30, 00).unwrap(),
            });
        }
        Product::DangerousProduct(DangerousProduct {
            max_temperature: 75.0,
        })
    }
}

#[derive(Interface, OneofObject)]
#[graphql(
    field(name = "id", ty = "Uuid"),
    field(name = "code", ty = "String"),
    field(name = "description", ty = "String"),
    input_name = "ProductInput"
)]
pub enum Product {
    DangerousProduct(DangerousProduct),
    ExpiringProduct(ExpiringProduct),
}

#[derive(SimpleObject, InputObject)]
#[graphql(complex, input_name = "DangerousProductInput")]
pub struct DangerousProduct {
    pub max_temperature: f64,
}

#[ComplexObject]
impl DangerousProduct {
    async fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    async fn code(&self) -> String {
        format!("DANG-{}", rand::random::<u32>())
    }

    async fn description(&self) -> String {
        "A dangerous product".to_string()
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(complex, input_name = "ExpiringProductInput")]
pub struct ExpiringProduct {
    pub expiration_date: DateTime<Utc>,
}

#[ComplexObject]
impl ExpiringProduct {
    async fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    async fn code(&self) -> String {
        format!("EXP-{}", rand::random::<u32>())
    }

    async fn description(&self) -> String {
        "An expiring product".to_string()
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[derive(Default)]
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn new_products(
        &self,
        #[graphql(default = 2)] interval: u64,
    ) -> impl Stream<Item = Vec<Product>> {
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(
            interval,
        )))
        .map(move |_| {
            let new_product_added = rand::random::<u8>() % 3 + 1;
            let mut products = Vec::new();
            for i in 0..new_product_added {
                if i % 2 == 0 {
                    products.push(Product::ExpiringProduct(ExpiringProduct {
                        expiration_date: Utc::now() + chrono::Duration::days(i.into()),
                    }));
                } else {
                    products.push(Product::DangerousProduct(DangerousProduct {
                        max_temperature: 60.0 + (rand::random::<u32>() as f64),
                    }));
                }
            }
            products
        })
    }
}
