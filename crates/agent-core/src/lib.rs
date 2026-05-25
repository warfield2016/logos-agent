//! agent-core — the Logos Autonomous AI Agent runtime.
//!
//! This crate exposes a stable C ABI consumed by the auto-generated Qt plugin
//! shim (`logos-cpp-generator --from-c-header`). All long-lived state lives in
//! `Runtime`, accessed through opaque handles passed across the FFI boundary.
//!
//! Architectural overview:
//!
//! ```text
//!   Logos Core (Basecamp / logoscore)
//!         │   loads .so/.dylib
//!         ▼
//!   [Auto-generated Qt plugin shim]
//!         │   FFI: extern "C" fn agent_*
//!         ▼
//!   agent-core (this crate)
//!         ├── Runtime  ───── Tokio runtime, owns all state
//!         ├── Identity ───── NPK/ISK keypair
//!         ├── Owner Channel  encrypted Messaging topic w/ owner
//!         ├── Spending Policy threshold engine + approval queue
//!         ├── A2A binding   Card publish/discover, task lifecycle
//!         ├── Skill Registry  Box<dyn Skill> lookup table
//!         └── LLM backend   pluggable inference trait
//!         │   calls wallet-ffi (Rust), storage_module, delivery_module
//!         ▼
//!   wallet_ffi  +  Other Logos modules via LogosAPI bridge
//! ```

pub mod a2a;
pub mod dispatcher;
pub mod ffi;
pub mod identity;
pub mod llm;
pub mod owner_channel;
pub mod runtime;
pub mod skills;
pub mod spending_policy;
pub mod state;

// Re-export the SDK so embedders only need one dep.
pub use agent_skill_sdk as sdk;

/// Library version reported via `agent_version()`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
