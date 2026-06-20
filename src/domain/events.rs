use crate::domain::lead::events::LeadEvent;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum DomainEvent {
    Lead(LeadEvent),
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: DomainEvent);
}

pub type DynEventPublisher = Arc<dyn EventPublisher>;