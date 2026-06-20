use crate::domain::events::*;
use async_trait::async_trait;

pub struct CompositeEventPublisher {
    adapters: Vec<DynEventPublisher>,
}

impl CompositeEventPublisher {
    pub fn new(adapters: Vec<DynEventPublisher>) -> Self {
        Self { adapters }
    }
}

#[async_trait]
impl EventPublisher for CompositeEventPublisher {
    async fn publish(&self, event: DomainEvent) {
        for adapter in &self.adapters {
            let a = adapter.clone();
            let e = event.clone();
            tokio::spawn(async move {
                a.publish(e).await;
            });
        }
    }
}