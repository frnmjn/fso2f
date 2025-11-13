//! Shared by the exercise servers (src/bin/server_NN.rs) that `cargo make servers`
//! starts on http://localhost:40NN/graphql, to call the exercises from Postman.

use async_graphql::{ObjectType, Schema, SubscriptionType, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn pool() -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://fso2f:fso2f@localhost:5432/fso2f")
        .await
}

/// Serves `schema` on http://localhost:<port>/graphql, with GraphiQL on GET.
pub async fn serve<Q, M, S>(port: u16, name: &str, schema: Schema<Q, M, S>)
where
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static,
{
    let app = Router::new().route("/graphql", get(graphiql).post_service(GraphQL::new(schema)));
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|e| panic!("cannot listen on port {port} for {name}: {e}"));
    tracing::info!("🚀 {name} running at http://localhost:{port}/graphql");
    axum::serve(listener, app).await.unwrap();
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
