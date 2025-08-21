use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

// structuring memory in agent system.
// a well-designed memory system includes at least three layers:
//    - a working memory that holds temporary data relevant to the current session of the task.
//    - a long-term memory where structured data (e.g., past tasks, goals or user input) is persist.
//    - an index or recall mechanism to retrieve relevant entries based on semantic or exact-match queries.

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
    pub tags: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

// building the memory interface
#[async_trait]
pub trait MemoryStore: Sync + Send {
    async fn write(&self, entry: MemoryEntry) -> bool;
    async fn read_by_key(&self, key: &str) -> Option<MemoryEntry>;
    async fn read_recent(&self, limit: usize) -> Vec<MemoryEntry>;
    async fn search_by_tag(&self, tag: &str) -> Vec<MemoryEntry>;
}

// implementing an in-memory store
pub type SharedMemory = Arc<RwLock<HashMap<String, MemoryEntry>>>;

pub struct InMemoryStore {
    memory: SharedMemory,
}

#[async_trait]
impl MemoryStore for InMemoryStore {
    async fn write(&self, entry: MemoryEntry) -> bool {
        let mut store = self.memory.write().await;
        store.insert(entry.key.clone(), entry);
        true
    }
    async fn read_by_key(&self, key: &str) -> Option<MemoryEntry> {
        let store = self.memory.read().await;
        store.get(key).cloned()
    }
    async fn read_recent(&self, limit: usize) -> Vec<MemoryEntry> {
        let store = self.memory.read().await;
        let mut entries: Vec<_> = store.values().cloned().collect();
        entries.sort_by_key(|e| std::cmp::Reverse(e.timestamp));
        entries.truncate(limit);
        entries
    }
    async fn search_by_tag(&self, tag: &str) -> Vec<MemoryEntry> {
        let store = self.memory.read().await;
        store
            .values()
            .filter(|entry| entry.tags.contains(&tag.to_string()))
            .cloned()
            .collect()
    }
}

// integrating memory into agent behavior
// for writing:
/*
let entry = MemoryEntry {
    key: format!("summary_{}", task_id),
    value: summary_text,
    tags: vec!["summary".into(), task_id.clone()],
    timestamp: Utc::now(),
};

memory_store.write(entry).await;
*/

// for reading:
/*
if let Some(mem) = memory_store.read_be_key("summary_task_001").await {
    println!("Recalled memory: {}", mem.value);
}
*/

// the existing persisten storage can be extended with embedded database like SQLite.