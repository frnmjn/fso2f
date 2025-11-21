use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use fso2f::schema::{MutationRoot, QueryRoot};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // create a db connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://fso2f:fso2f@localhost/fso2f")
        .await?;

    let schema = Schema::new(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    );

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .with_state(pool.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// basic handler that responds with a static string
async fn root(State(pool): State<Pool<Postgres>>) -> String {
    let (msg,): (String,) = sqlx::query_as("SELECT 'Hello, World!'")
        .fetch_one(&pool)
        .await
        .unwrap_or(("Error connecting to database".to_string(),));
    msg
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
