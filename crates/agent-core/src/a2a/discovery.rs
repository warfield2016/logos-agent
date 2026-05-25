//! Agent discovery — publish & fetch AgentCards over the Logos Messaging
//! discovery topic.

use super::agent_card::AgentCard;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardAnnouncement {
    pub card: AgentCard,
    pub announced_unix: i64,
    /// Time-to-live in seconds — receivers should treat the card as stale
    /// after this duration unless refreshed.
    pub ttl_secs: u32,
}

impl CardAnnouncement {
    pub fn new(card: AgentCard, ttl_secs: u32) -> Self {
        Self {
            card,
            announced_unix: chrono::Utc::now().timestamp(),
            ttl_secs,
        }
    }
}
