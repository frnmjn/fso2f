use std::time::Duration;

use async_graphql::{
    ComplexObject, ID, InputObject, Interface, MergedObject, Object, OneofObject, SimpleObject,
    Subscription,
};
use sqlx::types::chrono::{DateTime, TimeZone, Utc};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct Query(ProductQuery);

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    #[graphql(entity)]
    async fn find_simple_product_by_id(&self, id: ID) -> SimpleProduct {
        SimpleProduct {
            id,
            code: "SIMPLE-001".to_string(),
            description: "A simple product".to_string(),
        }
    }

    // #[graphql(entity)]
    // async fn find_expiring_product_by_id(&self, #[graphql(key)] _id: ID) -> ExpiringProduct {
    //     ExpiringProduct {
    //         expiration_date: Utc.with_ymd_and_hms(1988, 6, 8, 19, 30, 00).unwrap(),
    //     }
    // }

    // #[graphql(entity)]
    // async fn find_dangerous_product_by_id(&self, #[graphql(key)] _id: ID) -> DangerousProduct {
    //     DangerousProduct {
    //         max_temperature: 75.0,
    //     }
    // }

    // async fn product(&self, code: String) -> Product {
    //     tracing::info!("Fetching product with code: {}", code);
    //     if code.starts_with("EXP") {
    //         return Product::ExpiringProduct(ExpiringProduct {
    //             expiration_date: Utc.with_ymd_and_hms(1988, 6, 8, 19, 30, 00).unwrap(),
    //         });
    //     }
    //     Product::DangerousProduct(DangerousProduct {
    //         max_temperature: 75.0,
    //     })
    // }
    async fn product(&self, code: String) -> SimpleProduct {
        tracing::info!("Fetching product with code: {}", code);
        SimpleProduct {
            id: ID::from(Uuid::new_v4().to_string()),
            code: code.clone(),
            description: format!("Product with code: {}", code),
        }
    }
}

// #[derive(Interface, OneofObject)]
// #[graphql(
//     field(name = "id", ty = "ID"),
//     field(name = "code", ty = "String"),
//     field(name = "description", ty = "String"),
//     input_name = "ProductInput"
// )]
// pub enum Product {
//     DangerousProduct(DangerousProduct),
//     ExpiringProduct(ExpiringProduct),
// }

#[derive(SimpleObject)]
pub struct SimpleProduct {
    pub id: ID,
    pub code: String,
    pub description: String,
}

// #[derive(SimpleObject, InputObject)]
// #[graphql(complex, input_name = "DangerousProductInput")]
// pub struct DangerousProduct {
//     pub max_temperature: f64,
// }

// #[ComplexObject]
// impl DangerousProduct {
//     async fn id(&self) -> ID {
//         ID::from(Uuid::new_v4().to_string())
//     }

//     async fn code(&self) -> String {
//         format!("DANG-{}", self.max_temperature as u32)
//     }

//     async fn description(&self) -> String {
//         "A dangerous product".to_string()
//     }
// }

// #[derive(SimpleObject, InputObject)]
// #[graphql(complex, input_name = "ExpiringProductInput")]
// pub struct ExpiringProduct {
//     pub expiration_date: DateTime<Utc>,
// }

// #[ComplexObject]
// impl ExpiringProduct {
//     async fn id(&self) -> ID {
//         ID::from(Uuid::new_v4().to_string())
//     }

//     async fn code(&self) -> String {
//         format!("EXP-{}", self.expiration_date.timestamp())
//     }

//     async fn description(&self) -> String {
//         "An expiring product".to_string()
//     }
// }

#[derive(Default)]
pub struct Mutation;

#[derive(Default)]
pub struct Subscription;

// #[Subscription]
// impl Subscription {
//     async fn new_products(
//         &self,
//         #[graphql(default = 2)] interval: u64,
//     ) -> impl Stream<Item = Vec<Product>> {
//         tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(
//             interval,
//         )))
//         .map(move |_| {
//             let new_product_added = rand::random::<u8>() % 3 + 1;
//             let mut products = Vec::new();
//             for i in 0..new_product_added {
//                 if i % 2 == 0 {
//                     products.push(Product::ExpiringProduct(ExpiringProduct {
//                         expiration_date: Utc::now() + chrono::Duration::days(i.into()),
//                     }));
//                 } else {
//                     products.push(Product::DangerousProduct(DangerousProduct {
//                         max_temperature: 60.0 + (rand::random::<u32>() as f64),
//                     }));
//                 }
//             }
//             products
//         })
//     }
// }
