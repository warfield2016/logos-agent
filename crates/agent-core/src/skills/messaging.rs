use agent_skill_sdk::{
    async_trait, ParamKind, ParamSpec, Skill, SkillContext, SkillError, SkillManifest, SkillResult,
};
use serde_json::{json, Value};

pub struct SendSkill;
#[async_trait]
impl Skill for SendSkill {
    fn name(&self) -> &str {
        "messaging.send"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Send an encrypted message to a Logos Messaging address.".into(),
            params: vec![
                ParamSpec {
                    name: "recipient".into(),
                    kind: ParamKind::Address,
                    required: true,
                    description: "Recipient NPK or group id".into(),
                },
                ParamSpec {
                    name: "message".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Message body (UTF-8)".into(),
                },
            ],
            output_schema: json!({"type": "object", "properties": {"message_id": {"type": "string"}}}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let recipient = params["recipient"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("recipient required".into()))?;
        let body = params["message"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("message required".into()))?;
        let id = ctx.messaging().send(recipient, body.as_bytes()).await?;
        Ok(json!({"message_id": id.0}))
    }
}

pub struct JoinSkill;
#[async_trait]
impl Skill for JoinSkill {
    fn name(&self) -> &str {
        "messaging.join"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Join a Logos Messaging group topic.".into(),
            params: vec![ParamSpec {
                name: "group_id".into(),
                kind: ParamKind::String,
                required: true,
                description: "Group id".into(),
            }],
            output_schema: json!({"type": "object"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let group = params["group_id"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("group_id required".into()))?;
        ctx.messaging().join(group).await?;
        Ok(json!({"joined": group}))
    }
}

pub struct CreateGroupSkill;
#[async_trait]
impl Skill for CreateGroupSkill {
    fn name(&self) -> &str {
        "messaging.create_group"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Create a new group topic and invite members.".into(),
            params: vec![ParamSpec {
                name: "members".into(),
                kind: ParamKind::Object,
                required: true,
                description: "Array of NPK strings".into(),
            }],
            output_schema: json!({"type": "object", "properties": {"group_id": {"type": "string"}}}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let members: Vec<String> = serde_json::from_value(params["members"].clone())
            .map_err(|e| SkillError::InvalidParams(format!("members: {e}")))?;
        let id = ctx.messaging().create_group(&members).await?;
        Ok(json!({"group_id": id.0}))
    }
}
