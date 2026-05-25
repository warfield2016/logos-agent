use crate::a2a::{topics, AgentCard, A2A_VERSION};
use agent_skill_sdk::{
    async_trait, ParamKind, ParamSpec, Skill, SkillContext, SkillError, SkillManifest, SkillResult,
};
use serde_json::{json, Value};

pub struct CardSkill;
#[async_trait]
impl Skill for CardSkill {
    fn name(&self) -> &str {
        "agent.card"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Return this agent's A2A-compatible AgentCard.".into(),
            params: vec![],
            output_schema: json!({"type": "object"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, _params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let id = ctx.identity();
        let card = AgentCard::for_agent(
            id.npk.clone(),
            id.name.clone(),
            format!("Logos Autonomous Agent (A2A v{A2A_VERSION})"),
            "logos-agent".into(),
            vec![],
            &id.npk,
        );
        serde_json::to_value(card).map_err(|e| SkillError::Execution(e.to_string()))
    }
}

pub struct DiscoverSkill;
#[async_trait]
impl Skill for DiscoverSkill {
    fn name(&self) -> &str {
        "agent.discover"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Fetch AgentCards from a discovery topic.".into(),
            params: vec![ParamSpec {
                name: "topic".into(),
                kind: ParamKind::String,
                required: false,
                description: format!("Discovery topic (default {})", topics::CARDS_DISCOVERY),
            }],
            output_schema: json!({"type": "array"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, _params: Value, _ctx: &dyn SkillContext) -> SkillResult<Value> {
        Err(SkillError::Upstream(
            "agent.discover: pending Week-3 discovery-topic subscription".into(),
        ))
    }
}

pub struct TaskSkill;
#[async_trait]
impl Skill for TaskSkill {
    fn name(&self) -> &str {
        "agent.task"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Send a task request to another agent over A2A.".into(),
            params: vec![
                ParamSpec {
                    name: "agent_address".into(),
                    kind: ParamKind::Address,
                    required: true,
                    description: "Provider NPK".into(),
                },
                ParamSpec {
                    name: "skill".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Skill name to invoke".into(),
                },
                ParamSpec {
                    name: "params".into(),
                    kind: ParamKind::Object,
                    required: true,
                    description: "Skill params".into(),
                },
            ],
            output_schema: json!({"type": "object", "properties": {"task_id": {"type": "string"}, "status": {"type": "string"}}}),
            price_lez: None,
            may_spend: true,
        }
    }
    async fn invoke(&self, _params: Value, _ctx: &dyn SkillContext) -> SkillResult<Value> {
        Err(SkillError::Upstream(
            "agent.task: pending Week-3 A2A transport binding".into(),
        ))
    }
}

pub struct SubscribeSkill;
#[async_trait]
impl Skill for SubscribeSkill {
    fn name(&self) -> &str {
        "agent.subscribe"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Subscribe to streaming status updates for a task.".into(),
            params: vec![
                ParamSpec {
                    name: "agent_address".into(),
                    kind: ParamKind::Address,
                    required: true,
                    description: "Provider NPK".into(),
                },
                ParamSpec {
                    name: "task_id".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Task id".into(),
                },
            ],
            output_schema: json!({"type": "object"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, _params: Value, _ctx: &dyn SkillContext) -> SkillResult<Value> {
        Err(SkillError::Upstream(
            "agent.subscribe: pending Week-3 streaming binding".into(),
        ))
    }
}

pub struct CancelSkill;
#[async_trait]
impl Skill for CancelSkill {
    fn name(&self) -> &str {
        "agent.cancel"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Cancel an in-progress task and trigger any applicable refund.".into(),
            params: vec![
                ParamSpec {
                    name: "agent_address".into(),
                    kind: ParamKind::Address,
                    required: true,
                    description: "Provider NPK".into(),
                },
                ParamSpec {
                    name: "task_id".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Task id".into(),
                },
            ],
            output_schema: json!({"type": "object", "properties": {"canceled": {"type": "boolean"}}}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, _params: Value, _ctx: &dyn SkillContext) -> SkillResult<Value> {
        Err(SkillError::Upstream(
            "agent.cancel: pending Week-3 cancel binding".into(),
        ))
    }
}
