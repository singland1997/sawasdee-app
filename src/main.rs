use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use sawasdee_api::config::AppConfig;
use sawasdee_api::db::postgres_connector::setup_pool;
use sawasdee_api::router::build_router;
use sawasdee_api::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sawasdee=debug,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Sawasdee API...");

    let app_config = AppConfig::from_env()?;

    tracing::info!("Connecting to database...");
    let pool = setup_pool(&app_config.database_url).await?;

    let app_state = AppState {
        db: pool,
        jwt_secret: app_config.jwt_secret,
    };

    let app = build_router(app_state);

    let addr = format!("0.0.0.0:{}", app_config.port);

    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}