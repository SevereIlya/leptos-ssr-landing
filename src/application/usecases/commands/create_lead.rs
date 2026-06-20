use crate::application::error::*;
use crate::domain::error::*;
use crate::domain::events::*;
use crate::domain::lead::entity::*;
use crate::domain::lead::repository::*;
use crate::domain::lead::value_objects::*;

pub struct CreateLeadCommand {
    lead_repo: DynLeadRepository,
    event_publisher: DynEventPublisher,
}

impl CreateLeadCommand {
    pub fn new(lead_repo: DynLeadRepository, event_publisher: DynEventPublisher) -> Self {
        Self {
            lead_repo,
            event_publisher,
        }
    }

    pub async fn execute(
        &self,
        name: Option<String>,
        contact_method: String,
        contact_value: String,
        message: Option<String>,
        source: String,
        consent: bool,
    ) -> AppResult<Lead> {
        let name = match name {
            Some(name) => Some(
                Name::try_new(name)
                    .map_err(|_| DomainError::InvalidInput("Имя не может быть пустым".into()))?,
            ),
            None => None,
        };

        let contact_method = ContactMethod::try_from(contact_method.as_str())?;

        let contact_value = ContactValue::try_new(contact_value)
            .map_err(|_| DomainError::InvalidInput("Контакт не может быть пустым".into()))?;

        let message =
            match message {
                Some(message) => Some(Message::try_new(message).map_err(|_| {
                    DomainError::InvalidInput("Сообщение не может быть пустым".into())
                })?),
                None => None,
            };

        let source = LeadSource::try_new(source)
            .map_err(|_| DomainError::InvalidInput("Источник не может быть пустым".into()))?;

        let mut new_lead = Lead::new(
            name,
            contact_method,
            contact_value,
            message,
            source,
            consent,
        )?;
        self.lead_repo.create(&new_lead).await?;

        for event in new_lead.take_events() {
            self.event_publisher.publish(DomainEvent::Lead(event)).await;
        }

        Ok(new_lead)
    }
}
