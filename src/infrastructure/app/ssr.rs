use crate::adapters::leptos::ui::*;
use crate::infrastructure::setup::*;
use axum::Router;
use leptos::prelude::*;
use leptos_axum::LeptosRoutes;

pub fn attach_ssr_routes(router: Router<AppState>, state: &AppState) -> Router<AppState> {
    let cmd_for_routes = state.create_lead_cmd.clone();
    let leptos_options = state.leptos_options.clone();

    router.leptos_routes_with_context(
        state,
        state.routes.clone(),
        move || {
            provide_context(cmd_for_routes.clone());
        },
        move || shell(leptos_options.clone()),
    )
}
