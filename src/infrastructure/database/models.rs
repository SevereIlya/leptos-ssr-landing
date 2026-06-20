use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct LeadRow {
    pub id: Uuid,
    pub name: Option<String>,
    pub contact_method: String,
    pub contact_value: String,
    pub message: Option<String>,
    pub source: String,
    pub consent_given: bool,
    pub created_at: DateTime<Utc>,
}
