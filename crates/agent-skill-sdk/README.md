# agent-skill-sdk

Build skills for the [Logos Autonomous AI Agent Module](https://github.com/warfield2016/logos-agent) (LP-0008).

A *skill* is a small, composable, side-effect-isolated unit of agent capability — e.g. `storage.upload`, `wallet.send`, `translate.en_to_ja`. Third-party skills plug into the agent runtime via the `Skill` trait and become callable both by the agent's owner and by other agents over A2A.

## Quickstart

```toml
[dependencies]
agent-skill-sdk = "0.1"
async-trait = "0.1"
serde_json = "1"
```

```rust
use agent_skill_sdk::{
    async_trait, register_skill, Skill, SkillContext, SkillManifest, SkillResult,
    ParamKind, ParamSpec, TokenAmount,
};
use serde_json::{json, Value};

pub struct EchoSkill;

#[async_trait]
impl Skill for EchoSkill {
    fn name(&self) -> &str { "demo.echo" }

    fn manifest(&self) -> SkillManifest {
        SkillManifest {
            name: self.name().to_string(),
            description: "Echo the input text back to the caller.".into(),
            params: vec![ParamSpec {
                name: "text".into(),
                kind: ParamKind::String,
                required: true,
                description: "Text to echo".into(),
            }],
            output_schema: json!({ "type": "object", "properties": { "echoed": { "type": "string" } } }),
            price_lez: Some(TokenAmount(1)),    // 1 micro-LEZ per call
            may_spend: false,
        }
    }

    async fn invoke(&self, params: Value, _ctx: &dyn SkillContext) -> SkillResult<Value> {
        let text = params["text"].as_str().ok_or_else(|| {
            agent_skill_sdk::SkillError::InvalidParams("text required".into())
        })?;
        Ok(json!({ "echoed": text }))
    }
}

register_skill!(EchoSkill);
```

## Deploying your skill

Once your skill crate is published to crates.io (or hosted as a git dependency), an agent operator can add it to their agent config:

```toml
# ~/.logos-agent/config.toml
[skills]
custom = ["my-cool-skill = { git = \"https://github.com/me/my-cool-skill\" }"]
```

Then redeploy: `logos-agent reload`.

## What the SDK provides

| Trait / Type | Purpose |
|-------------|---------|
| `Skill` | The trait every skill implements |
| `SkillContext` | Handles to wallet/storage/messaging/LLM, scoped state |
| `WalletHandle`, `StorageHandle`, `MessagingHandle` | Async APIs to other Logos modules |
| `SkillManifest` | Self-describing metadata published in the AgentCard |
| `register_skill!` | Macro to declare a skill for the runtime to discover |

## Guarantees

- **Panic isolation:** a panicking skill never crashes the agent or other skills.
- **Spending policy enforcement:** the runtime intercepts `WalletHandle::send` calls and queues above-threshold txs for owner approval — skills don't need to think about it.
- **Encrypted messaging:** all `MessagingHandle::send` is end-to-end encrypted via Logos Messaging.

## Spec

Full skill contract: [`/spec/skill-interface.md`](https://github.com/warfield2016/logos-agent/blob/main/spec/skill-interface.md).
A2A binding: [`/spec/a2a-logos-messaging-binding.md`](https://github.com/warfield2016/logos-agent/blob/main/spec/a2a-logos-messaging-binding.md).
