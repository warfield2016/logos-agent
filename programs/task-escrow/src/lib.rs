//! LEZ program: task-escrow
//!
//! Used when an A2A task is settled via `Settlement::Escrow` rather than
//! direct shielded/transparent transfer. Funds are locked at task acceptance
//! and released atomically on provider's completion attestation, or refunded
//! on cancellation/timeout.
//!
//! Layout (Week-3 fills in instruction handlers):
//!
//!   instructions:
//!     - new_definition
//!     - open(task_id, provider, amount)       // buyer locks funds
//!     - complete(task_id, completion_proof)   // provider claims
//!     - cancel(task_id)                       // buyer reclaims
//!     - dispute(task_id, evidence_cid)        // either party
//!     - resolve(task_id, ruling)              // arbiter outcome
//!
//!   queries:
//!     - get(task_id) -> Escrow

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Escrow {
    pub task_id: String,
    pub buyer_npk: [u8; 32],
    pub provider_npk: [u8; 32],
    pub amount_smallest: u128,
    pub state: EscrowState,
    pub opened_block: u64,
    pub timeout_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscrowState {
    Open,
    Completed,
    Canceled,
    Disputed,
    Resolved,
}
