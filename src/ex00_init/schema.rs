use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    async fn greeting(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }
}
