//! LEZ Payment Extension — A2A extension URI `https://logos.co/spec/a2a-extensions/lez-payment/0.1`.
//!
//! Flow:
//!   1. Client agent reads provider's AgentCard, sees `price_lez` for skill.
//!   2. Client sends `Payment::Commit { task_id, amount, tx_hash }` after
//!      shielded transfer is confirmed on LEZ.
//!   3. Provider verifies the tx on-chain, then begins execution
//!      (status → WORKING).
//!   4. On task completion, provider emits `Payment::Receipt { tx_hash }`.
//!   5. On cancellation before completion, provider emits a refund
//!      `Payment::Refund { tx_hash, refund_tx_hash }`.
//!
//! Settlement options:
//!   - "shielded": full ZK-private transfer (default).
//!   - "transparent": public LEZ transfer (lower latency, less privacy).
//!   - "escrow": uses programs/task-escrow for atomic-on-completion settlement.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PaymentEnvelope {
    Commit {
        amount_smallest: u128,
        token: String,
        tx_hash: String,
        settlement: Settlement,
    },
    Receipt {
        confirmed_tx_hash: String,
    },
    Refund {
        original_tx_hash: String,
        refund_tx_hash: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Settlement {
    Shielded,
    Transparent,
    Escrow,
}
