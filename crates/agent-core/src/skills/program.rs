use agent_skill_sdk::{
    async_trait, ParamKind, ParamSpec, Skill, SkillContext, SkillError, SkillManifest, SkillResult,
};
use serde_json::{json, Value};

pub struct QuerySkill;
#[async_trait]
impl Skill for QuerySkill {
    fn name(&self) -> &str {
        "program.query"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Read state from a LEZ program.".into(),
            params: vec![
                ParamSpec {
                    name: "program_id".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Program id".into(),
                },
                ParamSpec {
                    name: "params".into(),
                    kind: ParamKind::Object,
                    required: true,
                    description: "Query params (JSON object)".into(),
                },
            ],
            output_schema: json!({"type": "object"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let program_id = params["program_id"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("program_id required".into()))?;
        ctx.wallet()
            .query_program(program_id, params["params"].clone())
            .await
    }
}

pub struct CallSkill;
#[async_trait]
impl Skill for CallSkill {
    fn name(&self) -> &str {
        "program.call"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Submit a transaction to a LEZ program. Subject to spending policy."
                .into(),
            params: vec![
                ParamSpec {
                    name: "program_id".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Program id".into(),
                },
                ParamSpec {
                    name: "instruction".into(),
                    kind: ParamKind::String,
                    required: true,
                    description: "Instruction name".into(),
                },
                ParamSpec {
                    name: "params".into(),
                    kind: ParamKind::Object,
                    required: true,
                    description: "Instruction params (JSON object)".into(),
                },
            ],
            output_schema: json!({"type": "object", "properties": {"tx_hash": {"type": "string"}}}),
            price_lez: None,
            may_spend: true,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let program_id = params["program_id"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("program_id required".into()))?;
        let instruction = params["instruction"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("instruction required".into()))?;
        let receipt = ctx
            .wallet()
            .call_program(program_id, instruction, params["params"].clone())
            .await?;
        Ok(json!({"tx_hash": receipt.tx_hash, "block_height": receipt.block_height}))
    }
}

pub struct DeploySkill;
#[async_trait]
impl Skill for DeploySkill {
    fn name(&self) -> &str {
        "program.deploy"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Deploy a compiled LEZ program binary to the network.".into(),
            params: vec![ParamSpec {
                name: "binary_path".into(),
                kind: ParamKind::Path,
                required: true,
                description: "Local path to compiled RISC-V binary".into(),
            }],
            output_schema: json!({"type": "object", "properties": {"program_id": {"type": "string"}}}),
            price_lez: None,
            may_spend: true,
        }
    }
    async fn invoke(&self, _params: Value, _ctx: &dyn SkillContext) -> SkillResult<Value> {
        // Implemented Week-3 via a dedicated `wallet-ffi` deploy entrypoint.
        Err(SkillError::Upstream(
            "program.deploy: pending Week-3 wallet-ffi deploy hook".into(),
        ))
    }
}
