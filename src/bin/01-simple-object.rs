use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use fso2f::_01_simple_object::schema::QueryRoot;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://fso2f:fso2f@localhost/fso2f")
        .await?;

    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .with_state(pool);

    tracing::info!("🚀 Monolith running at http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
