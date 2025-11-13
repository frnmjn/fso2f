//! In-memory copy of the database seed (migrations/20260407_seed.sql), used by
//! exercises 01-06 before the schema talks to the database.
//!
//! The functions mirror the ones in `db::products` and `db::orders`: same names,
//! same return types, but synchronous and without the pool.

use chrono::{DateTime, TimeZone, Utc};

use super::orders::{DbOrder, DbOrderLine};
use super::products::{DbDangerousProduct, DbExpiringProduct, DbProduct, DbProductKind};

fn date(y: i32, m: u32, d: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(y, m, d, 0, 0, 0).unwrap()
}

fn products() -> Vec<DbProductKind> {
    vec![
        DbProductKind::Product(DbProduct {
            id: "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d".into(),
            code: "WIDGET-001".into(),
            description: "Widget".into(),
        }),
        DbProductKind::Expiring(DbExpiringProduct {
            id: "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e".into(),
            code: "CHEESE-002".into(),
            description: "Cheese".into(),
            expiration_date: date(2027, 6, 15),
        }),
        DbProductKind::Dangerous(DbDangerousProduct {
            id: "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f".into(),
            code: "PISTOL-003".into(),
            description: "Pistol".into(),
            max_temperature: 75.0,
        }),
    ]
}

fn orders() -> Vec<DbOrder> {
    let order = |id: &str, customer: (&str, &str, &str), total_amount: f64, status: &str| DbOrder {
        id: id.into(),
        customer_id: customer.0.into(),
        customer_name: customer.1.into(),
        customer_vat: customer.2.into(),
        total_amount,
        status: status.into(),
        created_at: date(2026, 4, 7),
    };
    let acme = (
        "11111111-1111-4111-8111-111111111111",
        "Acme Srl (From Order)",
        "IT01234567890",
    );
    let beta = (
        "22222222-2222-4222-8222-222222222222",
        "Beta SpA (From Order)",
        "IT09876543210",
    );
    let gamma = (
        "33333333-3333-4333-8333-333333333333",
        "Gamma Snc (From Order)",
        "IT11223344556",
    );
    vec![
        order("aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa", acme, 150.00, "Confirmed"),
        order("bbbb2222-bb22-4bb2-8bb2-bbbbbbbbbbbb", acme, 42.50, "Draft"),
        order("cccc3333-cc33-4cc3-8cc3-cccccccccccc", beta, 320.75, "Confirmed"),
        order("dddd4444-dd44-4dd4-8dd4-dddddddddddd", gamma, 89.90, "Deleted"),
    ]
}

fn order_lines() -> Vec<DbOrderLine> {
    let line = |n: u8,
                order_id: &str,
                product_id: &str,
                line_number: i32,
                quantity: i32,
                price: f64,
                discount: Option<f64>| DbOrderLine {
        id: format!("10000000-0000-4000-8000-00000000000{n}"),
        order_id: order_id.into(),
        product_id: product_id.into(),
        line_number,
        quantity,
        price,
        discount,
    };
    let (a, b, c, d) = (
        "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa",
        "bbbb2222-bb22-4bb2-8bb2-bbbbbbbbbbbb",
        "cccc3333-cc33-4cc3-8cc3-cccccccccccc",
        "dddd4444-dd44-4dd4-8dd4-dddddddddddd",
    );
    let (widget, cheese, pistol) = (
        "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d",
        "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e",
        "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f",
    );
    vec![
        line(1, a, widget, 1, 5, 12.50, Some(2.00)),
        line(2, a, pistol, 2, 10, 7.50, None),
        line(3, b, cheese, 1, 1, 42.50, Some(5.00)),
        line(4, c, widget, 1, 3, 12.50, None),
        line(5, c, cheese, 2, 7, 35.00, Some(10.00)),
        line(6, d, pistol, 1, 2, 44.95, None),
    ]
}

/// Like `db::products::get_product_by_id`: the common fields of any product.
pub fn get_product_by_id(id: &str) -> Option<DbProduct> {
    retrieve_product_by_id(id).map(common_fields)
}

/// The common fields of any product, looked up by code (no database counterpart).
pub fn get_product_by_code(code: &str) -> Option<DbProduct> {
    retrieve_product_by_code(code).map(common_fields)
}

/// Like `db::products::retrieve_product_by_id`.
pub fn retrieve_product_by_id(id: &str) -> Option<DbProductKind> {
    products().into_iter().find(|p| p.id() == id)
}

/// Like `db::products::retrieve_product_by_code`.
pub fn retrieve_product_by_code(code: &str) -> Option<DbProductKind> {
    products()
        .into_iter()
        .find(|p| common_fields(p.clone()).code == code)
}

/// Like `db::orders::get_order_by_id`.
pub fn get_order_by_id(id: &str) -> Option<DbOrder> {
    orders().into_iter().find(|o| o.id == id)
}

/// Like `db::orders::get_order_lines_by_order_id`.
pub fn get_order_lines_by_order_id(order_id: &str) -> Vec<DbOrderLine> {
    order_lines()
        .into_iter()
        .filter(|l| l.order_id == order_id)
        .collect()
}

fn common_fields(product: DbProductKind) -> DbProduct {
    match product {
        DbProductKind::Product(p) => p,
        DbProductKind::Dangerous(p) => DbProduct {
            id: p.id,
            code: p.code,
            description: p.description,
        },
        DbProductKind::Expiring(p) => DbProduct {
            id: p.id,
            code: p.code,
            description: p.description,
        },
    }
}
