# Skill Interface Specification

**Status:** 0.1 — 2026-05-24
**Crate:** `agent-skill-sdk` v0.1

This document specifies the contract between the Logos Autonomous AI Agent runtime and third-party skills. It is the canonical reference for skill authors and the basis of the prize's "documented skill interface" success criterion.

## 1. Goals

- **Pluggable.** A skill may be added to a running agent without modifying the agent's core module.
- **Isolated.** A skill failure must not crash the runtime or affect other skills.
- **Self-describing.** Each skill publishes its parameters, output schema, and price; this metadata is consumed both by owners and by other agents over A2A.
- **Composable.** Skills are small and orthogonal; complex behaviour is composed by chaining skills, not by overloading a single skill.

## 2. The `Skill` Trait

A skill is anything that implements:

```rust
#[async_trait]
pub trait Skill: Send + Sync {
    fn name(&self) -> &str;
    fn manifest(&self) -> SkillManifest;
    async fn invoke(
        &self,
        params: serde_json::Value,
        ctx: &dyn SkillContext,
    ) -> SkillResult<serde_json::Value>;
}
```

### 2.1 `name`

The skill's stable identifier in `<category>.<verb>` form (e.g., `storage.upload`, `wallet.send`, `translate.en_to_ja`). Names MUST be globally unique within a deployed agent. Skill authors SHOULD prefix proprietary skills with their org name (e.g., `acme.priceFeed`) to avoid collisions.

### 2.2 `manifest`

Returns the self-describing `SkillManifest` published in the agent's AgentCard. The manifest declares:

- `name`, `description`
- `params: Vec<ParamSpec>` — typed parameter schema
- `output_schema: serde_json::Value` — JSON Schema for the return shape
- `price_lez: Option<TokenAmount>` — per-call price if offered over A2A
- `may_spend: bool` — whether the skill may move tokens or modify on-chain state (subject to spending policy)

### 2.3 `invoke`

Executes the skill with JSON-encoded params and a runtime context. Returns a `SkillResult<Value>`.

Skills MUST NOT panic. The runtime catches panics and reports them as `SkillError::Execution`, but a returned error is more actionable than a caught panic.

## 3. The `SkillContext` Trait

Skills do not have direct access to other Logos modules. The runtime injects a `SkillContext` providing:

```rust
trait SkillContext {
    fn wallet(&self) -> &dyn WalletHandle;
    fn storage(&self) -> &dyn StorageHandle;
    fn messaging(&self) -> &dyn MessagingHandle;
    fn identity(&self) -> &AgentIdentity;
    async fn infer(&self, prompt: &str) -> SkillResult<String>;
    async fn state_get(&self, key: &str) -> SkillResult<Option<Vec<u8>>>;
    async fn state_put(&self, key: &str, value: Vec<u8>) -> SkillResult<()>;
    async fn request_owner_approval(&self, summary: &str) -> SkillResult<bool>;
}
```

`wallet`, `storage`, and `messaging` route through the corresponding Logos Core modules via the LogosAPI bridge. `infer` calls the agent's configured LLM backend. `state_get`/`state_put` provide persistent K/V scoped per skill name.

`request_owner_approval` is rarely needed: the runtime automatically routes above-threshold wallet operations through the owner channel.

## 4. Spending Policy Interaction

When `may_spend = true`, the runtime intercepts the skill's wallet operations:

```
skill calls ctx.wallet().send(to, amount)
   │
   ▼
spending_policy.evaluate(token, amount)
   │
   ├─── Autonomous ───────────────────────► wallet-ffi transfer
   │
   └─── OwnerApprovalRequired(summary) ───► queue on OwnerChannel
                                            await reply
                                            if approved ──► wallet-ffi
                                            if denied   ──► SkillError::ApprovalDenied
```

Skills MUST treat `ApprovalDenied` as a non-error condition for control flow — the owner exercised legitimate authority. Returning a clear failure message helps the agent's caller understand why the task couldn't complete.

## 5. Registration

```rust
use agent_skill_sdk::{register_skill, Skill, ...};

pub struct MySkill;
impl Skill for MySkill { ... }

register_skill!(MySkill);
```

The macro emits a hidden `__agent_register_skill()` function the runtime calls during startup. Third-party skill crates expose a public `skills()` function returning `Vec<Box<dyn Skill>>` that the operator references in their agent config:

```toml
# ~/.logos-agent/config.toml
[skills.third_party]
my-cool-skill = { crate = "my-cool-skill", version = "0.1" }
```

## 6. Conventions

- **Names.** `<category>.<verb>`. Verbs are imperative. Categories are lowercase. Use `.` as separator, not `_` or `-`.
- **Param schemas.** Prefer explicit `ParamSpec` lists over free-form `Object` params. The dispatcher uses the schema to validate inputs and to render help in the owner CLI.
- **Output schemas.** Use JSON Schema draft 2020-12. Keep return shapes flat and stable; consumers will pattern-match on them.
- **Idempotency.** Where possible, design skills so that calling them twice with the same params yields the same result. This makes retry logic safe.
- **Side effects.** Document side effects in the `description` field. "Uploads a file to Logos Storage" is fine; "Uploads a file, then notifies the owner, then debits 5 LEZ" is more informative.

## 7. Examples

See `crates/agent-core/src/skills/` for the 21 default skills.

## 8. Versioning

The SDK follows semver. The 0.x line may break compatibility between minor versions; 1.0 will commit to backward compatibility within the major.
