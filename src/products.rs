use std::time::Duration;

use async_graphql::{ID, MergedObject, Object, SimpleObject, Subscription};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    #[graphql(entity)]
    async fn find_simple_product_by_id(&self, #[graphql(key)] id: ID) -> SimpleProduct {
        SimpleProduct {
            id,
            code: "EX23".to_string(),
            description: "A simple product".to_string(),
            pippo: "PIPPO".to_string(),
        }
    }

    // #[graphql(entity)]
    // async fn find_expiring_product_by_id(&self, #[graphql(key)] _id: String) -> ExpiringProduct {
    //     ExpiringProduct {
    //         id: ID::from(Uuid::new_v4().to_string()),
    //         code: "EXP123".to_string(),
    //         description: "An expiring product".to_string(),
    //         expiration_date: Utc.with_ymd_and_hms(1988, 6, 8, 19, 30, 00).unwrap(),
    //     }
    // }

    // #[graphql(entity)]
    // async fn find_dangerous_product_by_id(&self, #[graphql(key)] _id: String) -> DangerousProduct {
    //     DangerousProduct {
    //         id: ID::from(Uuid::new_v4().to_string()),
    //         code: "DANG123".to_string(),
    //         description: "A dangerous product".to_string(),
    //         max_temperature: 75.0,
    //     }
    // }

    async fn product(&self, code: String) -> SimpleProduct {
        tracing::info!("Fetching product with code: {}", code);
        SimpleProduct {
            id: ID::from(Uuid::new_v4().to_string()),
            code: code.clone(),
            description: format!("Product with code: {}", code),
            pippo: "PIPPO".to_string(),
        }
    }
}

// #[derive(Interface)]
// #[graphql(
//     field(name = "id", ty = "&ID"),
//     field(name = "code", ty = "String"),
//     field(name = "description", ty = "String")
// )]
// pub enum Product {
//     DangerousProduct(DangerousProduct),
//     ExpiringProduct(ExpiringProduct),
// }

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct SimpleProduct {
    pub id: ID,
    pub code: String,
    pub description: String,
    pub pippo: String,
}

// #[derive(SimpleObject)]
// // #[graphql(shareable)]
// pub struct DangerousProduct {
//     #[graphql(override_from = "subgraph-orders")]
//     pub id: ID,
//     pub code: String,
//     pub description: String,
//     pub max_temperature: f64,
// }

// #[derive(SimpleObject)]
// // #[graphql(shareable)]
// pub struct ExpiringProduct {
//     #[graphql(override_from = "subgraph-orders")]
//     pub id: ID,
//     pub code: String,
//     pub description: String,
//     pub expiration_date: DateTime<Utc>,
// }

#[derive(Default)]
pub struct Mutation;

#[derive(Default)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn new_products(
        &self,
        #[graphql(default = 2)] interval: u64,
    ) -> impl Stream<Item = Vec<SimpleProduct>> {
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(
            interval,
        )))
        .map(move |_| {
            let new_product_added = rand::random::<u8>() % 3 + 1;
            let mut products = Vec::new();
            for i in 0..new_product_added {
                if i % 2 == 0 {
                    products.push(SimpleProduct {
                        id: ID::from(Uuid::new_v4().to_string()),
                        code: format!("EXP{}", Uuid::new_v4().to_string()),
                        description: "A newly added expiring product".to_string(),
                        pippo: "PIPPO".to_string(),
                    });
                } else {
                    products.push(SimpleProduct {
                        id: ID::from(Uuid::new_v4().to_string()),
                        code: format!("DANG{}", Uuid::new_v4().to_string()),
                        description: "A newly added dangerous product".to_string(),
                        pippo: "PIPPO".to_string(),
                    });
                }
            }
            products
        })
    }
}
