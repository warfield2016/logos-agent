# A2A Extension: LEZ Payment

**Extension URI:** `https://logos.co/spec/a2a-extensions/lez-payment/0.1`
**A2A Version:** 1.0.0
**Status:** Draft 0.1 — 2026-05-24

## 1. Abstract

This extension defines how agents using the A2A protocol settle per-task payments using LEZ tokens. It addresses a deliberate gap in the A2A specification, which leaves payment to the implementer. The extension is compatible with both the canonical HTTP transport and the [Logos Messaging Transport Binding](./a2a-logos-messaging-binding.md).

This extension is analogous to [`a2a-x402`](https://github.com/google-agentic-commerce/a2a-x402) (HTTP 402-based) but uses on-chain settlement on the Logos Execution Zone, with three settlement modes: shielded (private), transparent (public), and escrow (atomic-on-completion via a LEZ program).

## 2. Conformance

A conforming AgentCard declares this extension in its `extensions` array:

```json
{
  "extensions": [{
    "uri": "https://logos.co/spec/a2a-extensions/lez-payment/0.1",
    "required": true,
    "params": {
      "token": "LEZ",
      "settlement": "shielded"
    }
  }]
}
```

If `required: true`, clients without LEZ-payment capability MUST NOT initiate tasks with this provider.

## 3. Skill Pricing

Each skill in the AgentCard's `skills` array MAY include a `price_lez` field, expressed as the smallest unit of the declared token (string-encoded to preserve `u128` precision):

```json
{
  "name": "translate.en_to_ja",
  "description": "...",
  "params": [...],
  "output_schema": {...},
  "price_lez": "1000",
  "may_spend": false
}
```

If `price_lez` is absent or zero, the skill is free.

## 4. Settlement Modes

### 4.1 Shielded (default)

The client transfers `price_lez` to the provider's shielded account via `wallet_ffi_transfer_shielded`. The transaction's nullifier is included in the `Payment::Commit` envelope. The provider verifies the nullifier on-chain before beginning execution.

**Properties:** End-to-end private. Off-chain observers learn nothing about the transfer.

### 4.2 Transparent

The client transfers via `wallet_ffi_transfer_public`. The provider verifies the public transaction on-chain.

**Properties:** Lower latency than shielded. Publicly visible.

### 4.3 Escrow

The client opens an escrow on the [`task-escrow` LEZ program](../programs/task-escrow/) with funds locked until either:

- The provider submits a `complete` instruction with a completion proof (claims funds), or
- The client submits a `cancel` instruction (reclaims funds), or
- The configured timeout elapses (auto-refund to client).

**Properties:** Atomic on completion; protects against provider take-money-and-run.

## 5. Payment Flow

For shielded and transparent modes:

```
Client                                  Provider
  │                                        │
  │  AgentCard with price_lez=N            │
  │ <───────────────────────────────────── │
  │                                        │
  │  wallet_ffi_transfer_shielded(N)       │
  │  → tx_hash                             │
  │                                        │
  │  Envelope::Payment(Commit {            │
  │     amount: N, tx_hash, settlement     │
  │  })                                    │
  │ ──────────────────────────────────────>│
  │                                        │  verify tx on-chain
  │                                        │  begin execution
  │                                        │
  │  TaskStatus(WORKING)                   │
  │ <───────────────────────────────────── │
  │                                        │
  │           ... execution ...            │
  │                                        │
  │  TaskStatus(COMPLETED) + Artifact      │
  │ <───────────────────────────────────── │
  │                                        │
  │  Envelope::Payment(Receipt {           │
  │     confirmed_tx_hash                  │
  │  })                                    │
  │ <───────────────────────────────────── │
```

For escrow mode, the `Commit` envelope references the open escrow record instead of a direct transfer; the `Receipt` envelope references the `complete` instruction.

## 6. Cancellation and Refund

If the client cancels a task before the terminal status:

- **Shielded/Transparent:** the provider, on receiving `Cancel`, MAY issue a partial or full refund via a counter-transfer. The refund envelope (`Payment::Refund`) references both the original `tx_hash` and the refund `tx_hash`.
- **Escrow:** the client invokes `cancel` on the escrow program, which atomically returns the locked funds.

Refund policy is provider-defined and SHOULD be declared in the AgentCard's `extensions[0].params` field for client inspection before commitment.

## 7. Income Threshold (Anti-Spam)

This extension defines an optional **inverse spending threshold** on the provider side: a per-period cap on incoming payments above which the provider's agent autonomously rejects the task with `TaskStatus { status: REJECTED, message: "INCOME_CAP_EXCEEDED" }`.

This protects providers from being grooming-attacked into accepting tasks they cannot fulfil at scale, or from being used to launder large transfers. The cap is declared in the AgentCard:

```json
"extensions": [{
  "uri": "https://logos.co/spec/a2a-extensions/lez-payment/0.1",
  "params": {
    "token": "LEZ",
    "settlement": "shielded",
    "income_cap_per_day_smallest": "1000000"
  }
}]
```

The reference implementation enforces this in `spending_policy::SpendingPolicy::evaluate_incoming`.

## 8. Tax and Compliance

This extension makes no claims about the legal or tax status of payments. Operators are responsible for compliance with the jurisdictions in which they and their counterparties operate.

## 9. Open Questions

| Q | Status |
|---|--------|
| Should the extension support multi-token pricing (e.g., LEZ or USDC-on-LEZ at the client's choice)? | Open — current 0.1 limits to single token per AgentCard; multi-token is plausible for 0.2. |
| Should escrow include a multi-party dispute resolver (M-of-N from LP-0002)? | Open — depends on LP-0002 status. |
| Should we publish per-skill price floors and ceilings to enable bidding? | Out of scope for 0.1; possible 0.2 feature aligned with A2A's potential auction primitives. |
