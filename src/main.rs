use nyog_sothep::config::AppConfig;
use nyog_sothep::web::{AppState, build_router};
use nyog_sothep::{db, server};
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
    let router = build_router(AppState { pool: pool.clone() });
    server::serve(router, &config.listen_on).await?;
    pool.close().await;
    Ok(())
}
