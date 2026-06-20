#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use astrology_website::infrastructure::{setup::init_app_state, app::create_app};
    use tracing::info;

    tracing_subscriber::fmt::init();
    info!("Загрузка конфигурации...");
    
    let app_state = init_app_state().await?;
    let app = create_app(app_state).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Сервер запущен на http://localhost:3000");

    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // WASM заглушка
}
