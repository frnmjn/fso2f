use async_graphql::{ID, MergedObject, Object, SimpleObject};

#[derive(MergedObject, Default)]
pub struct Query(FakeCustomerQuery);

#[derive(Default)]
pub struct FakeCustomerQuery;

#[Object]
impl FakeCustomerQuery {
    #[graphql(entity)]
    async fn find_customer_by_id(&self, #[graphql(key)] id: ID) -> Customer {
        tracing::info!("Resolving customer entity with id: {}", id.to_string());
        Customer { id: id.clone() }
    }
}

#[derive(SimpleObject)]
pub struct Customer {
    pub id: ID,
}

#[derive(Default)]
pub struct Mutation;

#[derive(Default)]
pub struct Subscription;
