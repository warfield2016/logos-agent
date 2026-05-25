//! A2A coordination — Agent Card publish/discover + Task lifecycle, bound to
//! Logos Messaging as transport and LEZ as payment layer.
//!
//! Formal spec: `/spec/a2a-logos-messaging-binding.md`
//! Payment ext: `/spec/lez-payment-extension.md`
//!
//! Binding URI: `https://logos.co/spec/a2a-bindings/logos-messaging/0.1`
//! Payment ext URI: `https://logos.co/spec/a2a-extensions/lez-payment/0.1`

pub mod agent_card;
pub mod discovery;
pub mod payment;
pub mod task;
pub mod transport;

pub use agent_card::AgentCard;
pub use task::{Task, TaskStatus, TaskStore};

/// Canonical URI for the A2A-over-Logos-Messaging transport binding.
pub const BINDING_URI: &str = "https://logos.co/spec/a2a-bindings/logos-messaging/0.1";

/// Canonical URI for the LEZ payment extension.
pub const PAYMENT_EXT_URI: &str = "https://logos.co/spec/a2a-extensions/lez-payment/0.1";

/// A2A protocol version implemented.
pub const A2A_VERSION: &str = "1.0.0";

/// Canonical Logos Messaging topic schema. All topics include the binding
/// version (`0.1`) so future revisions can coexist on the same network.
pub mod topics {
    pub const CARDS_DISCOVERY: &str = "/logos-a2a/0.1/cards";
    pub fn card_by_provider(provider_npk_hex: &str) -> String {
        format!("/logos-a2a/0.1/cards/{provider_npk_hex}")
    }
    pub fn task_inbox(recipient_npk_hex: &str) -> String {
        format!("/logos-a2a/0.1/tasks/{recipient_npk_hex}/inbox")
    }
    pub fn task_status(task_id: &str) -> String {
        format!("/logos-a2a/0.1/tasks/{task_id}/status")
    }
    pub fn task_artifacts(task_id: &str) -> String {
        format!("/logos-a2a/0.1/tasks/{task_id}/artifacts")
    }
    pub fn payment(task_id: &str) -> String {
        format!("/logos-a2a/0.1/payments/{task_id}")
    }
}
