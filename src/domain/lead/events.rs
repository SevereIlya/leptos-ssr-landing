use super::value_objects::*;

#[derive(Debug, Clone)]
pub enum LeadEvent {
    Created {
        id: LeadId,
        name: Option<Name>,
        contact_method: ContactMethod,
        contact_value: ContactValue,
        message: Option<Message>,
        source: LeadSource,
    }
}