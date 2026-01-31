//! Event stream system for Nomade
//!
//! Provides pub/sub event system for real-time updates

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    ArtifactCreated { id: String },
    ArtifactUpdated { id: String },
    ArtifactDeleted { id: String },
    DeviceConnected { device_id: String },
    DeviceDisconnected { device_id: String },
    SyncStarted,
    SyncCompleted { artifacts_synced: usize },
}

/// Event stream for subscribing to events
pub struct EventStream {
    tx: broadcast::Sender<Event>,
}

impl EventStream {
    /// Create new event stream
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }

    /// Publish an event
    pub fn publish(&self, event: Event) {
        let _ = self.tx.send(event); // Ignore if no subscribers
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }
}

impl Default for EventStream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_stream() {
        let stream = EventStream::new();
        let mut rx = stream.subscribe();

        stream.publish(Event::ArtifactCreated {
            id: "test-123".into(),
        });

        let event = rx.recv().await.unwrap();
        match event {
            Event::ArtifactCreated { id } => assert_eq!(id, "test-123"),
            _ => panic!("Wrong event type"),
        }
    }
}
