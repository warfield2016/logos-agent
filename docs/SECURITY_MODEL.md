# Security Model

This document specifies what the agent can do without owner approval, what
requires approval, and what is impossible by construction.

## Trust boundaries

```
┌─────────────────────────────────────────────────────────────┐
│                        OWNER                                 │
│  Holds: owner NPK/ISK                                        │
│  Authority: all configuration changes; above-threshold txs   │
└─────────────────────────────────────────────────────────────┘
                              │ encrypted Logos Messaging
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                        AGENT                                 │
│  Holds: agent NPK/ISK (separate from owner)                  │
│  Authority: skill invocation; below-threshold txs            │
│  Constrained by: spending policy                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              OTHER AGENTS (via A2A)                          │
│  May: request tasks, pay LEZ                                 │
│  May not: change agent config, exceed income cap             │
└─────────────────────────────────────────────────────────────┘
```

## What the agent does autonomously

- Invoke any skill where `may_spend = false`.
- Send tokens up to `per_tx_lez` and within `per_day_lez` cap.
- Call LEZ programs up to the same caps.
- Publish AgentCard, accept incoming A2A tasks below the `income_cap_lez`.
- Read storage, send messages, query programs.

## What requires owner approval

- Any single transaction above `per_tx_lez`.
- Any transaction that would push daily total over `per_day_lez`.
- Program deployment (`program.deploy`) — always.
- Configuration changes (`meta.configure`) — always, with NPK-signature check.

Approval flow: agent publishes encrypted approval request to owner's Messaging
inbox. Owner replies (CLI or Basecamp UI). On approval, agent attaches the
owner's signature and submits. On denial, skill returns `ApprovalDenied`.

## What the agent CANNOT do

- Change its own spending policy (only the owner can, via `meta.configure`).
- Transfer the agent identity to a new owner (would require both old and new owner signatures; not implemented in v0.1).
- Initiate transactions on the owner's behalf using the owner's keys (agent has no access to owner's ISK).
- Bypass the spending policy by composing skills (the runtime intercepts wallet calls at the `WalletHandle` boundary, not the skill boundary).

## What an attacker outside the system can do

- See that traffic exists on the agent's Messaging topics. Cannot see content (E2E encrypted).
- See on-chain transactions if `settlement = transparent`. Cannot see content if `settlement = shielded`.
- Send unsolicited A2A task requests. Rejected by income cap if value exceeds threshold; otherwise enters the task queue and is evaluated like any other task.

## What an attacker controlling the agent's machine can do

- Read the agent's ISK from disk. **Mitigation:** v0.2 will encrypt the
  identity file with a passphrase derived via Argon2id from owner-provided
  material; in v0.1 the file is plaintext on disk and protected only by
  filesystem permissions.
- Forge AgentCards as if they came from the agent. Mitigations: any AgentCard
  consumer should verify the embedded signature, and the signature is over
  the canonicalized card content excluding the signature itself.
- Replay old transactions. **Mitigation:** every transaction signs the current
  block height; replays past 10-block staleness are rejected by the sequencer.

## What an attacker controlling another A2A agent can do

- Advertise a skill they cannot perform. Mitigation: provider receives payment
  in escrow (when `settlement = escrow`); on non-delivery, client triggers
  refund.
- Pay legitimately, then deny they received the result. Mitigation: artifact
  delivery via Logos Storage produces a content-addressed CID; provider
  publishes the CID in `TaskArtifact`; the very act of receiving it is
  on-the-record. (Disputes still possible but evidence is asymmetric.)
- Spam our discovery topic with bogus AgentCards. Mitigation: RLN (rate-limiting
  nullifier) at the Messaging layer caps publication rate per identity.

## Known limitations (v0.1)

- Identity at rest is plaintext (filesystem permissions only). Fix planned for v0.2.
- No automatic key rotation. Operators rotating identity must re-publish AgentCard with a fresh sig.
- LLM inference is local-default but the API backend allows arbitrary outbound HTTP. Operators using the API backend should treat the configured endpoint as a trust dependency.
- The spending policy does NOT cap calls to free skills; a malicious A2A peer could exhaust the agent's LLM budget by sending many free task requests. Mitigation in v0.2: per-peer LLM cost cap.
