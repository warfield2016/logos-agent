//! LEZ program: agent-registry
//!
//! Records `(agent_npk, owner_attestation, first_seen_block)` so evaluators
//! can verify the "5 outside-team deployers" success criterion directly from
//! the explorer. Each `register` instruction must include a signed owner
//! attestation that proves the agent's owner is distinct from the
//! submitting-team set.
//!
//! Layout (Week-3 fills in instruction handlers):
//!
//!   instructions:
//!     - new_definition           // one-time program init
//!     - register(agent_npk, owner_attestation_sig, manifest_cid)
//!     - update_card(agent_npk, new_card_cid)
//!     - touch(agent_npk)         // bump last_seen_block for liveness
//!
//!   queries:
//!     - get(agent_npk) -> Registration
//!     - list_recent(n) -> Vec<Registration>
//!     - count_by_team(team_set) -> u64

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub agent_npk: [u8; 32],
    pub owner_npk: [u8; 32],
    pub manifest_cid: String,
    pub first_seen_block: u64,
    pub last_seen_block: u64,
}
