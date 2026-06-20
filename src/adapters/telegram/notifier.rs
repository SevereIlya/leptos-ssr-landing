use crate::domain::events::*;
use crate::domain::lead::events::*;
use async_trait::async_trait;
use teloxide::prelude::*;
use teloxide::types::{ChatId, ParseMode};
use teloxide::utils::html::escape;
use tracing::{error, info};

pub struct TelegramNotifier {
    bot: Bot,
    chat_id: ChatId,
}

impl TelegramNotifier {
    pub fn new(token: &str, chat_id: i64) -> anyhow::Result<Self> {
        let bot = Bot::new(token);
        Ok(Self {
            bot,
            chat_id: ChatId(chat_id),
        })
    }
}

#[async_trait]
impl EventPublisher for TelegramNotifier {
    async fn publish(&self, event: DomainEvent) {
        let DomainEvent::Lead(lead_event) = event;

        match lead_event {
            LeadEvent::Created {
                id,
                name,
                contact_method,
                contact_value,
                message,
                source,
            } => {
                let name_str = name
                    .map(|name| name.as_ref().to_string())
                    .unwrap_or_else(|| "Не указано".to_string());
                let msg_str = message
                    .map(|m| m.as_ref().to_string())
                    .unwrap_or_else(|| "Без сообщения".to_string());

                let text = format!(
                "<b>Новый лид на сайте!</b>\n\n\
                <b>ID:</b> <code>{}</code>\n\
                <b>Источник:</b> {}\n\
                <b>Имя:</b> {}\n\
                <b>Способ связи:</b> {}\n\
                <b>Контакт:</b> <code>{}</code>\n\n\
                <b>Вопрос/Сообщение:</b>\n{}",
                    id,
                    escape(source.as_ref()),
                    escape(&name_str),
                    contact_method,
                    escape(contact_value.as_ref()),
                    escape(&msg_str)
                );

                match self.bot.send_message(self.chat_id, &text).parse_mode(ParseMode::Html).await {
                    Ok(_) => info!("Отправлено уведомление в Telegram о лиде {}", id),
                    Err(e) => error!("Не удалось отправить уведомление в Telegram: {}", e),
                }
            }
        }
    }
}
