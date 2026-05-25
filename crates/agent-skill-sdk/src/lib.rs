//! Agent Skill SDK — the public surface third parties depend on to write skills
//! that plug into the Logos Autonomous AI Agent Module.
//!
//! The agent runtime discovers `Skill` implementations via the [`register_skill!`]
//! macro and dispatches owner/A2A requests to them. Skills should be small,
//! composable, and side-effect-isolated: a panic or error in one skill must not
//! affect the agent or other skills (the runtime enforces this).
//!
//! See `/spec/skill-interface.md` in the agent-module repo for the full
//! contract and naming conventions (`<category>.<verb>`).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export the `#[async_trait]` proc-macro so SDK consumers only need one
// dependency. Skill implementors write `use agent_skill_sdk::async_trait;`.
pub use ::async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum SkillError {
    #[error("invalid params: {0}")]
    InvalidParams(String),
    #[error("skill failed: {0}")]
    Execution(String),
    #[error("owner approval required (above spending threshold)")]
    ApprovalRequired,
    #[error("owner approval denied")]
    ApprovalDenied,
    #[error("timeout after {0}ms")]
    Timeout(u64),
    #[error("upstream module error: {0}")]
    Upstream(String),
}

pub type SkillResult<T = serde_json::Value> = Result<T, SkillError>;

/// Declarative parameter schema for a single skill argument.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSpec {
    pub name: String,
    pub kind: ParamKind,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamKind {
    String,
    Bytes,
    Integer,
    Bool,
    Address, // LEZ NPK or transparent account ID
    TokenAmount,
    Path,
    Object, // arbitrary JSON
}

/// Self-describing skill metadata included in the agent's A2A AgentCard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    /// Dotted name, e.g. "storage.upload" or "wallet.send".
    pub name: String,
    pub description: String,
    pub params: Vec<ParamSpec>,
    /// JSON Schema describing the output shape.
    pub output_schema: serde_json::Value,
    /// LEZ token price if this skill is offered to other agents over A2A.
    /// `None` ⇒ skill is owner-only and not advertised.
    pub price_lez: Option<TokenAmount>,
    /// True if the skill may spend tokens or modify on-chain state, which
    /// makes it subject to the agent's spending policy.
    pub may_spend: bool,
}

/// Fixed-precision token amount (smallest unit).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenAmount(pub u128);

/// Runtime context passed to every skill invocation. Provides handles to other
/// Logos modules (wallet, storage, messaging) via the LogosAPI bridge.
///
/// The trait is intentionally narrow so a SkillContext can be mocked in tests
/// without depending on the agent-core crate.
#[async_trait]
pub trait SkillContext: Send + Sync {
    /// Wallet operations gated by the spending policy.
    fn wallet(&self) -> &dyn WalletHandle;

    /// Storage operations (Logos Storage / Codex).
    fn storage(&self) -> &dyn StorageHandle;

    /// Messaging operations (Logos Messaging / Waku).
    fn messaging(&self) -> &dyn MessagingHandle;

    /// The agent's own identity.
    fn identity(&self) -> &AgentIdentity;

    /// LLM inference (pluggable backend).
    async fn infer(&self, prompt: &str) -> SkillResult<String>;

    /// Persistent K/V scoped to this skill's namespace.
    async fn state_get(&self, key: &str) -> SkillResult<Option<Vec<u8>>>;
    async fn state_put(&self, key: &str, value: Vec<u8>) -> SkillResult<()>;

    /// Request out-of-band owner approval. Returns Ok(true) if approved.
    /// Skills should NOT call this directly for spending decisions — the
    /// runtime intercepts wallet methods and handles approval transparently.
    async fn request_owner_approval(&self, summary: &str) -> SkillResult<bool>;
}

#[async_trait]
pub trait WalletHandle: Send + Sync {
    async fn balance(&self) -> SkillResult<TokenAmount>;
    async fn send(&self, to: &str, amount: TokenAmount) -> SkillResult<TxReceipt>;
    async fn history(&self, limit: usize) -> SkillResult<Vec<TxRecord>>;
    async fn call_program(
        &self,
        program_id: &str,
        instruction: &str,
        params: serde_json::Value,
    ) -> SkillResult<TxReceipt>;
    async fn query_program(
        &self,
        program_id: &str,
        params: serde_json::Value,
    ) -> SkillResult<serde_json::Value>;
}

#[async_trait]
pub trait StorageHandle: Send + Sync {
    async fn upload(&self, path: &str, label: Option<&str>) -> SkillResult<ContentAddress>;
    async fn download(&self, addr: &ContentAddress, to_path: &str) -> SkillResult<()>;
    async fn list(&self) -> SkillResult<Vec<StoredFile>>;
    async fn share(&self, addr: &ContentAddress, recipient: &str) -> SkillResult<()>;
}

#[async_trait]
pub trait MessagingHandle: Send + Sync {
    async fn send(&self, recipient: &str, body: &[u8]) -> SkillResult<MessageId>;
    async fn join(&self, group_id: &str) -> SkillResult<()>;
    async fn create_group(&self, members: &[String]) -> SkillResult<GroupId>;
    async fn subscribe(&self, topic: &str) -> SkillResult<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    /// Agent's NPK (shielded address, hex-encoded).
    pub npk: String,
    /// Public name used in AgentCard `name`.
    pub name: String,
    /// Owner's NPK — only the owner can change spending policy.
    pub owner_npk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxReceipt {
    pub tx_hash: String,
    pub block_height: u64,
    pub timestamp_unix: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxRecord {
    pub tx_hash: String,
    pub direction: TxDirection,
    pub counterparty: String,
    pub amount: TokenAmount,
    pub timestamp_unix: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TxDirection {
    In,
    Out,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContentAddress(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredFile {
    pub address: ContentAddress,
    pub label: Option<String>,
    pub size_bytes: u64,
    pub uploaded_unix: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupId(pub String);

/// The trait every skill implements. Keep `invoke` panic-free; the runtime
/// will catch panics but a panic produces a less actionable error than a
/// returned `SkillError`.
#[async_trait]
pub trait Skill: Send + Sync {
    /// Stable name in `<category>.<verb>` form, e.g. `"storage.upload"`.
    fn name(&self) -> &str;

    /// Self-describing manifest published in the A2A AgentCard.
    fn manifest(&self) -> SkillManifest;

    /// Execute the skill. `params` matches the manifest schema.
    /// `ctx` provides access to wallet, storage, messaging, and inference.
    async fn invoke(
        &self,
        params: serde_json::Value,
        ctx: &dyn SkillContext,
    ) -> SkillResult<serde_json::Value>;
}

/// Collection used by the runtime to look up skills by name.
#[derive(Default)]
pub struct SkillRegistry {
    skills: HashMap<String, Box<dyn Skill>>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, skill: Box<dyn Skill>) {
        self.skills.insert(skill.name().to_string(), skill);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Skill> {
        self.skills.get(name).map(|b| b.as_ref())
    }

    pub fn list(&self) -> Vec<SkillManifest> {
        self.skills.values().map(|s| s.manifest()).collect()
    }

    pub fn len(&self) -> usize {
        self.skills.len()
    }

    pub fn is_empty(&self) -> bool {
        self.skills.is_empty()
    }
}

/// Macro to declare a skill module that the agent runtime can discover at
/// compile time. Third-party skill crates expose a `register_skills` function
/// that the agent links against.
///
/// ```ignore
/// use agent_skill_sdk::{register_skills, SkillRegistry};
///
/// #[register_skills]
/// pub fn skills() -> Vec<Box<dyn Skill>> {
///     vec![Box::new(MyTranslateSkill::new())]
/// }
/// ```
#[macro_export]
macro_rules! register_skill {
    ($skill:expr) => {
        #[doc(hidden)]
        pub fn __agent_register_skill() -> Box<dyn $crate::Skill> {
            Box::new($skill)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_starts_empty() {
        let r = SkillRegistry::new();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
    }
}
