use sqlx::{FromRow, Pool, Postgres, Result, query_as};

#[derive(Debug, Clone, FromRow)]
pub struct DbOrder {
    pub id: String,
    pub customer_id: String,
    pub customer_name: String,
    pub customer_vat: String,
    pub total_amount: f64,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbOrderLine {
    pub id: String,
    pub order_id: String,
    pub product_id: String,
    pub line_number: i32,
    pub quantity: i32,
    pub price: f64,
    pub discount: Option<f64>,
}

pub async fn get_order_by_id(pool: &Pool<Postgres>, id: &str) -> Result<Option<DbOrder>> {
    query_as!(DbOrder, "SELECT * FROM orders WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}

pub async fn get_order_lines_by_order_id(
    pool: &Pool<Postgres>,
    order_id: &str,
) -> Result<Vec<DbOrderLine>> {
    query_as!(
        DbOrderLine,
        "SELECT * FROM order_lines WHERE order_id = $1 ORDER BY line_number",
        order_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_order_by_customer_id(
    pool: &Pool<Postgres>,
    customer_id: &str,
) -> Result<Option<DbOrder>> {
    query_as!(
        DbOrder,
        "SELECT * FROM orders WHERE customer_id = $1",
        customer_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn insert_order(
    pool: &Pool<Postgres>,
    id: &str,
    customer_id: &str,
    customer_name: &str,
    customer_vat: &str,
    total_amount: f64,
    status: &str,
) -> Result<DbOrder> {
    query_as!(
        DbOrder,
        "INSERT INTO orders (id, customer_id, customer_name, customer_vat, total_amount, status) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        id,
        customer_id,
        customer_name,
        customer_vat,
        total_amount,
        status
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_order_line(
    pool: &Pool<Postgres>,
    id: &str,
    order_id: &str,
    product_id: &str,
    line_number: i32,
    quantity: i32,
) -> Result<DbOrderLine> {
    query_as!(
        DbOrderLine,
        "INSERT INTO order_lines (id, order_id, product_id, line_number, quantity) \
         VALUES ($1, $2, $3, $4, $5) RETURNING *",
        id,
        order_id,
        product_id,
        line_number,
        quantity
    )
    .fetch_one(pool)
    .await
}
