use sqlx::{FromRow, Pool, Postgres, Result, query_as};

#[derive(Debug, Clone, FromRow)]
pub struct DbCustomer {
    pub id: String,
    pub name: String,
    pub vat: String,
    pub email: String,
    pub phone: String,
}

pub async fn get_customer_by_id(pool: &Pool<Postgres>, id: &str) -> Result<Option<DbCustomer>> {
    query_as!(DbCustomer, "SELECT * FROM customers WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}
