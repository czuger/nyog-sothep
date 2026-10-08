use nyog_sothep::config::AppConfig;
use nyog_sothep::db;
use nyog_sothep::web::{AppState, build_router};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let config = AppConfig::from_env();
    let pool = db::connect(&config.database_url).await?;
    let router = build_router(AppState { pool }, &config.asset_root);
    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!("listening on http://{}", config.bind_addr);
    axum::serve(listener, router).await?;
    Ok(())
}
