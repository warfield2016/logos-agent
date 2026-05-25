//! Skill dispatcher — constructs a `SkillContext` per invocation, wiring the
//! agent's modules into the skill execution environment.
//!
//! Lives in its own module so the runtime can be tested without spinning up the
//! full FFI/Tokio stack.

use crate::runtime::Agent;
use agent_skill_sdk::{
    async_trait, AgentIdentity, ContentAddress, GroupId, MessageId, MessagingHandle, SkillContext,
    SkillError, SkillResult, StorageHandle, StoredFile, TokenAmount, TxReceipt, TxRecord,
    WalletHandle,
};

#[derive(Default)]
pub struct Dispatcher;

impl Dispatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn context_for<'a>(&self, agent: &'a Agent) -> Box<dyn SkillContext + 'a> {
        Box::new(RuntimeContext {
            agent,
            identity: AgentIdentity {
                npk: agent.identity.npk_hex(),
                name: "logos-agent".into(),
                owner_npk: agent.config.owner_npk.clone(),
            },
        })
    }
}

/// `SkillContext` impl backed by the live `Agent`. All handles delegate to
/// stub adapters in Week-1; Week-2 swaps in wallet-ffi / storage / messaging.
struct RuntimeContext<'a> {
    #[allow(dead_code)]
    agent: &'a Agent,
    identity: AgentIdentity,
}

#[async_trait]
impl SkillContext for RuntimeContext<'_> {
    fn wallet(&self) -> &dyn WalletHandle {
        &StubWallet
    }
    fn storage(&self) -> &dyn StorageHandle {
        &StubStorage
    }
    fn messaging(&self) -> &dyn MessagingHandle {
        &StubMessaging
    }
    fn identity(&self) -> &AgentIdentity {
        &self.identity
    }
    async fn infer(&self, _prompt: &str) -> SkillResult<String> {
        // TODO Week-2: route through llm::Backend trait.
        Err(SkillError::Execution("LLM backend not configured".into()))
    }
    async fn state_get(&self, _key: &str) -> SkillResult<Option<Vec<u8>>> {
        Ok(None)
    }
    async fn state_put(&self, _key: &str, _value: Vec<u8>) -> SkillResult<()> {
        Ok(())
    }
    async fn request_owner_approval(&self, _summary: &str) -> SkillResult<bool> {
        // TODO Week-2: bridge to OwnerChannel + await reply.
        Err(SkillError::ApprovalRequired)
    }
}

// ─── Week-1 stub adapters ────────────────────────────────────────────────
//
// These return Err(Upstream) for now. Week-2 replaces them with real
// implementations that call wallet-ffi (Rust), storage_module (via LogosAPI
// IPC), and delivery_module (via LogosAPI IPC).

struct StubWallet;
#[async_trait]
impl WalletHandle for StubWallet {
    async fn balance(&self) -> SkillResult<TokenAmount> {
        Err(SkillError::Upstream("wallet-ffi not yet wired".into()))
    }
    async fn send(&self, _to: &str, _amount: TokenAmount) -> SkillResult<TxReceipt> {
        Err(SkillError::Upstream("wallet-ffi not yet wired".into()))
    }
    async fn history(&self, _limit: usize) -> SkillResult<Vec<TxRecord>> {
        Err(SkillError::Upstream("wallet-ffi not yet wired".into()))
    }
    async fn call_program(
        &self,
        _program_id: &str,
        _instruction: &str,
        _params: serde_json::Value,
    ) -> SkillResult<TxReceipt> {
        Err(SkillError::Upstream("wallet-ffi not yet wired".into()))
    }
    async fn query_program(
        &self,
        _program_id: &str,
        _params: serde_json::Value,
    ) -> SkillResult<serde_json::Value> {
        Err(SkillError::Upstream("wallet-ffi not yet wired".into()))
    }
}

struct StubStorage;
#[async_trait]
impl StorageHandle for StubStorage {
    async fn upload(&self, _path: &str, _label: Option<&str>) -> SkillResult<ContentAddress> {
        Err(SkillError::Upstream("storage_module not yet wired".into()))
    }
    async fn download(&self, _addr: &ContentAddress, _to_path: &str) -> SkillResult<()> {
        Err(SkillError::Upstream("storage_module not yet wired".into()))
    }
    async fn list(&self) -> SkillResult<Vec<StoredFile>> {
        Ok(vec![])
    }
    async fn share(&self, _addr: &ContentAddress, _recipient: &str) -> SkillResult<()> {
        Err(SkillError::Upstream("storage_module not yet wired".into()))
    }
}

struct StubMessaging;
#[async_trait]
impl MessagingHandle for StubMessaging {
    async fn send(&self, _recipient: &str, _body: &[u8]) -> SkillResult<MessageId> {
        Err(SkillError::Upstream("delivery_module not yet wired".into()))
    }
    async fn join(&self, _group_id: &str) -> SkillResult<()> {
        Err(SkillError::Upstream("delivery_module not yet wired".into()))
    }
    async fn create_group(&self, _members: &[String]) -> SkillResult<GroupId> {
        Err(SkillError::Upstream("delivery_module not yet wired".into()))
    }
    async fn subscribe(&self, _topic: &str) -> SkillResult<()> {
        Err(SkillError::Upstream("delivery_module not yet wired".into()))
    }
}
