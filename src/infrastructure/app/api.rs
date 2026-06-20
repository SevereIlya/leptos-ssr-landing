use crate::infrastructure::setup::*;
use axum::{
    Router,
    extract::{Request, State},
    response::IntoResponse,
    routing::post,
};
use leptos::prelude::*;
use leptos_axum::handle_server_fns_with_context;

pub fn router() -> Router<AppState> {
    Router::new().route("/api/{*fn_name}", post(server_fn_handler))
}

async fn server_fn_handler(State(state): State<AppState>, req: Request) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(state.create_lead_cmd.clone());
        },
        req,
    )
    .await
}
