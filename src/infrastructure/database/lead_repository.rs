use super::*;
use crate::domain::error::*;
use crate::domain::lead::entity::*;
use crate::domain::lead::repository::*;
use crate::domain::lead::value_objects::*;
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;

pub struct SqlxLeadRepository {
    pool: PgPool,
}

impl SqlxLeadRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LeadRepository for SqlxLeadRepository {
    #[instrument(skip(self, lead), fields(lead_id = %lead.id))]
    async fn create(&self, lead: &Lead) -> DomainResult<LeadId> {
        let query = sqlx::query!(
            r#"
            INSERT INTO leads (id, name, contact_method, contact_value, message, source, consent_given, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            lead.id.as_uuid(),
            lead.name.as_ref().map(|n| n.as_ref().to_string()),
            lead.contact_method.to_string(),
            lead.contact_value.as_ref().to_string(),
            lead.message.as_ref().map(|m| m.as_ref().to_string()),
            lead.source.as_ref().to_string(),
            true, // consent_given проверяется в домене, поэтому тут всегда true
            lead.created_at,
        );

        let result = query.execute(&self.pool).await;

        match result {
            Ok(_) => Ok(lead.id),
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
                Err(DomainError::LeadAlreadyExists)
            }
            Err(e) => Err(DomainError::SystemFailure(e.to_string())),
        }
    }

    #[instrument(skip(self))]
    async fn find_by_uuid(&self, id: LeadId) -> DomainResult<Option<Lead>> {
        let query = sqlx::query_as!(
            LeadRow,
            r#"
            SELECT *
            FROM leads
            WHERE id = $1
            "#,
            id.as_uuid(),
        );

        let result = query.fetch_optional(&self.pool).await;
        let row: Option<LeadRow> = result.map_err(|e| DomainError::SystemFailure(e.to_string()))?;
        let lead: Option<Lead> = row.map(TryInto::try_into).transpose()?;

        Ok(lead)
    }
}
