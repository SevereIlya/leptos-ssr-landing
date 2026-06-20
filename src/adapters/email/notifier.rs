use crate::domain::events::*;
use crate::domain::lead::events::*;
use async_trait::async_trait;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use tracing::{error, info};

pub struct EmailNotifier {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    to_email: String,
    from_email: String,
}

impl EmailNotifier {
    pub fn new(
        host: &str,
        username: &str,
        password: &str,
        admin_email: &str,
    ) -> anyhow::Result<Self> {
        let creds = Credentials::new(username.into(), password.into());
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(host)?.credentials(creds).build();
        Ok(Self {
            mailer,
            to_email: admin_email.to_owned(),
            from_email: username.to_owned(),
        })
    }
}

#[async_trait]
impl EventPublisher for EmailNotifier {
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
                let name_str: String = name
                    .map(|n| n.as_ref().to_string())
                    .unwrap_or_else(|| "Не указано".to_string());

                let msg_str: String = message
                    .map(|n| n.as_ref().to_string())
                    .unwrap_or_else(|| "Без сообщения".to_string());

                let email_body = format!(
                "<h2>Новый лид на сайте Astra Regalis</h2>\
                <p><strong>ID:</strong> {}</p>\
                <p><strong>Источник:</strong> {}</p>\
                <p><strong>Имя:</strong> {}</p>\
                <p><strong>Способ связи:</strong> {}</p>\
                <p><strong>Контакт:</strong> {}</p>\
                <br>\
                <p><strong>Вопрос/Сообщение:</strong><br>{}</p>",
                    id,
                    source.as_ref(),
                    name_str,
                    contact_method,
                    contact_value.as_ref(),
                    msg_str
                );

                let email = Message::builder()
                    .from(self.from_email.parse().unwrap())
                    .to(self.to_email.parse().unwrap())
                    .subject("Новая заявка с лендинга")
                    .header(ContentType::TEXT_HTML)
                    .body(email_body)
                    .unwrap();

                match self.mailer.send(email).await {
                    Ok(_) => info!("Отправлено уведомление по электронной почте о лиде {}", id),
                    Err(e) => error!("Не удалось отправить уведомление по электронной почте: {}", e),
                }
            }
        }
    }
}
