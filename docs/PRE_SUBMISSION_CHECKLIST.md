# Pre-Submission Checklist

> Every item that must be true before opening the PR against `logos-co/lambda-prize`.
> Each item has an **acceptance criterion** (how you know it's done) and an **effort estimate**.
>
> Companion to [`SUBMISSION_CHECKLIST.md`](SUBMISSION_CHECKLIST.md) (strategy) and
> [`BUILD_PLAN.md`](BUILD_PLAN.md) (4-week sprint). This doc is the flat operational queue.

**Status as of 2026-05-25:** Phase 0 complete, Phase 1 Step 1.1 complete (repo public at https://github.com/warfield2016/logos-agent), CI running.

**Estimated remaining effort:** ~120-160 hours of focused work.

---

## ✅ Already done (no further action)

- [x] Rust workspace scaffolded, 5 crates, clean compile (Rust 1.85.0)
- [x] `cargo clippy --workspace -- -D warnings` clean
- [x] `cargo fmt --all --check` clean
- [x] 6 unit tests pass (5 spending-policy + 1 task-lifecycle)
- [x] cbindgen generates `target/include/agent_core.h`
- [x] 21 default skills registered (stubbed at the `Upstream` boundary)
- [x] FFI surface complete: `agent_init_runtime/create/destroy/invoke_skill/list_skills/status/free_string/pending_approvals`
- [x] Two URI-addressable A2A spec drafts in `spec/`
- [x] Skill SDK contract in `spec/skill-interface.md`
- [x] 6 strategy docs in `docs/`
- [x] 3 evaluator scripts in `scripts/` (verify-submission, verify-deployers, full-demo)
- [x] CI workflow (`.github/workflows/ci.yml`) with fmt+clippy+test+cdylib build
- [x] Dual MIT + Apache-2.0 licenses
- [x] CONTRIBUTING.md
- [x] Public GitHub repo with v0.0.1-scaffold tag, Issues + Discussions enabled

---

## Phase 1 — Spec broadcast (remaining: ~2 hours)

The cheap-and-durable standards-author positioning. Do these regardless of whether the module ships.

### 1.2 Logos forum post — A2A binding spec
- [ ] Subject line: *"Proposal: A2A Transport Binding over Logos Messaging (v0.1 draft)"*
- [ ] Post content drafted (markdown ready to paste)
- [ ] Posted to forum.logos.co (or equivalent)
- [ ] `@fryorcraken` and `@logos-messaging-team` tagged
- [ ] Link bookmarked for response monitoring
- **Acceptance:** post URL recorded in `docs/RECRUITMENT_TRACKER.md`
- **Effort:** 20 min

### 1.3 Logos forum post — LEZ Payment Extension spec
- [ ] Subject: *"Proposal: A2A LEZ Payment Extension (v0.1 draft)"*
- [ ] Posted; cross-references `a2a-x402` and AP2 in framing
- **Acceptance:** post URL recorded
- **Effort:** 15 min

### 1.4 Discord `#builder-hub` heads-up
- [ ] Brief post mentioning specs are out for review + repo public-draft
- [ ] Add to Friday `#dev-office-hours` agenda
- **Acceptance:** message link recorded
- **Effort:** 10 min

### 1.5 GitHub Discussion on `a2aproject/A2A`
- [ ] *"Custom transport binding for libp2p/Waku — looking for early reviewers"*
- [ ] Links to our binding doc; does NOT request inclusion in upstream (premature)
- **Acceptance:** discussion URL recorded
- **Effort:** 15 min

### 1.6 Repository polish (defer if low signal from above)
- [ ] Add repo topics: `rust`, `agent`, `a2a-protocol`, `logos`, `zero-knowledge`
- [ ] Add social-preview image (1200×630 PNG with project name + tagline)
- [ ] Pin one Discussion: "Roadmap and Build Plan"
- **Acceptance:** repo home page renders with topics + preview
- **Effort:** 30 min

**Phase 1 exit gate:** if fryorcraken or another Logos team member responds positively within 48 hours, proceed to Phase 2 with confidence. If crickets after 5 days, re-evaluate against OOBE.

---

## Phase 2 — Live module integration (remaining: ~40-50 hours)

### 2.1 Wallet integration (wallet-ffi)
- [ ] Add `wallet-ffi` git dep to `Cargo.toml` pinned to specific commit hash
- [ ] Replace `dispatcher::StubWallet` with `WalletFfiAdapter`
- [ ] Implement `WalletHandle::balance` → `wallet_ffi_get_balance`
- [ ] Implement `WalletHandle::send` → `wallet_ffi_transfer_shielded` (gated by spending policy)
- [ ] Implement `WalletHandle::history` → poll sequencer + cache
- [ ] Implement `WalletHandle::call_program` and `query_program`
- [ ] All wallet calls go through Tokio `spawn_blocking` (wallet-ffi is sync internally)
- [ ] Document the wallet-ffi calling convention in `docs/INTEGRATION.md`
- **Acceptance:** `cargo run --bin logos-agent invoke wallet.balance` returns a real number from a real testnet account
- **Effort:** 12-16 h

### 2.2 Storage integration
- [ ] Implement `StorageHandle` against `storage_module` via `LogosAPI` IPC
- [ ] Wire `storage.upload` → `uploadInit` → chunk loop → `uploadFinalize`
- [ ] Wire `storage.download` → `downloadChunks` → reassemble
- [ ] Wire `storage.list` and `storage.share`
- **Acceptance:** upload a 1 KB file, get a CID, download it, byte-compare matches
- **Effort:** 6-8 h

### 2.3 Messaging integration
- [ ] Implement `MessagingHandle` against `delivery_module` via `LogosAPI` IPC
- [ ] Wire `messaging.send` → `delivery_module.send(contentTopic, payload)`
- [ ] Subscribe loop: handle `messageReceived` events, route into pending-task queue
- [ ] Wire `messaging.join` and `messaging.create_group`
- **Acceptance:** two local agents exchange a message round-trip in < 5 seconds
- **Effort:** 6-8 h

### 2.4 Owner channel (CRITICAL — spec criterion)
- [ ] Implement `OwnerChannel::request_approval` → publish encrypted envelope to owner's NPK
- [ ] Subscribe to `/logos-agent/0.1/owner/<our_npk>/inbox`; parse `ApproveTx`/`DenyTx` replies
- [ ] Wire spending-policy gate: above-threshold → suspend tx → await reply → resume or reject
- [ ] Test: above-threshold tx → owner notified → reply approve → tx settles
- [ ] Test: above-threshold tx → owner replies deny → skill returns `ApprovalDenied`
- [ ] **Test: above-threshold tx → owner unreachable 5 min → tx NEVER submitted** ← spec criterion
- [ ] Persistent state: pending approvals survive process restart
- **Acceptance:** all 3 test scenarios pass in CI integration tests
- **Effort:** 8-10 h

### 2.5 CLI quickstart wizard (the recruitment funnel)
- [ ] Generate NPK/ISK keypair → `~/.logos-agent/identity.json`
- [ ] Call testnet faucet API → fund account
- [ ] Prompt for owner NPK (or generate via `logos-agent owner init`)
- [ ] Set default spending policy
- [ ] Publish AgentCard on discovery topic
- [ ] Print agent NPK + next steps (registry registration)
- [ ] Time-trial: stranger completes in ≤ 5 minutes on fresh VM
- [ ] Idempotent: re-running detects existing identity
- [ ] Every error message is actionable
- **Acceptance:** recorded video of fresh-VM run shows ≤ 5 min completion
- **Effort:** 6-8 h

### 2.6 LLM backend
- [ ] Wire `InferenceBackend` trait + concrete `LlamaCpp` impl using `llama-cpp-2` crate
- [ ] Add `Ollama` adapter (HTTP)
- [ ] Add `OpenAICompat` adapter (HTTP)
- [ ] Config-driven selection in `~/.logos-agent/config.toml`
- [ ] Default model path discovery via `~/.logos-agent/models/`
- **Acceptance:** all 3 backends return sensible completion for a test prompt
- **Effort:** 4-6 h

### 2.7 Failure isolation (spec criterion)
- [ ] Wrap each `Skill::invoke` in `catch_unwind` so panics don't crash runtime
- [ ] Add panic test: register panic-skill, invoke, verify other skills still work
- [ ] Add per-skill timeout (default 5 min, configurable)
- [ ] Timeout test: skill that sleeps forever → returns `Timeout`, runtime healthy
- **Acceptance:** 2 new tests pass; runtime survives intentional skill panic
- **Effort:** 3-4 h

### 2.8 Persistent K/V (state_get / state_put)
- [ ] Replace in-memory stubs with sled-backed persistent K/V
- [ ] Per-skill namespace prefix on keys
- [ ] Survives process restart
- **Acceptance:** `state_put` → restart → `state_get` returns same value
- **Effort:** 2-3 h

### Phase 2 exit gate
- [ ] All 21 skills are functional (no `Upstream` errors except `program.deploy` and A2A skills which land in Phase 3)
- [ ] `logos-agent quickstart` produces working agent in ≤ 5 min on fresh VM
- [ ] Owner approval flow tested in all 3 states (approve/deny/timeout)
- [ ] CI green with integration tests against local sequencer

---

## Phase 3 — A2A + recruitment (remaining: ~35-45 hours engineering + ongoing recruitment)

### 3.1 A2A discovery
- [ ] Implement `AgentCard::sign` (ed25519 over canonicalized card excluding signature field)
- [ ] Implement `agent.card` returning the signed card
- [ ] Publish card to `/logos-a2a/0.1/cards` on agent start
- [ ] Implement `agent.discover`: subscribe to discovery topic, parse cards, cache by NPK
- [ ] Verify signature on every incoming card; reject mismatches
- [ ] Card TTL: re-publish on configured interval (default 1h)
- **Acceptance:** two agents on different machines see each other within 30 seconds
- **Effort:** 6-8 h

### 3.2 A2A task lifecycle
- [ ] Implement client-side `agent.task`: build TaskRequest envelope, sign, publish to recipient inbox
- [ ] Implement server-side task receiver: subscribe to own inbox, validate sig, route to skill
- [ ] Status publish: `TaskStatus { task_id, seq, status }` on transitions
- [ ] Subscriber reorder buffer by `seq` (Waku is unordered)
- [ ] Terminal-state handling: COMPLETED/FAILED/CANCELED/REJECTED closes stream
- [ ] Replay support: on reconnect, subscriber catches up via Waku store protocol
- **Acceptance:** agent A invokes a skill on agent B end-to-end; result received on status/artifact topics
- **Effort:** 10-12 h

### 3.3 LEZ payment integration
- [ ] `Payment::Commit` flow: client transfers LEZ shielded, embeds tx_hash
- [ ] Server-side: provider checks tx_hash on-chain before WORKING transition
- [ ] `Payment::Receipt` on completion
- [ ] `Payment::Refund` on cancellation
- [ ] Test: free skill executes with no payment envelope
- [ ] Test: paid skill rejects until valid payment seen, then executes
- [ ] Settlement modes: shielded (default), transparent
- [ ] Escrow mode: lock funds in `task-escrow` program, release on completion attestation
- **Acceptance:** end-to-end paid task from discovery → payment → execution → receipt
- **Effort:** 8-10 h

### 3.4 LEZ programs
- [ ] Complete `programs/agent-registry`: `new_definition`, `register`, `touch`, `get`, `list_recent`, `count_by_team`
- [ ] Compile to RISC-V via Risc0; verify `RISC0_DEV_MODE=0` build succeeds
- [ ] Deploy to LEZ testnet; record program ID in `docs/DEPLOYED_PROGRAMS.md`
- [ ] Complete `programs/task-escrow`: `open`, `complete`, `cancel`, `dispute`, `resolve`
- [ ] Deploy to testnet; record program ID
- [ ] Wire `scripts/verify-deployers.sh` to query agent-registry live
- **Acceptance:** `scripts/verify-deployers.sh` returns non-placeholder output
- **Effort:** 8-10 h

### 3.5 Cross-framework interop demo (LangChain)
- [ ] Write `examples/langchain-client/` — Python A2A client
- [ ] 30-line demo: discovers Logos agent, requests task, pays in LEZ, gets result
- [ ] Add to `docs/EXAMPLES.md`
- [ ] Record video (saved for Phase 4)
- **Acceptance:** Python script runs from clean venv, completes round-trip in < 30s
- **Effort:** 4-6 h

### 3.6 Recruitment (runs in parallel with engineering — DO NOT defer)
- [ ] Day 1: Discord `#builder-hub` post per RECRUITMENT.md template
- [ ] Day 1: DM 5 personal candidates (other λPrize builders: mmlado, bristinWild, Gmin2, Tranquil-Flow, syafiqeil)
- [ ] Day 2: Forum cross-post
- [ ] Day 3: GitHub Discussion on `a2aproject/A2A` w/ standards angle
- [ ] Day 5: Status check — if <3 verified deployers, escalate bounty to $100 + activate backup pool
- [ ] Track all candidates in `docs/RECRUITMENT_TRACKER.md` (private; gitignored)
- [ ] Pay $50 USDT per verified deployer within 24h of registration
- **Acceptance:** `scripts/verify-deployers.sh` shows ≥ 5 unique non-team NPKs by end of Phase 3
- **Effort:** 6-10 h spread across the phase + $250 bounty budget

### Phase 3 exit gate
- [ ] Two agents (different machines) execute full A2A task with LEZ payment
- [ ] Both LEZ programs deployed on testnet with recorded program IDs
- [ ] ≥ 3 verified outside-team deployers on-chain
- [ ] LangChain interop demo recorded as MP4

---

## Phase 4 — Demos, docs, submission prep (remaining: ~30-40 hours)

### 4.1 Three use-case demo recordings

Each: narrated walkthrough, terminal visible, `RISC0_DEV_MODE=0` shown, ≤ 10 min.

#### Demo 1 — Privacy-preserving notary
- [ ] Script written
- [ ] Test run (no recording) — works end-to-end
- [ ] Recorded with narration
- [ ] Uploaded to YouTube (or equivalent persistent host)
- [ ] Linked in README + `docs/DEMOS.md` + embedded on Vercel site
- **Acceptance:** anyone can click the YouTube link and see proof generation in terminal
- **Effort:** 4 h

#### Demo 2 — Agent services marketplace
- [ ] Two agents on screen (split-screen or sequential)
- [ ] Card publication shown live
- [ ] LEZ payment shown on-chain
- [ ] Result delivered to caller
- **Effort:** 4 h

#### Demo 3 — Cross-framework interop (LangChain)
- [ ] 30-line Python script visible
- [ ] Logos agent discovered without modification
- [ ] Task completes, payment settles
- **Effort:** 3 h

### 4.2 Final recruitment push
- [ ] HN post: *"Show HN: Sovereign AI agents with native crypto payments, no central server"*
- [ ] Twitter/X thread with demo GIF + arch diagram
- [ ] Final outreach to candidates still in funnel
- [ ] Pay all remaining bounties
- [ ] Lock final deployer count (target ≥ 5, aim 7 for safety margin)
- **Acceptance:** verify-deployers.sh shows ≥ 5 unique non-team NPKs
- **Effort:** 4 h

### 4.3 Documentation finalization
- [ ] `DEPLOYMENT.md` — verified end-to-end on Linux AND macOS clean VMs
- [ ] `SECURITY_MODEL.md` — finalized post-implementation
- [ ] `docs/CU_COSTS.md` — per-skill CU costs measured on testnet (table format)
- [ ] `docs/OWNER_GUIDE.md` — day-to-day operation guide
- [ ] `docs/SKILL_SDK_GUIDE.md` — third-party skill author quickstart
- [ ] `docs/EXAMPLES.md` — directory of usage examples
- [ ] All Phase 1-3 `<TBD>` placeholders resolved
- [ ] README updated to "Status: Ready for submission"
- **Acceptance:** `scripts/verify-submission.sh` reports 0 WARN
- **Effort:** 6-8 h

### 4.4 Reproducible demo script
- [ ] `scripts/full-demo.sh` performs real assertions (not placeholders)
- [ ] Boots sequencer with `RISC0_DEV_MODE=0`
- [ ] Installs module, runs quickstart
- [ ] Executes skill touching wallet + storage + messaging + LEZ program
- [ ] Tested on clean macOS VM
- [ ] Tested on clean Ubuntu VM
- [ ] Total runtime documented (target ≤ 15 min)
- **Acceptance:** clean-clone clean-VM execution succeeds on both OSes
- **Effort:** 4-6 h

### 4.5 Vercel demo site
- [ ] `gh repo create logos-agent-site --public`
- [ ] Next.js or Astro static site:
  - Hero + value prop + install command
  - Embedded demo videos
  - **Live Agent Directory** reading from on-chain `agent-registry`
  - Link to specs + GitHub repo
- [ ] `vercel --prod` deployment
- [ ] Custom domain if available
- **Acceptance:** Vercel URL renders all three demos + live deployer count from chain
- **Effort:** 6-8 h

### 4.6 crates.io publication
- [ ] `agent-skill-sdk` published as v0.1.0 to crates.io
- [ ] README on crates.io renders correctly
- [ ] `cargo install logos-agent` works from crates.io (publish `agent-cli` as `logos-agent` bin crate)
- **Acceptance:** stranger can `cargo install logos-agent` and run the binary
- **Effort:** 2 h

### 4.7 Final spec polish
- [ ] Both A2A spec drafts incorporate any feedback from Phase 1 broadcasts
- [ ] Version-stamp specs as `0.1.0` (frozen for v0.1 of the implementation)
- [ ] Add `CHANGELOG.md` entry for v0.1.0
- **Acceptance:** specs and code agree on every field name and type
- **Effort:** 2-3 h

---

## Phase 5 — Pre-submission dress rehearsal (24 hours before opening PR)

This phase is non-negotiable. Beach-Bum likely skipped it; you cannot.

### 5.1 Clean-environment test
- [ ] Spin up fresh Ubuntu VM (never used for this project)
- [ ] Clone from GitHub (not local working dir)
- [ ] Follow README top-to-bottom step-by-step
- [ ] Run `scripts/full-demo.sh`
- [ ] Run `scripts/verify-submission.sh`
- [ ] Document every place where prior knowledge was needed → fix docs
- **Acceptance:** stranger could follow README without context
- **Effort:** 4-6 h

### 5.2 Spec compliance audit (LP-0008 success criteria walkthrough)

**Functionality (11 items):**
- [ ] Module loads in Logos Core alongside wallet/storage/messaging
- [ ] Agent has own shielded LEZ account
- [ ] Single CLI deploy command on remote node
- [ ] Real-time owner ↔ agent chat via Logos Messaging
- [ ] Spending threshold mechanism (above/below)
- [ ] All 21 default skills implemented + documented
- [ ] A2A-compatible coordination (Cards + lifecycle + transport binding documented)
- [ ] Two agents discover/execute/transfer LEZ payment without owner intervention
- [ ] ≥ 3 illustrative use cases demoed end-to-end on testnet
- [ ] ≥ 5 outside-team deployments on testnet
- [ ] Full documentation + clean public repo

**Usability (2 items):**
- [ ] Documented skill interface (SDK)
- [ ] Owner-facing interface accessible from Basecamp

**Reliability (3 items):**
- [ ] Recovery from transient failures (network, node restart)
- [ ] Above-threshold txs that fail owner notification are NOT executed
- [ ] Skill failures isolated (panic in one ≠ crash module)

**Performance (1 item):**
- [ ] Per-skill CU cost table on LEZ testnet

**Supportability (6 items):**
- [ ] Deployed on LEZ devnet/testnet
- [ ] E2E integration tests against sequencer in CI
- [ ] CI green on default branch
- [ ] README documents end-to-end usage
- [ ] Reproducible demo script with `RISC0_DEV_MODE=0`
- [ ] Recorded video demo showing terminal output proving `RISC0_DEV_MODE=0`

**If any of the 23 items above is unchecked → DO NOT SUBMIT.** Use remaining time to fix.

- **Effort:** 4 h (the audit itself; fixes vary)

### 5.3 PR description drafted
- [ ] Lead with 5-deployer verification (link to script output)
- [ ] Three demo video links with key-moment timestamps
- [ ] Spec docs section (standards contribution framing)
- [ ] Architecture diagram embedded
- [ ] Per-skill CU cost table
- [ ] Tagged: @fryorcraken + any other Logos team members from Phase 1 responses
- **Acceptance:** PR body draft is a complete, scannable document
- **Effort:** 2 h

### 5.4 Last competitive check
- [ ] `gh pr list --repo logos-co/lambda-prize --search "LP-0008"`
- [ ] If competitor PR opened in last 7 days: evaluate their 5-deployer count
- [ ] If competitor has ≥ 5 verified deployers AND merged-eligible: regroup
- [ ] Check `Beach-Bum/Agora` for any reopened activity
- **Acceptance:** confirmed no merge-eligible competitor in flight
- **Effort:** 30 min

---

## Phase 6 — Submission day

### 6.1 Open the PR
- [ ] Fork `logos-co/lambda-prize`
- [ ] Create branch `solution/lp-0008-warfield`
- [ ] Add `solutions/LP-0008.md` with PR-description content
- [ ] Open PR titled: *"Solution: LP-0008 — Logos Autonomous AI Agent Module"*
- [ ] Post PR link in Logos Discord `#builder-hub` for visibility
- **Acceptance:** PR URL recorded
- **Effort:** 1 h

### 6.2 Same-day support window (4 hours)
- [ ] Stay online and responsive
- [ ] Respond to reviewer questions within 30 min
- [ ] Fix any flagged issues within 24 hours (not a week)
- **Acceptance:** zero unanswered reviewer comments
- **Effort:** 4 h reserved

### 6.3 Monitor
- [ ] Daily PR check for reviewer activity
- [ ] If 7 days no response: polite comment tagging @fryorcraken
- [ ] All change requests treated as priority-zero
- **Acceptance:** PR merged
- **Effort:** ongoing, low

---

## Win conditions (must ALL be true)

1. ✅ PR opened with complete `solutions/LP-0008.md`
2. ✅ `scripts/verify-submission.sh` all-green from clean clone
3. ✅ ≥ 5 outside-team deployers verifiable on-chain
4. ✅ 3 demo videos public, each showing `RISC0_DEV_MODE=0` in terminal
5. ✅ No competing PR merged first
6. ✅ Reviewer marks all success criteria green
7. ✅ PR merged
8. ✅ USDT received

---

## Estimated total remaining effort

| Phase | Hours | Money |
|-------|-------|-------|
| 1 (broadcast) | 2 | — |
| 2 (live integration) | 40-50 | — |
| 3 (A2A + recruitment) | 35-45 | $250 bounties |
| 4 (demos + docs + Vercel) | 30-40 | <$20 hosting |
| 5 (dress rehearsal) | 8-10 | — |
| 6 (submission) | 5+ ongoing | — |
| **TOTAL** | **120-160h** | **~$270** |

At $1,200 prize, this is **$7.50–10/hr** if you cleared every gate solo. Standards-author positioning, spec-broadcast feedback, and reused architecture (for Pulse) are the only justifications for taking this rate. See [`COMPETITIVE_INTEL.md`](COMPETITIVE_INTEL.md) §"Strategic differentiators" for the offsetting value.

---

## Abort gates (when to stop)

| Trigger | Action |
|---------|--------|
| Phase 1 broadcasts get zero response in 5 days | De-rate the standards play; module + recruitment is the only win path |
| Phase 2 exit gate fails by end of Week 2 | Slip Phase 3 by exact delay; do NOT enter recruitment with broken software |
| Phase 3 has < 3 deployers by EOW3 | Escalate to $100/deployer + activate backup pool, or extend sprint 1 week |
| Competitor PR opens during Phase 3-4 with ≥ 5 deployers | Step back, don't burn a submission slot |
| Higher-rate work (Pulse, Godot MWA, OOBE) becomes time-critical | Honor the rate analysis; pause LP-0008 |
| fryorcraken signals he won't review for X weeks | Don't open PR; wait |
