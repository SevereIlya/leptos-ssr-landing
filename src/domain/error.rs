use thiserror::Error;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("No contact info provided")]
    NoContactInfo,
    #[error("Lead already exists")]
    LeadAlreadyExists,
    #[error("Lead not found")]
    LeadNotFound,
    
    #[error("Invalid contact method: {0}")]
    InvalidContactMethod(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("System failure: {0}")]
    SystemFailure(String),
}

impl DomainError {
    pub fn message_error(&self) -> String {
        match self {
            Self::NoContactInfo => "Введи хотя бы email или номер телефона".to_owned(),
            Self::LeadAlreadyExists => "Ты уже оставлял заявку. Имей терпение".to_owned(),
            Self::LeadNotFound => "Такого чувака в базе нет".to_owned(),
            Self::InvalidContactMethod(_) => "Неизвестный способ связи".to_owned(),
            Self::InvalidInput(msg) => msg.clone(),
            Self::SystemFailure(_) => "Попробуй позже".to_owned(),
        }
    }
}