use crate::adapters::email::notifier::*;
use crate::adapters::leptos::*;
use crate::adapters::telegram::notifier::*;
use crate::application::usecases::commands::*;
use crate::domain::events::*;
use crate::domain::lead::repository::*;
use crate::infrastructure::config::*;
use crate::infrastructure::database::*;
use crate::infrastructure::event_bus::*;
use axum::extract::FromRef;
use leptos::prelude::*;
use leptos_axum::{AxumRouteListing, generate_route_list};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<AxumRouteListing>,
    pub create_lead_cmd: Arc<CreateLeadCommand>,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.leptos_options.clone()
    }
}

pub async fn init_app_state() -> anyhow::Result<AppState> {
    // Конфиг и БД
    let config: AppConfig = AppConfig::load()?;
    let pool: PgPool = create_pg_pool(&config.database_url).await?;
    sqlx::migrate!().run(&pool).await?;

    // Репозитории и ивенты
    let repo: DynLeadRepository = Arc::new(SqlxLeadRepository::new(pool));
    let telegram_notifier: DynEventPublisher = Arc::new(TelegramNotifier::new(
        &config.telegram_bot_token,
        config.telegram_chat_id,
    )?);
    let email_notifier: DynEventPublisher = Arc::new(EmailNotifier::new(
        &config.smpt.host,
        &config.smpt.user,
        &config.smpt.password,
        &config.smpt.admin_email,
    )?);
    let event_publisher: DynEventPublisher = Arc::new(CompositeEventPublisher::new(vec![
        telegram_notifier,
        email_notifier,
    ]));

    // Юзкейсы
    let create_lead_cmd = Arc::new(CreateLeadCommand::new(repo, event_publisher));

    // Leptos
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    Ok(AppState {
        leptos_options,
        routes,
        create_lead_cmd,
    })
}
