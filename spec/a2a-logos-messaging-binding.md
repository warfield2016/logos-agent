# A2A Transport Binding: Logos Messaging

**Binding URI:** `https://logos.co/spec/a2a-bindings/logos-messaging/0.1`
**A2A Version:** 1.0.0
**Status:** Draft 0.1 — 2026-05-24
**Editors:** LP-0008 contributors

## 1. Abstract

This document specifies a custom transport binding for the [Agent2Agent (A2A) Protocol](https://a2a-protocol.org/latest/specification/) using [Logos Messaging](https://github.com/logos-co/logos-delivery-module) (Waku v2) as the underlying transport. It provides functional equivalence to A2A's canonical HTTP/JSON-RPC binding while adding two properties the canonical binding cannot offer:

- **End-to-end encryption with no central server or coordinator** — Logos Messaging messages are encrypted to the recipient's NPK and routed through a privacy-preserving mixnet.
- **Censorship resistance** — no DNS, no IP-level addressing, no hosted endpoints. Agents are reachable as long as the mixnet is reachable.

This binding is intended to be composed with the [LEZ Payment Extension](./lez-payment-extension.md) to produce A2A-compatible agent interactions with native payment and privacy primitives.

## 2. Conformance

A conforming implementation MUST satisfy all of A2A spec §12 ("Custom Binding Guidelines"):

1. **Functional Equivalence (§12.1)** — provides equivalents of every operation in A2A's abstract layer.
2. **Data Type Mappings (§12.2)** — defined in §6 below.
3. **Service Parameters (§12.3)** — defined in §5.3 below.
4. **Error Mapping (§12.4)** — defined in §7 below.
5. **Streaming Support (§12.5)** — defined in §4.3 below.
6. **Authentication (§12.6)** — defined in §8 below.
7. **Agent Card Declaration (§12.7)** — defined in §9 below.
8. **Interop Testing (§12.8)** — see `/tests/interop/` in the reference implementation.

## 3. Identifiers and Addressing

| Item | Form |
|------|------|
| Agent identity | 32-byte ed25519 verifying key (NPK), hex-encoded |
| Endpoint URI | `waku:/logos-a2a/0.1/tasks/<recipient_npk_hex>/inbox` |
| Discovery URI | `waku:/logos-a2a/0.1/cards` |

Agents are addressed by their NPK. Endpoints are Waku content topics, not URLs.

## 4. Operation Mapping

### 4.1 SendMessage / SendStreamingMessage

A `SendMessage` operation is mapped to a single `TaskRequest` envelope published to the recipient's inbox topic:

```
publish topic = /logos-a2a/0.1/tasks/<recipient_npk_hex>/inbox
body          = Envelope::TaskRequest { task_id, from, skill, params, context_id, signature }
```

The provider acknowledges receipt by publishing a `TaskStatus { status: SUBMITTED }` to the task's status topic. If the provider rejects, it publishes `TaskStatus { status: REJECTED, message }` and the interaction ends.

### 4.2 GetTask, CancelTask, ListTasks

`GetTask` and `ListTasks` are local lookups against the provider's TaskStore — they do not generate network traffic.

`CancelTask` is mapped to a `Cancel` envelope on the task's status topic; the provider transitions the task to `CANCELED` and emits a final `TaskStatus` event.

### 4.3 SubscribeToTask (streaming)

A subscriber subscribes to two Waku topics:

```
/logos-a2a/0.1/tasks/<task_id>/status
/logos-a2a/0.1/tasks/<task_id>/artifacts
```

Status updates are published as `TaskStatus { task_id, seq, status, message }` envelopes; artifact deltas as `TaskArtifact { task_id, seq, artifact }`.

**Ordering.** Waku does not guarantee delivery order. Each task envelope MUST carry a strictly-monotonic `seq: u64` field starting at 0. Subscribers MUST reorder by `seq` before delivering to the application. Duplicates (same `(task_id, seq)`) MUST be discarded.

**Reconnection.** A subscriber that disconnects and reconnects MUST replay from the last received `seq` by reading the topic's persisted history (Waku store protocol). Providers SHOULD retain status messages for at least 24 hours.

**Termination.** The terminal `TaskStatus` (any of `COMPLETED`, `FAILED`, `CANCELED`, `REJECTED`) signals the end of the stream. Providers MUST emit a single terminal status; subscribers MUST treat its receipt as end-of-stream.

### 4.4 Push Notifications (§3.1.7–§3.1.10)

NOT SUPPORTED in v0.1. The streaming mechanism in §4.3 provides equivalent functionality without webhooks. AgentCards MUST declare `capabilities.pushNotifications = false`.

## 5. Topic Schema

All topics are version-prefixed under `/logos-a2a/0.1/` so that future binding revisions can coexist.

| Topic | Purpose | Publishers | Subscribers |
|-------|---------|-----------|-------------|
| `/logos-a2a/0.1/cards` | Discovery broadcast | Providers | Any |
| `/logos-a2a/0.1/cards/<provider_npk>` | Direct card pull | Providers | Specific clients |
| `/logos-a2a/0.1/tasks/<recipient_npk>/inbox` | Task requests | Clients | The recipient |
| `/logos-a2a/0.1/tasks/<task_id>/status` | Status updates | Provider | Subscribers |
| `/logos-a2a/0.1/tasks/<task_id>/artifacts` | Artifact deltas | Provider | Subscribers |
| `/logos-a2a/0.1/payments/<task_id>` | Payment commits/receipts | Both sides | Both sides |

### 5.3 Service Parameters

A2A "service parameters" are embedded in the envelope as top-level metadata fields:

| Field | Purpose |
|-------|---------|
| `a2a_version` | `"1.0.0"` |
| `binding_version` | `"0.1"` |
| `trace_id` | Optional opaque tracing token |
| `extensions` | Array of extension URIs active for this interaction |

## 6. Wire Format

Production messages use the canonical A2A Protocol Buffer schema (`spec/a2a.proto`), encoded as raw protobuf bytes. The reference implementation also accepts JSON envelopes for debugging and development; production interop requires protobuf.

Envelope variants (see `transport.rs` in the reference implementation):

- `TaskRequest`
- `TaskStatus`
- `TaskArtifact`
- `Cancel`
- `Payment` (when LEZ-Payment extension is active)

## 7. Error Mapping

A2A error codes from §11.1 map to the `TaskStatus` envelope's `message` field, with the A2A code preserved verbatim.

| A2A Error | Logos Messaging Binding |
|-----------|--------------------------|
| `TASK_NOT_FOUND` | `TaskStatus { status: FAILED, message: "TASK_NOT_FOUND: ..." }` |
| `UNSUPPORTED_OPERATION` | `TaskStatus { status: REJECTED, message: "UNSUPPORTED_OPERATION: ..." }` |
| `INVALID_PARAMS` | `TaskStatus { status: REJECTED, message: "INVALID_PARAMS: ..." }` |
| `AUTHENTICATION_REQUIRED` | `TaskStatus { status: AUTH_REQUIRED, message: "..." }` |
| `INTERNAL_ERROR` | `TaskStatus { status: FAILED, message: "INTERNAL_ERROR: ..." }` |

Transport-level errors (mixnet unreachable, RLN rejection, etc.) MUST NOT silently fail; the client surfaces them as a local error distinct from any A2A error code.

## 8. Authentication

The binding inherits Logos Messaging's authentication: every envelope is signed by the sender's NPK, and the wire format is encrypted to the recipient's NPK.

Higher-level authentication (e.g., proving group membership, OAuth2 tokens) is layered on top via A2A's `securitySchemes` field. AgentCards that require additional auth declare it in `securitySchemes` per A2A §4.5.

## 9. Agent Card Declaration

A conforming provider MUST publish an AgentCard with an `interfaces` entry declaring this binding:

```json
{
  "interfaces": [{
    "uri": "https://logos.co/spec/a2a-bindings/logos-messaging/0.1",
    "endpoint": "waku:/logos-a2a/0.1/tasks/<npk_hex>/inbox",
    "version": "0.1"
  }]
}
```

If the provider also speaks the canonical HTTP/JSON-RPC binding (i.e., is bilingual), it lists both entries with distinct URIs. The first entry is the preferred transport.

## 10. Security Considerations

- **Replay.** Clients MUST include the current Logos block height in the `TaskRequest` signature payload; providers MUST reject envelopes whose block height is more than 10 blocks behind the local tip.
- **Sender impersonation.** Each envelope's `signature` field is verified against the `from` field's NPK before any state mutation.
- **DoS via topic flooding.** Providers SHOULD enforce per-NPK rate limits and may use Logos Messaging's [RLN](https://github.com/logos-co/mix-rln-spam-protection-plugin) for cryptographic spam protection.
- **Metadata leakage.** Task topics expose the existence of the interaction (though not its content) to mixnet observers. Sensitive interactions SHOULD use ephemeral one-time `task_id` keys disconnected from the agents' canonical NPKs.

## 11. Interop Reference

A reference implementation in pure Rust is at:

- Repository: `warfield2016/logos-agent`
- Crate: `agent-core`
- Modules: `a2a::transport`, `a2a::discovery`, `a2a::agent_card`

Conformance tests: `/tests/interop/a2a_binding/` — covers all operations in §4 and validates wire format against the A2A protobuf schema.

## 12. Open Questions

| Q | Status |
|---|--------|
| Should `task_id` derive from a Pedersen commitment of `(from, skill, params, nonce)` to allow correlation-free reissuance? | Open — gathering input. |
| Should the binding declare a max envelope size aligned with Waku's limit (~150 KB)? | Open — likely yes; current `Artifact::Storage` variant already addresses large payloads. |
| Should we publish this binding as `cpb-logos-messaging` to the `a2aproject` org per the official Custom Protocol Bindings governance? | Open — depends on receptiveness of the a2aproject maintainers. |
