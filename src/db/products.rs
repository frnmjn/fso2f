use sqlx::{FromRow, Pool, Postgres, Result, query, query_as};

#[derive(Debug, Clone)]
pub enum DbProductKind {
    Product(DbProduct),
    Dangerous(DbDangerousProduct),
    Expiring(DbExpiringProduct),
}

impl DbProductKind {
    pub fn id(&self) -> &str {
        match self {
            DbProductKind::Product(p) => &p.id,
            DbProductKind::Dangerous(p) => &p.id,
            DbProductKind::Expiring(p) => &p.id,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct DbProduct {
    pub id: String,
    pub code: String,
    pub description: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbDangerousProduct {
    pub id: String,
    pub code: String,
    pub description: String,
    pub max_temperature: f64,
}

#[derive(Debug, Clone, FromRow)]
pub struct DbExpiringProduct {
    pub id: String,
    pub code: String,
    pub description: String,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}

pub async fn retrieve_product_by_id(
    pool: &Pool<Postgres>,
    id: &str,
) -> Result<Option<DbProductKind>> {
    struct KindRow {
        kind: String,
    }
    let row = query_as!(KindRow, "SELECT kind FROM products WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;
    match row.as_ref().map(|r| r.kind.as_str()) {
        Some("dangerous") => Ok(get_dangerous_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Dangerous)),
        Some("expiring") => Ok(get_expiring_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Expiring)),
        Some(_) => Ok(get_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Product)),
        None => Ok(None),
    }
}

pub async fn retrieve_product_by_code(
    pool: &Pool<Postgres>,
    code: &str,
) -> Result<Option<DbProductKind>> {
    struct IdKindRow {
        id: String,
        kind: String,
    }
    let row = query_as!(
        IdKindRow,
        "SELECT id, kind FROM products WHERE code = $1",
        code
    )
    .fetch_optional(pool)
    .await?;
    match row.as_ref().map(|r| (r.id.as_str(), r.kind.as_str())) {
        Some((id, "dangerous")) => Ok(get_dangerous_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Dangerous)),
        Some((id, "expiring")) => Ok(get_expiring_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Expiring)),
        Some((id, _)) => Ok(get_product_by_id(pool, id)
            .await?
            .map(DbProductKind::Product)),
        None => Ok(None),
    }
}

pub async fn get_product_by_id(pool: &Pool<Postgres>, id: &str) -> Result<Option<DbProduct>> {
    query_as!(
        DbProduct,
        "SELECT id, code, description FROM products WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn retrieve_products_by_ids(
    pool: &Pool<Postgres>,
    ids: &[String],
) -> Result<Vec<DbProductKind>> {
    struct ProductJoinRow {
        id: String,
        code: String,
        description: String,
        kind: String,
        max_temperature: Option<f64>,
        expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    }
    let rows = query_as!(
        ProductJoinRow,
        "SELECT p.id, p.code, p.description, p.kind, dp.max_temperature, ep.expiration_date \
         FROM products p \
         LEFT JOIN dangerous_products dp ON p.id = dp.id \
         LEFT JOIN expiring_products ep ON p.id = ep.id \
         WHERE p.id = ANY($1)",
        ids
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| match row.kind.as_str() {
            "dangerous" => DbProductKind::Dangerous(DbDangerousProduct {
                id: row.id,
                code: row.code,
                description: row.description,
                max_temperature: row.max_temperature.unwrap_or_default(),
            }),
            "expiring" => DbProductKind::Expiring(DbExpiringProduct {
                id: row.id,
                code: row.code,
                description: row.description,
                expiration_date: row.expiration_date.unwrap_or_default(),
            }),
            _ => DbProductKind::Product(DbProduct {
                id: row.id,
                code: row.code,
                description: row.description,
            }),
        })
        .collect())
}

pub async fn get_dangerous_product_by_id(
    pool: &Pool<Postgres>,
    id: &str,
) -> Result<Option<DbDangerousProduct>> {
    query_as!(
        DbDangerousProduct,
        "SELECT p.id, p.code, p.description, dp.max_temperature \
         FROM products p JOIN dangerous_products dp ON p.id = dp.id \
         WHERE p.id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_expiring_product_by_id(
    pool: &Pool<Postgres>,
    id: &str,
) -> Result<Option<DbExpiringProduct>> {
    query_as!(
        DbExpiringProduct,
        "SELECT p.id, p.code, p.description, ep.expiration_date \
         FROM products p JOIN expiring_products ep ON p.id = ep.id \
         WHERE p.id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn insert_product(
    pool: &Pool<Postgres>,
    product: DbProductKind,
) -> Result<DbProductKind> {
    Ok(match product {
        DbProductKind::Product(p) => {
            DbProductKind::Product(insert_standard_product(pool, p).await?)
        }
        DbProductKind::Dangerous(p) => {
            DbProductKind::Dangerous(insert_dangerous_product(pool, p).await?)
        }
        DbProductKind::Expiring(p) => {
            DbProductKind::Expiring(insert_expiring_product(pool, p).await?)
        }
    })
}

pub async fn insert_standard_product(
    pool: &Pool<Postgres>,
    product: DbProduct,
) -> Result<DbProduct> {
    query_as!(
        DbProduct,
        "INSERT INTO products (id, code, description, kind) \
         VALUES ($1, $2, $3, $4) \
         RETURNING id, code, description",
        product.id,
        product.code,
        product.description,
        "standard"
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_dangerous_product(
    pool: &Pool<Postgres>,
    product: DbDangerousProduct,
) -> Result<DbDangerousProduct> {
    let mut tx = pool.begin().await?;

    query(
        "INSERT INTO products (id, code, description, kind) \
         VALUES ($1, $2, $3, $4)",
    )
    .bind(&product.id)
    .bind(&product.code)
    .bind(&product.description)
    .bind("dangerous")
    .execute(&mut *tx)
    .await?;

    query("INSERT INTO dangerous_products (id, max_temperature) VALUES ($1, $2)")
        .bind(&product.id)
        .bind(product.max_temperature)
        .execute(&mut *tx)
        .await?;

    let row = query_as!(
        DbDangerousProduct,
        "SELECT p.id, p.code, p.description, dp.max_temperature \
         FROM products p JOIN dangerous_products dp ON p.id = dp.id \
         WHERE p.id = $1",
        product.id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(row)
}

pub async fn insert_expiring_product(
    pool: &Pool<Postgres>,
    product: DbExpiringProduct,
) -> Result<DbExpiringProduct> {
    let mut tx = pool.begin().await?;

    query(
        "INSERT INTO products (id, code, description, kind) \
         VALUES ($1, $2, $3, $4)",
    )
    .bind(&product.id)
    .bind(&product.code)
    .bind(&product.description)
    .bind("expiring")
    .execute(&mut *tx)
    .await?;

    query("INSERT INTO expiring_products (id, expiration_date) VALUES ($1, $2)")
        .bind(&product.id)
        .bind(product.expiration_date)
        .execute(&mut *tx)
        .await?;

    let row = query_as!(
        DbExpiringProduct,
        "SELECT p.id, p.code, p.description, ep.expiration_date \
         FROM products p JOIN expiring_products ep ON p.id = ep.id \
         WHERE p.id = $1",
        product.id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(row)
}
