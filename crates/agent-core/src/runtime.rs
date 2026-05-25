//! Long-lived agent runtime — owns the Tokio runtime, skill registry, identity,
//! spending policy, A2A transport, and owner channel.

use crate::a2a::TaskStore;
use crate::dispatcher::Dispatcher;
use crate::identity::Identity;
use crate::owner_channel::OwnerChannel;
use crate::spending_policy::SpendingPolicy;
use agent_skill_sdk::{SkillManifest, SkillRegistry};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::Arc;

static GLOBAL_TOKIO: OnceCell<tokio::runtime::Runtime> = OnceCell::new();

/// Process-global handles created once on `agent_init_runtime`.
pub struct Runtime;

impl Runtime {
    /// Idempotent: builds the global Tokio runtime + tracing subscriber.
    pub fn init_global() -> anyhow::Result<()> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "agent_core=info".into()),
            )
            .try_init();
        GLOBAL_TOKIO.get_or_try_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_name("agent-rt")
                .build()
        })?;
        Ok(())
    }

    pub fn tokio() -> &'static tokio::runtime::Runtime {
        GLOBAL_TOKIO
            .get()
            .expect("Runtime::init_global() must run first")
    }

    /// Construct an agent from a config file. Returns an owned `Agent` whose
    /// lifetime is managed by the FFI caller.
    pub fn create_agent(config_path: &str) -> anyhow::Result<Agent> {
        let cfg = Self::load_config(config_path)?;
        let identity = Identity::load_or_create(&cfg.identity_path)?;
        let spending_policy = SpendingPolicy::from_config(&cfg.spending);
        let owner_channel = OwnerChannel::new(identity.clone(), cfg.owner_npk.clone());
        let mut registry = SkillRegistry::new();
        crate::skills::register_default_skills(&mut registry);

        Ok(Agent {
            identity,
            registry: Arc::new(registry),
            dispatcher: Dispatcher::new(),
            spending_policy: Arc::new(Mutex::new(spending_policy)),
            owner_channel: Arc::new(owner_channel),
            task_store: Arc::new(TaskStore::new()),
            config: cfg,
        })
    }

    fn load_config(path: &str) -> anyhow::Result<AgentConfig> {
        // Stub. Week-1 will replace with a real TOML parser.
        Ok(AgentConfig {
            identity_path: format!("{}/identity.json", default_data_dir()),
            owner_npk: "<owner-npk-from-config>".into(),
            spending: SpendingConfig::default(),
            _config_path: path.to_string(),
        })
    }
}

fn default_data_dir() -> String {
    std::env::var("LOGOS_AGENT_DATA_DIR").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        format!("{home}/.logos-agent")
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct AgentConfig {
    pub identity_path: String,
    pub owner_npk: String,
    pub spending: SpendingConfig,
    _config_path: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SpendingConfig {
    pub per_tx_lez: u128,
    pub per_day_lez: u128,
    pub income_cap_lez: Option<u128>,
}

pub struct Agent {
    pub identity: Identity,
    pub registry: Arc<SkillRegistry>,
    pub dispatcher: Dispatcher,
    pub spending_policy: Arc<Mutex<SpendingPolicy>>,
    pub owner_channel: Arc<OwnerChannel>,
    pub task_store: Arc<TaskStore>,
    pub config: AgentConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum InvokeError {
    #[error("skill not found")]
    NotFound,
    #[error("owner approval required")]
    ApprovalRequired,
    #[error("owner approval denied")]
    ApprovalDenied,
    #[error("invalid params: {0}")]
    InvalidParams(String),
    #[error("skill error: {0}")]
    Skill(String),
}

impl Agent {
    /// Synchronous wrapper used by the FFI layer. Spawns the async invoke onto
    /// the global Tokio runtime and blocks until complete.
    pub fn invoke_blocking(
        &self,
        skill_name: &str,
        params_json: &str,
    ) -> Result<String, InvokeError> {
        let skill = self.registry.get(skill_name).ok_or(InvokeError::NotFound)?;
        let params: serde_json::Value = serde_json::from_str(params_json)
            .map_err(|e| InvokeError::InvalidParams(e.to_string()))?;

        let ctx = self.dispatcher.context_for(self);

        let result = Runtime::tokio().block_on(skill.invoke(params, ctx.as_ref()));

        match result {
            Ok(v) => Ok(serde_json::to_string(&v).unwrap_or_else(|_| "null".into())),
            Err(agent_skill_sdk::SkillError::ApprovalRequired) => {
                Err(InvokeError::ApprovalRequired)
            }
            Err(agent_skill_sdk::SkillError::ApprovalDenied) => Err(InvokeError::ApprovalDenied),
            Err(e) => Err(InvokeError::Skill(e.to_string())),
        }
    }

    pub fn list_skills(&self) -> Vec<SkillManifest> {
        self.registry.list()
    }

    pub fn status_blocking(&self) -> AgentStatus {
        AgentStatus {
            version: crate::VERSION.into(),
            npk: self.identity.npk_hex(),
            owner_npk: self.config.owner_npk.clone(),
            skill_count: self.registry.len(),
            pending_approvals: self.owner_channel.pending_approval_count(),
            active_tasks: self.task_store.active_count(),
        }
    }

    pub fn pending_approval_count(&self) -> usize {
        self.owner_channel.pending_approval_count()
    }
}

#[derive(Debug, Serialize)]
pub struct AgentStatus {
    pub version: String,
    pub npk: String,
    pub owner_npk: String,
    pub skill_count: usize,
    pub pending_approvals: usize,
    pub active_tasks: usize,
}
