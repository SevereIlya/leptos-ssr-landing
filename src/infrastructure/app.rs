pub mod api;
pub mod event_bus;
pub mod ssr;

use crate::adapters::leptos::*;
use crate::infrastructure::setup::*;
use axum::Router;
use tracing::info;

pub async fn create_app(state: AppState) -> Router {
    info!("Сборка роутера Axum...");

    // Создаем новый роутер
    let app = Router::new().merge(api::router());

    // Leptos SSR
    let app = ssr::attach_ssr_routes(app, &state);
    
    app.fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state)
}
