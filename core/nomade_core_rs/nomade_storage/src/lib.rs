//! Storage layer for Nomade
//!
//! Provides artifact store interface

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Artifact metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub title: String,
    pub created_at: u64,
    pub modified_at: u64,
    pub content_hash: String,
}

/// Artifact store interface
pub trait ArtifactStore: Send + Sync {
    /// Store an artifact
    fn store(&self, artifact: &Artifact) -> anyhow::Result<()>;
    
    /// Retrieve an artifact
    fn get(&self, id: &str) -> anyhow::Result<Option<Artifact>>;
    
    /// List all artifacts
    fn list(&self) -> anyhow::Result<Vec<Artifact>>;
    
    /// Delete an artifact
    fn delete(&self, id: &str) -> anyhow::Result<()>;
}

/// Simple in-memory artifact store for testing
pub struct InMemoryStore {
    artifacts: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Artifact>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            artifacts: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactStore for InMemoryStore {
    fn store(&self, artifact: &Artifact) -> anyhow::Result<()> {
        let mut artifacts = self.artifacts.lock().unwrap();
        artifacts.insert(artifact.id.clone(), artifact.clone());
        Ok(())
    }
    
    fn get(&self, id: &str) -> anyhow::Result<Option<Artifact>> {
        let artifacts = self.artifacts.lock().unwrap();
        Ok(artifacts.get(id).cloned())
    }
    
    fn list(&self) -> anyhow::Result<Vec<Artifact>> {
        let artifacts = self.artifacts.lock().unwrap();
        Ok(artifacts.values().cloned().collect())
    }
    
    fn delete(&self, id: &str) -> anyhow::Result<()> {
        let mut artifacts = self.artifacts.lock().unwrap();
        artifacts.remove(id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_in_memory_store() {
        let store = InMemoryStore::new();
        
        let artifact = Artifact {
            id: "test-123".into(),
            title: "Test".into(),
            created_at: 0,
            modified_at: 0,
            content_hash: "hash".into(),
        };
        
        store.store(&artifact).unwrap();
        let retrieved = store.get("test-123").unwrap().unwrap();
        assert_eq!(retrieved.title, "Test");
        
        store.delete("test-123").unwrap();
        assert!(store.get("test-123").unwrap().is_none());
    }
}

