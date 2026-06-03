use axum::{Router, routing::get};
use sqlx::{Error, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://fso2f:fso2f@localhost/fso2f")
        .await?;

    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .with_state(pool);

    tracing::info!("🚀 Monolith running at http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
