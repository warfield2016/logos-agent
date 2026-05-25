//! A2A transport binding for Logos Messaging.
//!
//! Implements the functional-equivalence contract from A2A spec §12 over the
//! Logos Messaging (Waku) pub/sub primitive:
//!
//! - `SendMessage`              → publish to `/logos-a2a/0.1/tasks/<recipient>/inbox`
//! - `SendStreamingMessage`     → same; updates arrive on `task_status` topic
//! - `GetTask`                  → local TaskStore lookup (or `RequestTask` over inbox)
//! - `CancelTask`               → publish CancelEnvelope to task_status topic
//! - `SubscribeToTask`          → subscribe to `task_status` + `task_artifacts`
//! - `ListTasks`                → local TaskStore filter
//!
//! Wire format: A2A Protocol Buffers, raw bytes. JSON envelopes used only for
//! debugging in Week-1.
//!
//! Ordering: because Waku is not ordered, each envelope carries a monotonic
//! `seq` field per task. Clients reorder on receive.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Envelope {
    /// Client → provider: please run a skill.
    TaskRequest {
        task_id: String,
        from: String,
        skill: String,
        params: serde_json::Value,
        context_id: Option<String>,
        /// Hex sig over (task_id ‖ skill ‖ canonical_params).
        signature: String,
    },
    /// Provider → client: task state changed.
    TaskStatus {
        task_id: String,
        seq: u64,
        status: super::task::TaskStatus,
        message: Option<String>,
    },
    /// Provider → client: artifact produced (inline / Storage CID / URL).
    TaskArtifact {
        task_id: String,
        seq: u64,
        artifact: super::task::Artifact,
    },
    /// Client → provider: cancel running task.
    Cancel { task_id: String, signature: String },
    /// Either side: payment commit/receipt (see `payment.rs`).
    Payment {
        task_id: String,
        payload: super::payment::PaymentEnvelope,
    },
}

// Real publishing happens through the delivery_module via LogosAPI IPC.
// Week-2 fills this in; for now we just provide the envelope schema so the
// spec doc is grounded in real types.
