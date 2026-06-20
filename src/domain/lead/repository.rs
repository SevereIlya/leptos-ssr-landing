use super::entity::Lead;
use super::value_objects::LeadId;
use crate::domain::error::*;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynLeadRepository = Arc<dyn LeadRepository + Send + Sync>;

#[async_trait]
pub trait LeadRepository: Send + Sync {
    async fn create(&self, lead: &Lead) -> DomainResult<LeadId>;
    async fn find_by_uuid(&self, id: LeadId) -> DomainResult<Option<Lead>>;
}
