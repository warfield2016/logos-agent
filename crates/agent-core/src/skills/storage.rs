use agent_skill_sdk::{
    async_trait, ParamKind, ParamSpec, Skill, SkillContext, SkillError, SkillManifest, SkillResult,
};
use serde_json::{json, Value};

pub struct UploadSkill;
#[async_trait]
impl Skill for UploadSkill {
    fn name(&self) -> &str {
        "storage.upload"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Encrypt and upload a local file to Logos Storage.".into(),
            params: vec![
                ParamSpec {
                    name: "path".into(),
                    kind: ParamKind::Path,
                    required: true,
                    description: "Local file path".into(),
                },
                ParamSpec {
                    name: "label".into(),
                    kind: ParamKind::String,
                    required: false,
                    description: "Human label".into(),
                },
            ],
            output_schema: json!({"type": "object", "properties": {"address": {"type": "string"}}}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("path required".into()))?;
        let label = params["label"].as_str();
        let addr = ctx.storage().upload(path, label).await?;
        Ok(json!({"address": addr.0}))
    }
}

pub struct DownloadSkill;
#[async_trait]
impl Skill for DownloadSkill {
    fn name(&self) -> &str {
        "storage.download"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Retrieve and decrypt a file from Logos Storage to a local path.".into(),
            params: vec![
                ParamSpec {
                    name: "address".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Content address".into(),
                },
                ParamSpec {
                    name: "path".into(),
                    kind: ParamKind::Path,
                    required: true,
                    description: "Destination path".into(),
                },
            ],
            output_schema: json!({"type": "object", "properties": {"path": {"type": "string"}}}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let addr = agent_skill_sdk::ContentAddress(
            params["address"]
                .as_str()
                .ok_or_else(|| SkillError::InvalidParams("address required".into()))?
                .to_string(),
        );
        let path = params["path"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("path required".into()))?;
        ctx.storage().download(&addr, path).await?;
        Ok(json!({"path": path}))
    }
}

pub struct ListSkill;
#[async_trait]
impl Skill for ListSkill {
    fn name(&self) -> &str {
        "storage.list"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "List files the agent has stored.".into(),
            params: vec![],
            output_schema: json!({"type": "array"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, _params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let files = ctx.storage().list().await?;
        Ok(json!(files))
    }
}

pub struct ShareSkill;
#[async_trait]
impl Skill for ShareSkill {
    fn name(&self) -> &str {
        "storage.share"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Share access to a stored file with another Logos identity.".into(),
            params: vec![
                ParamSpec {
                    name: "address".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Content address".into(),
                },
                ParamSpec {
                    name: "recipient".into(),
                    kind: ParamKind::Address,
                    required: true,
                    description: "Recipient NPK".into(),
                },
            ],
            output_schema: json!({"type": "object"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let addr = agent_skill_sdk::ContentAddress(
            params["address"]
                .as_str()
                .ok_or_else(|| SkillError::InvalidParams("address required".into()))?
                .to_string(),
        );
        let recipient = params["recipient"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("recipient required".into()))?;
        ctx.storage().share(&addr, recipient).await?;
        Ok(json!({"shared": true}))
    }
}
