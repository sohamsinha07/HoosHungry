mod db;
mod graphql;
mod redis_cache;
mod schema;
mod models;

use axum::{routing::get, Router};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use tracing_subscriber::EnvFilter;

use crate::graphql::{AppSchema, AppState, QueryRoot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_url = std::env::var("DATABASE_URL")?;
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let pool = db::connect(&db_url).await?;
    let cache = redis_cache::RedisCache::new(&redis_url)?;

    let state = AppState { db: pool, cache };

    let schema: AppSchema = async_graphql::Schema::build(QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .data(state)
        .finish();

    let app = Router::new()
        .route("/", get(|| async { "HoosHungry backend OK" }))
        .route("/graphql", axum::routing::post(move |req: GraphQLRequest| graphql_handler(schema.clone(), req)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Backend listening on :8080");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn graphql_handler(schema: AppSchema, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
