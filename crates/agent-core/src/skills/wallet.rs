use agent_skill_sdk::{
    async_trait, ParamKind, ParamSpec, Skill, SkillContext, SkillError, SkillManifest, SkillResult,
    TokenAmount,
};
use serde_json::{json, Value};

pub struct BalanceSkill;
#[async_trait]
impl Skill for BalanceSkill {
    fn name(&self) -> &str {
        "wallet.balance"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Return the agent's current shielded token balance.".into(),
            params: vec![],
            output_schema: json!({"type": "object", "properties": {"balance_smallest": {"type": "string"}}}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, _params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let bal = ctx.wallet().balance().await?;
        Ok(json!({"balance_smallest": bal.0.to_string()}))
    }
}

pub struct SendSkill;
#[async_trait]
impl Skill for SendSkill {
    fn name(&self) -> &str {
        "wallet.send"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Send tokens to a Logos address. Subject to spending policy.".into(),
            params: vec![
                ParamSpec {
                    name: "recipient".into(),
                    kind: ParamKind::Address,
                    required: true,
                    description: "Recipient NPK".into(),
                },
                ParamSpec {
                    name: "amount".into(),
                    kind: ParamKind::TokenAmount,
                    required: true,
                    description: "Amount in smallest unit (string to preserve u128 precision)"
                        .into(),
                },
            ],
            output_schema: json!({"type": "object", "properties": {"tx_hash": {"type": "string"}}}),
            price_lez: None,
            may_spend: true,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let recipient = params["recipient"]
            .as_str()
            .ok_or_else(|| SkillError::InvalidParams("recipient required".into()))?;
        let amount_str = params["amount"]
            .as_str()
            .or_else(|| params["amount"].as_u64().map(|_| ""))
            .ok_or_else(|| SkillError::InvalidParams("amount required as string".into()))?;
        let amount: u128 = amount_str
            .parse()
            .map_err(|_| SkillError::InvalidParams("amount must be u128".into()))?;
        let receipt = ctx.wallet().send(recipient, TokenAmount(amount)).await?;
        Ok(json!({"tx_hash": receipt.tx_hash, "block_height": receipt.block_height}))
    }
}

pub struct HistorySkill;
#[async_trait]
impl Skill for HistorySkill {
    fn name(&self) -> &str {
        "wallet.history"
    }
    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().into(),
            description: "Return a summary of recent transactions.".into(),
            params: vec![ParamSpec {
                name: "limit".into(),
                kind: ParamKind::Integer,
                required: false,
                description: "Max records (default 25)".into(),
            }],
            output_schema: json!({"type": "array"}),
            price_lez: None,
            may_spend: false,
        }
    }
    async fn invoke(&self, params: Value, ctx: &dyn SkillContext) -> SkillResult<Value> {
        let limit = params["limit"].as_u64().unwrap_or(25) as usize;
        let history = ctx.wallet().history(limit).await?;
        Ok(json!(history))
    }
}
