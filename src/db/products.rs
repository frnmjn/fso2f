use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum DbProductKind {
    Product(DbProduct),
    Dangerous(DbDangerousProduct),
    Expiring(DbExpiringProduct),
}
#[derive(Debug, Clone, FromRow)]
pub struct DbProduct {
    pub id: Uuid,
    pub code: String,
    pub description: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbDangerousProduct {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub max_temperature: f64,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbExpiringProduct {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}

pub async fn get_product_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> sqlx::Result<Option<DbProductKind>> {
    let row: Option<(String,)> = sqlx::query_as("SELECT kind FROM products WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    match row.as_ref().map(|r| r.0.as_str()) {
        Some("dangerous") => Ok(get_dangerous_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Dangerous)),
        Some("expiring") => Ok(get_expiring_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Expiring)),
        Some(_) => Ok(get_standard_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Product)),
        None => Ok(None),
    }
}

pub async fn get_standard_product_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> sqlx::Result<Option<DbProduct>> {
    sqlx::query_as::<_, DbProduct>("SELECT * FROM products WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_dangerous_product_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> sqlx::Result<Option<DbDangerousProduct>> {
    sqlx::query_as::<_, DbDangerousProduct>(
        "SELECT p.id, p.code, p.description, dp.max_temperature \
         FROM products p JOIN dangerous_products dp ON p.id = dp.id \
         WHERE p.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_expiring_product_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> sqlx::Result<Option<DbExpiringProduct>> {
    sqlx::query_as::<_, DbExpiringProduct>(
        "SELECT p.id, p.code, p.description, ep.expiration_date \
         FROM products p JOIN expiring_products ep ON p.id = ep.id \
         WHERE p.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
