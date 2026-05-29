use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct DbOrder {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub customer_name: String,
    pub customer_vat: String,
    pub total_amount: f64,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbOrderLine {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub line_number: i32,
    pub quantity: i32,
    pub price: f64,
    pub discount: Option<f64>,
}

pub async fn get_order_by_id(pool: &Pool<Postgres>, id: Uuid) -> sqlx::Result<Option<DbOrder>> {
    sqlx::query_as::<_, DbOrder>("SELECT * FROM orders WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_order_lines_by_order_id(
    pool: &Pool<Postgres>,
    order_id: Uuid,
) -> sqlx::Result<Vec<DbOrderLine>> {
    sqlx::query_as::<_, DbOrderLine>(
        "SELECT * FROM order_lines WHERE order_id = $1 ORDER BY line_number",
    )
    .bind(order_id)
    .fetch_all(pool)
    .await
}

pub async fn get_order_by_customer_id(
    pool: &Pool<Postgres>,
    customer_id: Uuid,
) -> sqlx::Result<Option<DbOrder>> {
    sqlx::query_as::<_, DbOrder>("SELECT * FROM orders WHERE customer_id = $1")
        .bind(customer_id)
        .fetch_optional(pool)
        .await
}

pub async fn insert_order(
    pool: &Pool<Postgres>,
    id: Uuid,
    customer_id: Uuid,
    customer_name: &str,
    customer_vat: &str,
    total_amount: f64,
    status: &str,
) -> sqlx::Result<DbOrder> {
    sqlx::query_as::<_, DbOrder>(
        "INSERT INTO orders (id, customer_id, customer_name, customer_vat, total_amount, status) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(id)
    .bind(customer_id)
    .bind(customer_name)
    .bind(customer_vat)
    .bind(total_amount)
    .bind(status)
    .fetch_one(pool)
    .await
}

pub async fn insert_order_line(
    pool: &Pool<Postgres>,
    id: Uuid,
    order_id: Uuid,
    product_id: Uuid,
    line_number: i32,
    quantity: i32,
) -> sqlx::Result<DbOrderLine> {
    sqlx::query_as::<_, DbOrderLine>(
        "INSERT INTO order_lines (id, order_id, product_id, line_number, quantity) \
         VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(id)
    .bind(order_id)
    .bind(product_id)
    .bind(line_number)
    .bind(quantity)
    .fetch_one(pool)
    .await
}
