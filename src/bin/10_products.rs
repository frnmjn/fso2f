use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

use fso2f::_10_federation::products::{Mutation, Query};
use sqlx::{Error, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // create a db connection pool
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://fso2f:fso2f@localhost/fso2f".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool.clone())
        .finish();

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .route(
            "/graphql",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .with_state(pool.clone());

    tracing::info!("🚀 Products subgraph running at http://0.0.0.0:3001/graphql");

    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
