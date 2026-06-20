use super::events::*;
use super::value_objects::*;
use crate::domain::error::*;
use chrono::{DateTime, Utc};

pub struct Lead {
    pub id: LeadId,
    pub name: Option<Name>,
    pub contact_method: ContactMethod,
    pub contact_value: ContactValue,
    pub message: Option<Message>,
    pub source: LeadSource,
    pub created_at: DateTime<Utc>,
    domain_events: Vec<LeadEvent>,
}

impl Lead {
    pub fn new(
        name: Option<Name>,
        contact_method: ContactMethod,
        contact_value: ContactValue,
        message: Option<Message>,
        source: LeadSource,
        consent_given: bool,
    ) -> DomainResult<Self> {
        if !consent_given {
            return Err(DomainError::InvalidInput(
                "Требуется согласие на обработку ПД".into(),
            ));
        }

        #[cfg(feature = "ssr")]
        {
            use email_address::EmailAddress;
            use std::str::FromStr;

            match contact_method {
                ContactMethod::Email => {
                    if let Err(_) = EmailAddress::from_str(contact_value.as_ref()) {
                        return Err(DomainError::InvalidInput("Некорректный email адрес".into()));
                    }
                }
                ContactMethod::Phone | ContactMethod::WhatsApp => {
                    if let Err(_) = phonenumber::parse(
                        Some(phonenumber::country::Id::RU),
                        contact_value.as_ref(),
                    ) {
                        return Err(DomainError::InvalidInput(
                            "Некорректный номер телефона. Укажите в формате +7...".into(),
                        ));
                    }
                }
                _ => {} // остальные могут быть любыми
            }
        }

        let id = LeadId::new();

        let event = LeadEvent::Created {
            id,
            name: name.clone(),
            contact_method: contact_method.clone(),
            contact_value: contact_value.clone(),
            message: message.clone(),
            source: source.clone(),
        };

        Ok(Self {
            id,
            name,
            contact_method,
            contact_value,
            message,
            source,
            created_at: Utc::now(),
            domain_events: vec![event],
        })
    }

    #[cfg(feature = "ssr")]
    pub(crate) fn reconstruct(
        id: LeadId,
        name: Option<Name>,
        contact_method: ContactMethod,
        contact_value: ContactValue,
        message: Option<Message>,
        source: LeadSource,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            contact_method,
            contact_value,
            message,
            source,
            created_at,
            domain_events: Vec::new(),
        }
    }

    pub fn take_events(&mut self) -> Vec<LeadEvent> {
        std::mem::take(&mut self.domain_events)
    }
}
