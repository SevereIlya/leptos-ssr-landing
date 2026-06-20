use crate::domain::error::*;
use crate::domain::lead::entity::*;
use crate::domain::lead::value_objects::*;
use crate::infrastructure::database::models::*;

impl TryFrom<LeadRow> for Lead {
    type Error = DomainError;

    fn try_from(value: LeadRow) -> Result<Self, Self::Error> {
        let name = value
            .name
            .map(|n| {
                Name::try_new(n)
                    .map_err(|_| DomainError::InvalidInput("Corrupted DB data: пустое имя".into()))
            })
            .transpose()?;
        
        let contact_method = ContactMethod::try_from(value.contact_method.as_str())?;
        
        let contact_value = ContactValue::try_new(value.contact_value)
            .map_err(|_| DomainError::InvalidInput("Corrupted DB data: пустой контакт".into()))?;

        let message = value
            .message
            .map(|m| {
                Message::try_new(m).map_err(|_| {
                    DomainError::InvalidInput("Corrupted DB data: пустое сообщение".into())
                })
            })
            .transpose()?;

        let source = LeadSource::try_new(value.source).map_err(|_| {
            DomainError::InvalidInput("Corrupted DB data: пустая форма захвата".into())
        })?;

        Ok(Lead::reconstruct(
            LeadId::from_uuid(value.id),
            name,
            contact_method,
            contact_value,
            message,
            source,
            value.created_at,
        ))
    }
}
