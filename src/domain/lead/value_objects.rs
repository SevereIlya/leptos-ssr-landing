use crate::domain::error::*;
use nutype::nutype;
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeadId(Uuid);

#[nutype(
    validate(not_empty),
    sanitize(trim),
    derive(Debug, Clone, PartialEq, Eq, Hash, AsRef)
)]
pub struct Name(String);

#[nutype(
    validate(not_empty),
    sanitize(trim),
    derive(Debug, Clone, PartialEq, Eq, Hash, AsRef)
)]
pub struct Message(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContactMethod {
    Email,
    Phone,
    Telegram,
    Vk,
    WhatsApp,
}

#[nutype(
    validate(not_empty),
    sanitize(trim),
    derive(Debug, Clone, PartialEq, Eq, Hash, AsRef)
)]
pub struct ContactValue(String);

#[nutype(
    validate(not_empty),
    sanitize(trim),
    derive(Debug, Clone, PartialEq, Eq, Hash, AsRef)
)]
pub struct LeadSource(String);

// ============================================================================================== //

impl LeadId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub const fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for LeadId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for LeadId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for ContactMethod {
    type Error = DomainError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "email" => Ok(Self::Email),
            "phone" => Ok(Self::Phone),
            "telegram" => Ok(Self::Telegram),
            "vk" => Ok(Self::Vk),
            "whatsapp" => Ok(Self::WhatsApp),
            _ => Err(DomainError::InvalidContactMethod(value.to_string())),
        }
    }
}

impl Display for ContactMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Telegram => "telegram",
            Self::Vk => "vk",
            Self::WhatsApp => "whatsapp",
        };
        write!(f, "{s}")
    }
}
