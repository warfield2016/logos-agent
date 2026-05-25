//! Built-in skills required by the LP-0008 prize spec.
//!
//! 21 skills across 6 categories:
//!   storage (4): upload, download, list, share
//!   messaging (3): send, join, create_group
//!   wallet (3): balance, send, history
//!   program (3): query, call, deploy
//!   agent (5): card, discover, task, subscribe, cancel
//!   meta (3): skills, status, configure
//!
//! All skills follow the same pattern: small struct, `Skill` impl, register in
//! `register_default_skills`. Skill bodies that touch wallet/storage/messaging
//! return `Err(Upstream)` in Week-1; Week-2 fills them in via the live
//! handles on `SkillContext`.

mod agent;
mod messaging;
mod meta;
mod program;
mod storage;
mod wallet;

use agent_skill_sdk::SkillRegistry;

pub fn register_default_skills(registry: &mut SkillRegistry) {
    // Storage
    registry.register(Box::new(storage::UploadSkill));
    registry.register(Box::new(storage::DownloadSkill));
    registry.register(Box::new(storage::ListSkill));
    registry.register(Box::new(storage::ShareSkill));

    // Messaging
    registry.register(Box::new(messaging::SendSkill));
    registry.register(Box::new(messaging::JoinSkill));
    registry.register(Box::new(messaging::CreateGroupSkill));

    // Wallet
    registry.register(Box::new(wallet::BalanceSkill));
    registry.register(Box::new(wallet::SendSkill));
    registry.register(Box::new(wallet::HistorySkill));

    // Program (LEZ)
    registry.register(Box::new(program::QuerySkill));
    registry.register(Box::new(program::CallSkill));
    registry.register(Box::new(program::DeploySkill));

    // Agent (A2A)
    registry.register(Box::new(agent::CardSkill));
    registry.register(Box::new(agent::DiscoverSkill));
    registry.register(Box::new(agent::TaskSkill));
    registry.register(Box::new(agent::SubscribeSkill));
    registry.register(Box::new(agent::CancelSkill));

    // Meta
    registry.register(Box::new(meta::SkillsSkill));
    registry.register(Box::new(meta::StatusSkill));
    registry.register(Box::new(meta::ConfigureSkill));
}
