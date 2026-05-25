//! Owner channel — encrypted Logos Messaging topic between the agent and its
//! owner. Approval requests for above-threshold transactions flow over this
//! channel.
//!
//! The channel is keyed by the owner's NPK; messages are end-to-end encrypted
//! using the Messaging module's native primitives (no custom crypto here).

use crate::identity::Identity;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Pending owner-approval requests, keyed by a per-process u64 ticket.
pub struct OwnerChannel {
    _identity: Identity,
    pub owner_npk: String,
    pending: Mutex<VecDeque<PendingApproval>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    pub ticket: u64,
    pub kind: ApprovalKind,
    pub summary: String,
    pub created_unix: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalKind {
    AboveThresholdSpend {
        token: String,
        amount_smallest: u128,
        recipient: String,
    },
    ProgramCall {
        program_id: String,
        instruction: String,
        params: serde_json::Value,
    },
    SkillCall {
        skill: String,
        params: serde_json::Value,
    },
}

impl OwnerChannel {
    pub fn new(identity: Identity, owner_npk: String) -> Self {
        Self {
            _identity: identity,
            owner_npk,
            pending: Mutex::new(VecDeque::new()),
        }
    }

    pub fn pending_approval_count(&self) -> usize {
        self.pending.lock().len()
    }

    /// Queue a request and (eventually) push it over Logos Messaging to the owner.
    /// TODO Week-2: actually publish to /logos-agent/0.1/owner/<owner_npk>/inbox
    pub fn request_approval(&self, kind: ApprovalKind, summary: &str) -> u64 {
        let mut q = self.pending.lock();
        let ticket =
            (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64) ^ (q.len() as u64);
        q.push_back(PendingApproval {
            ticket,
            kind,
            summary: summary.to_string(),
            created_unix: chrono::Utc::now().timestamp(),
        });
        tracing::info!(ticket, "queued owner approval");
        ticket
    }
}
