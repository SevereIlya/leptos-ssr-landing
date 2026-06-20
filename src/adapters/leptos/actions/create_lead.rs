#[cfg(feature = "ssr")]
use crate::application::error::*;
#[cfg(feature = "ssr")]
use crate::application::usecases::commands::*;
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use tracing::{error, warn};

use leptos::prelude::*;
use leptos::server;

#[server(CreateLeadServerFn, "/api/create_lead")]
pub async fn create_lead_action(
    name: String,
    contact_method: String,
    contact_value: String,
    message: String,
    source: String,
    consent: bool,
) -> Result<String, ServerFnError> {
    let cmd = use_context::<Arc<CreateLeadCommand>>()
        .expect("Юзкейс не найден");

    let name = if name.trim().is_empty() {
        None
    } else {
        Some(name.trim().to_string())
    };
    let message = if message.trim().is_empty() {
        None
    } else {
        Some(message.trim().to_string())
    };

    match cmd
        .execute(
            name,
            contact_method,
            contact_value,
            message,
            source,
            consent,
        )
        .await
    {
        Ok(_) => Ok(format!("Заявка принята!")),
        Err(e) => {
            match &e {
                AppError::Domain(domain_err) => {
                    warn!("Validation failed for new lead: {:?}", domain_err);
                }
                _ => {
                    error!("CRITICAL ERROR during create_lead: {:?}", e);
                }
            }
            Err(ServerFnError::ServerError(e.message_error()))
        }
    }
}
