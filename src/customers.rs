use async_graphql::{ID, MergedObject, Object, SimpleObject};
use uuid::Uuid;

#[derive(MergedObject, Default)]
pub struct Query(CustomerQuery);

#[derive(Default)]
pub struct CustomerQuery;

#[Object]
impl CustomerQuery {
    async fn customer(&self, id: ID) -> Customer {
        tracing::info!("Fetching customer with id: {}", id.to_string());
        Customer {
            id: id.clone(),
            // name: "John Doe From Customer".to_string(),
            // email: "john.doe@example.com".to_string(),
            // phone: Some("+39 123 456 7890".to_string()),
        }
    }

    #[graphql(entity)]
    async fn find_customer_by_id(&self, #[graphql(key)] id: ID) -> Customer {
        tracing::info!("Resolving customer entity with id: {}", id.to_string());
        Customer {
            id: id.clone(),
            // name: "John Doe From Customer".to_string(),
            // email: "john.doe@example.com".to_string(),
            // phone: Some("+39 123 456 7890".to_string()),
        }
    }
}

#[derive(SimpleObject)]
pub struct Customer {
    pub id: ID,
    // #[graphql(override_from = "orders")]
    // pub name: String,
    // /// New fields owned by customers subgraph
    // pub email: String,
    // pub phone: Option<String>,
}

#[derive(Default)]
pub struct Mutation;

#[derive(Default)]
pub struct Subscription;
