//! A2A Task — the unit of work between two agents.
//!
//! Lifecycle (matches A2A spec §4.1.3):
//!   SUBMITTED → WORKING → COMPLETED | FAILED | CANCELED | REJECTED
//!                       ↘ INPUT_REQUIRED ↗ (interrupted, resumable)
//!                       ↘ AUTH_REQUIRED  ↗

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    Submitted,
    Working,
    InputRequired,
    AuthRequired,
    Completed,
    Failed,
    Canceled,
    Rejected,
}

impl TaskStatus {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Canceled | Self::Rejected
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(rename = "contextId")]
    pub context_id: Option<String>,
    pub status: TaskStatus,
    pub skill: String,
    pub params: serde_json::Value,
    /// Requester NPK hex.
    pub from: String,
    /// Provider NPK hex (= our agent).
    pub to: String,
    pub created_unix: i64,
    pub updated_unix: i64,
    /// Monotonic per-task sequence — required by the Logos Messaging binding
    /// because Waku is not ordered transport.
    pub seq: u64,
    pub artifacts: Vec<Artifact>,
    pub error: Option<TaskError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub mime_type: String,
    pub data: ArtifactData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactData {
    Inline { bytes_b64: String },
    Storage { content_address: String },
    Url { url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskError {
    pub code: i32,
    pub message: String,
}

/// In-memory task store. Tasks survive process restart if persisted; Week-2
/// adds sled-backed persistence behind the same surface.
pub struct TaskStore {
    tasks: Arc<RwLock<HashMap<String, Task>>>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, task: Task) {
        self.tasks.write().insert(task.id.clone(), task);
    }

    pub fn get(&self, id: &str) -> Option<Task> {
        self.tasks.read().get(id).cloned()
    }

    pub fn update_status(&self, id: &str, status: TaskStatus) -> bool {
        let mut guard = self.tasks.write();
        if let Some(t) = guard.get_mut(id) {
            t.status = status;
            t.updated_unix = chrono::Utc::now().timestamp();
            t.seq += 1;
            true
        } else {
            false
        }
    }

    pub fn active_count(&self) -> usize {
        self.tasks
            .read()
            .values()
            .filter(|t| !t.status.is_terminal())
            .count()
    }
}

impl Default for TaskStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_states_classified_correctly() {
        assert!(TaskStatus::Completed.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(TaskStatus::Canceled.is_terminal());
        assert!(TaskStatus::Rejected.is_terminal());
        assert!(!TaskStatus::Submitted.is_terminal());
        assert!(!TaskStatus::Working.is_terminal());
        assert!(!TaskStatus::InputRequired.is_terminal());
        assert!(!TaskStatus::AuthRequired.is_terminal());
    }
}
