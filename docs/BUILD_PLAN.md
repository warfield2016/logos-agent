# LP-0008 Build Plan — 4-Week Sprint

**Goal:** First merged Large prize in `logos-co/lambda-prize`. $1,200, paid in USDT.

**Sprint window:** 2026-05-25 → 2026-06-22 (4 weeks).

**Reviewer:** fryorcraken (Logos / Waku). Standards alignment matters.

**Prior art:** Beach-Bum's `Agora` submission (PR #34, self-closed 2026-04-28). One attempt, zero reviews. Field wide open. See [`docs/COMPETITIVE_INTEL.md`](COMPETITIVE_INTEL.md) for the deep dive.

## The success criteria, mapped

| Criterion | Owner | Target week |
|-----------|-------|-------------|
| Module loads in Logos Core alongside wallet/storage/messaging | Eng | 1 |
| Agent has own shielded LEZ account | Eng | 2 |
| Single CLI deploy command on remote node | Eng | 2 |
| Real-time owner ↔ agent chat via Messaging | Eng | 2 |
| Spending threshold w/ owner approval | Eng | 2 ✓ (policy engine + tests landed Week 1) |
| All 21 default skills implemented | Eng | 3 |
| Agent-to-agent A2A-compatible coordination | Eng | 3 |
| LEZ payment between two agents | Eng | 3 |
| 3+ use cases demoed end-to-end on testnet | Eng | 4 |
| **5 third-party deployments on testnet** | **Marketing/Social** | **3–4** |
| Full docs, deployment guide, owner guide | Docs | 4 |
| Module deployed on devnet/testnet | DevOps | 3 |
| CI green with `RISC0_DEV_MODE=0` | DevOps | 3 |
| Skill SDK that 3rd parties can use | Eng | 1 ✓ (`agent-skill-sdk` v0.1 stub) |
| Recovery from transient failures | Eng | 4 |
| Per-skill CU costs documented | Docs | 4 |
| Reproducible demo script (`RISC0_DEV_MODE=0`) | DevOps | 4 |
| Recorded video demo showing terminal proof generation | Marketing | 4 |

## Week 1 — Foundation (2026-05-25 → 2026-05-31)

**Status: complete (scaffolded, compiling, 6 unit tests pass).**

- [x] Rust workspace with 5 crates: `agent-skill-sdk`, `agent-core`, `agent-cli`, `agent-registry-program`, `task-escrow-program`
- [x] `agent_core` cdylib exposing `extern "C" agent_*` functions
- [x] cbindgen-generated header at `target/include/agent_core.h`
- [x] `flake.nix` using `logos-module-builder.lib.mkLogosModule`
- [x] `metadata.json` with `interface: "universal"` + universal codegen
- [x] 21 default skill stubs registered in dispatcher
- [x] `SpendingPolicy` engine with per-tx, daily, and income caps (5 unit tests passing)
- [x] `TaskStore` + `TaskStatus` lifecycle state machine (1 unit test)
- [x] A2A binding skeleton with topic schema and envelope types
- [x] Spec drafts: A2A-over-Logos-Messaging binding, LEZ payment extension, skill interface

## Week 2 — Wire to live Logos modules (2026-06-01 → 2026-06-07)

- [ ] Link `wallet-ffi` as a build dependency; implement `WalletHandle` against it
- [ ] Implement `StorageHandle` against the `storage_module` Logos API
- [ ] Implement `MessagingHandle` against the `delivery_module` Logos API
- [ ] Wire `OwnerChannel` to actually publish/subscribe over Messaging
- [ ] Implement `meta.configure` with owner-NPK auth check + on-disk persistence
- [ ] Implement `logos-agent quickstart` end-to-end (identity, faucet, owner config)
- [ ] LLM trait + `llama.cpp` default backend, Ollama + OpenAI-compat alts
- [ ] Sled-backed `state_get`/`state_put` for per-skill K/V
- [ ] Owner-channel TUI (`logos-agent chat`)
- [ ] Integration test: deploy on local sequencer, send below-threshold tx, observe success
- [ ] Integration test: above-threshold tx, observe approval queue, approve, observe success

## Week 3 — A2A and 3rd-party recruitment (2026-06-08 → 2026-06-14)

- [ ] A2A AgentCard publishing on `/logos-a2a/0.1/cards`
- [ ] A2A discovery: subscribe + parse cards, build local index
- [ ] A2A task lifecycle: full SendMessage → SUBMITTED → WORKING → COMPLETED
- [ ] Streaming subscribe with monotonic `seq` reorder buffer
- [ ] LEZ payment integration (shielded mode first; escrow mode if time)
- [ ] Cancellation with refund
- [ ] Deploy `agent-registry` LEZ program on testnet
- [ ] Deploy `task-escrow` LEZ program on testnet
- [ ] Two-agent E2E test: agent A discovers B, requests task, pays in LEZ, gets result
- [ ] CI with `RISC0_DEV_MODE=0` proof verification — green on default branch
- [ ] **Recruit third-party deployers (target: 7 to safely clear the 5 minimum).** See [`docs/RECRUITMENT.md`](RECRUITMENT.md).

## Week 4 — Demos, docs, submission (2026-06-15 → 2026-06-22)

- [ ] Three illustrative use case demos recorded end-to-end:
  1. **Privacy-preserving notary** — owner sends doc, agent timestamps, stores, returns LEZ-recorded address
  2. **Agent services marketplace** — agent A advertises translation skill at price N, agent B discovers, pays, receives result
  3. **Cross-framework interop** — a 30-line LangChain agent discovers and calls a Logos agent over A2A
- [ ] All videos show `RISC0_DEV_MODE=0` in terminal banner (real proofs, not dev mode)
- [ ] `DEPLOYMENT.md` complete with sequencer-standalone + module-install + agent-config walkthrough
- [ ] `SECURITY_MODEL.md` complete: what the agent can / cannot do without owner approval
- [ ] Per-skill CU cost table from testnet measurements
- [ ] Reproducible demo script: `./scripts/full-demo.sh` from clean clone
- [ ] On-chain verification: query `agent-registry` to show 5+ unique non-team NPKs registered
- [ ] Submission PR opened against `logos-co/lambda-prize`

## Risk register

| Risk | Mitigation |
|------|-----------|
| 5-deployer recruitment fails | $50/deployer bounty (budgeted $250); ship one intrinsically-valuable starter skill so adoption is its own reward |
| `wallet-ffi` API changes mid-sprint | Pin to a specific commit hash in `flake.nix`; track upstream weekly |
| A2A spec evolves mid-sprint | Targeting 1.0.0 stable; deliver as 0.1 binding (room to update) |
| `RISC0_DEV_MODE=0` proving too slow for CI | Use Risc0's Bonsai remote prover in CI; local builds use dev mode |
| Logos Core module-loading breaks across Basecamp versions | Reference implementation tested against specific `logos-basecamp` commit; documented in README |
| Reviewer unavailable / slow | Open PR with one full week of cushion before sprint end |
| Another submitter wins first | Daily polling of `lambda-prize` PRs starting Week 3 |
