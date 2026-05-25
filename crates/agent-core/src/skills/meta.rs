use agent_skill_sdk::{
    async_trait, ParamKind, ParamSpec, Skill, SkillContext, SkillError, SkillManifest, SkillResult,
};
use serde_json::{json, Value};

pub struct SkillsSkill;
#[async_trait]
impl Skill for SkillsSkill {
    fn name(&self) -> &str {
        "meta.skills"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "List all available skills and their parameters.".into(),
            params: vec![],
            output_schema: json!({"type": "array"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, _params: Value, _ctx: &dyn SkillContext) -> SkillResult<Value> {
        // Note: in Week-2 this will pull from the live registry via ctx
        // (which currently doesn't expose registry — that's deliberate). For
        // now it returns the known skill names so meta.skills is responsive.
        Ok(json!([
            "storage.upload",
            "storage.download",
            "storage.list",
            "storage.share",
            "messaging.send",
            "messaging.join",
            "messaging.create_group",
            "wallet.balance",
            "wallet.send",
            "wallet.history",
            "program.query",
            "program.call",
            "program.deploy",
            "agent.card",
            "agent.discover",
            "agent.task",
            "agent.subscribe",
            "agent.cancel",
            "meta.skills",
            "meta.status",
            "meta.configure",
        ]))
    }
}

pub struct StatusSkill;
#[async_trait]
impl Skill for StatusSkill {
    fn name(&self) -> &str {
        "meta.status"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Report the agent's current state: balance, storage usage, active tasks."
                .into(),
            params: vec![],
            output_schema: json!({"type": "object"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, _params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let balance = ctx.wallet().balance().await.ok().map(|b| b.0.to_string());
        Ok(json!({
            "npk": ctx.identity().npk,
            "owner_npk": ctx.identity().owner_npk,
            "balance_smallest": balance,
        }))
    }
}

pub struct ConfigureSkill;
#[async_trait]
impl Skill for ConfigureSkill {
    fn name(&self) -> &str {
        "meta.configure"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Update runtime configuration (spending threshold, owner address, etc.)."
                .into(),
            params: vec![
                ParamSpec {
                    name: "key".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Config key".into(),
                },
                ParamSpec {
                    name: "value".into(),
                    kind: ParamKind::Object,
                    required: true,
                    description: "New value".into(),
                },
            ],
            output_schema: json!({"type": "object", "properties": {"updated": {"type": "boolean"}}}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, _ctx: &dyn SkillContext) -> SkillResult<Value> {
        let key = params["key"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("key required".into()))?;
        // Owner-gated; runtime should verify caller == owner before mutation.
        Err(SkillError::Upstream(format!(
            "meta.configure({key}): pending Week-2 owner-auth check + config persistence"
        )))
    }
}
