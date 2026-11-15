mod auth;
mod error;
mod handlers;
mod models;
mod pagination;
mod repo;
mod routes;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL")?;
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0.3080".into());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    let listener = TcpListener::bind(&bind_addr).await?;
    tracing::info!("listening on {bind_addr}");
    axum::serve(listener, routes::router(pool)).await?;

    Ok(())
}
