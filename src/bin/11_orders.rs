use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

use fso2f::_11_federated_subgraph::orders::Query;
use sqlx::{Error, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://fso2f:fso2f@localhost/fso2f".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .finish();

    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .route(
            "/graphql",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .with_state(pool.clone());

    tracing::info!("🚀 Orders subgraph running at http://0.0.0.0:3002/graphql");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
