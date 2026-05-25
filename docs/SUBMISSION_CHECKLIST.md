# LP-0008 Submission Checklist — Path to Win

**Goal:** First merged Large prize in `logos-co/lambda-prize`. Win = PR merged + $1,200 USDT paid.

**Reviewer:** fryorcraken (Logos / Waku). FCFS evaluation. Up to 3 submissions per builder, max one per week.

**Hard constraint:** Evaluators clone the repo and run the demo script from a clean environment. The script must succeed without modification. If anything works "because Warfield ran it on his Mac with these env vars set" — you lose.

---

## Phase 0 — Pre-flight (status: ✅ complete 2026-05-24)

- [x] Rust workspace at `/Users/warfield/Python experiments/logos-agent/` compiles clean
- [x] 21 default skill stubs registered in dispatcher
- [x] `agent-skill-sdk` v0.1 public crate with `Skill` trait + handles + macro
- [x] `agent-core` cdylib with `extern "C" agent_*` FFI surface
- [x] cbindgen generates `target/include/agent_core.h` (for auto-generated Qt shim)
- [x] `flake.nix` wired to `logos-co/logos-module-builder`
- [x] `metadata.json` with `interface: "universal"` codegen
- [x] `SpendingPolicy` with per-token caps + 5 passing unit tests
- [x] `TaskStore` lifecycle state machine + 1 passing unit test
- [x] A2A binding skeleton with topic schema and envelope types
- [x] Two URI-addressable spec drafts (`a2a-logos-messaging-binding.md`, `lez-payment-extension.md`)
- [x] Strategy docs (BUILD_PLAN, RECRUITMENT, COMPETITIVE_INTEL, DEPLOYMENT, SECURITY_MODEL)

---

## Phase 1 — Public draft + spec broadcast (Day 0, ~2 hours)

Do this BEFORE Week 2. Two purposes: (a) standards-author positioning even if module doesn't ship, (b) start the recruitment top-of-funnel early.

### Step 1.1 — Push GitHub repo as public draft (30 min)

- [ ] `gh repo create logos-agent --public --description "LP-0008: Autonomous AI Agent Module for Logos"`
- [ ] Push current scaffold; tag `v0.0.1-scaffold`
- [ ] Replace all `<TBD>` placeholders in `Cargo.toml`, `README.md`, SDK README with actual GitHub URL
- [ ] Add `Status: Week 1 scaffold — not yet functional` badge to top of README
- [ ] Enable GitHub Issues + Discussions
- [ ] Add `LICENSE` files (dual MIT + Apache-2.0)
- [ ] Add `.github/workflows/ci.yml` for `cargo check --workspace` + `cargo test --workspace`

**Acceptance:** repo is publicly viewable; CI green on default branch; SDK installable via `cargo add agent-skill-sdk --git ...`.

### Step 1.2 — Publish A2A binding spec to Logos forum (15 min)

- [ ] Cross-post `spec/a2a-logos-messaging-binding.md` to forum.logos.co (or equivalent)
- [ ] Subject line: *"Proposal: A2A Transport Binding over Logos Messaging (v0.1 draft)"*
- [ ] Tag `@fryorcraken` and `@logos-messaging-team` for review
- [ ] Link back to GitHub spec file for canonical version

**Acceptance:** post live, link in bookmarks for daily monitoring.

### Step 1.3 — Publish LEZ Payment Extension spec to Logos forum (15 min)

- [ ] Cross-post `spec/lez-payment-extension.md`
- [ ] Subject: *"Proposal: A2A LEZ Payment Extension (v0.1 draft)"*
- [ ] Reference `a2a-x402` and AP2 in framing for evaluator context

### Step 1.4 — Drop a Discord/Telegram heads-up (15 min)

- [ ] Post in `#builder-hub` (Logos Discord): *"I'm working on LP-0008 and have published two A2A spec drafts for the Logos community to review. Repo is public-draft, feedback welcome."*
- [ ] Brief mention in `#dev-office-hours` agenda for next Friday

### Step 1.5 — Filed under "free standards credibility" (15 min)

- [ ] Open a GitHub Discussion on `a2aproject/A2A`: *"Custom transport binding for libp2p/Waku — looking for early reviewers"*
- [ ] Link to our binding doc; do not request inclusion in upstream yet (premature)

**Gate to proceed:** if fryorcraken or any Logos team member responds positively within 48 hours → strong signal, proceed to Phase 2. If crickets after 5 days → re-evaluate vs. OOBE.

---

## Phase 2 — Week 2: Live module integration (target: minimum viable deployable)

Goal: by end of Week 2, an outside person can run `cargo install logos-agent && logos-agent quickstart` and end up with a working agent on testnet.

### Step 2.1 — Wallet integration

- [ ] Add `wallet-ffi` as build dep (pin to specific commit hash in `flake.nix`)
- [ ] Replace `dispatcher::StubWallet` with `WalletFfiAdapter` calling real `wallet_ffi_*` functions
- [ ] Implement `WalletHandle::balance` → `wallet_ffi_get_balance`
- [ ] Implement `WalletHandle::send` → `wallet_ffi_transfer_shielded` (gated by spending policy)
- [ ] Implement `WalletHandle::history` → poll sequencer + cache
- [ ] Implement `WalletHandle::call_program` and `query_program`
- [ ] Integration test: spin up local sequencer (`standalone` feature), create account, transfer to self, check balance
- [ ] Document the wallet-ffi calling convention in `docs/INTEGRATION.md`

**Acceptance:** `logos-agent invoke wallet.balance` returns a real number from a real testnet account.

### Step 2.2 — Storage integration

- [ ] Implement `StorageHandle` against `storage_module` via `LogosAPI->module("storage_module")` IPC
- [ ] Wire `storage.upload` → `uploadInit` → chunk loop → `uploadFinalize`
- [ ] Wire `storage.download` → `downloadChunks` → reassemble
- [ ] Integration test: upload a file, retrieve CID, download, verify hash matches

**Acceptance:** `logos-agent invoke storage.upload '{"path": "./README.md"}'` returns a CID; downloading from the CID reproduces the file byte-for-byte.

### Step 2.3 — Messaging integration

- [ ] Implement `MessagingHandle` against `delivery_module` via LogosAPI IPC
- [ ] Wire `messaging.send` → `delivery_module.send(contentTopic, payload)`
- [ ] Subscribe loop: handle `messageReceived` events and route into pending-task queue
- [ ] Integration test: two agents on localhost exchange a message round-trip

**Acceptance:** `logos-agent invoke messaging.send` results in the recipient seeing the message in their Basecamp UI.

### Step 2.4 — Owner channel (the most important UX surface)

- [ ] Implement `OwnerChannel::request_approval` → publish encrypted envelope to owner's NPK
- [ ] Implement `OwnerChannel` polling: subscribe to `/logos-agent/0.1/owner/<our_npk>/inbox`, parse `ApproveTx`/`DenyTx` replies
- [ ] Wire spending-policy gate: if `OwnerApprovalRequired`, suspend the tx, await reply, resume or reject
- [ ] Test: above-threshold tx → owner-side notification → reply approve → tx settles on-chain
- [ ] Test: above-threshold tx → owner replies deny → skill returns `ApprovalDenied`
- [ ] Test: above-threshold tx → owner unreachable for 5 min → skill returns `Timeout`, tx NEVER submitted

**Acceptance:** the third test above is the hardest and most important — failure here is a spec criterion miss. The agent MUST NOT execute above-threshold txs if owner notification can't be confirmed.

### Step 2.5 — CLI quickstart wizard (the recruitment funnel)

- [ ] `logos-agent quickstart` implements:
  1. Generate NPK/ISK keypair → write to `~/.logos-agent/identity.json`
  2. Call testnet faucet API → fund account
  3. Prompt for owner NPK (or generate one via separate `logos-agent owner init`)
  4. Set sensible default spending policy (`per_tx=10 LEZ, per_day=100 LEZ`)
  5. Publish AgentCard on discovery topic
  6. Print next steps + the agent's NPK (for registration in `agent-registry`)
- [ ] Time-trial: stranger with no context completes wizard in ≤ 5 minutes
- [ ] Idempotent: re-running wizard is safe (detects existing identity)
- [ ] Error messages are actionable: every error path tells the user exactly what to do

**Acceptance:** record yourself doing this on a fresh VM. Anything over 5 minutes is a fail and blocks Phase 3.

### Step 2.6 — LLM backend

- [ ] Wire `InferenceBackend` trait + concrete `LlamaCpp` impl
- [ ] Add `Ollama` and `OpenAICompat` adapters
- [ ] Config-driven backend selection in `~/.logos-agent/config.toml`
- [ ] Test each adapter returns sensible completion

**Acceptance:** `logos-agent invoke <any-skill-that-uses-inference>` produces a real LLM response.

### Step 2.7 — Failure isolation

- [ ] Wrap each `Skill::invoke` call in `catch_unwind` so panicking skills don't crash the runtime
- [ ] Add panic test: register a skill that panics, invoke it, verify other skills still work
- [ ] Add timeout: any skill invocation auto-aborts after configurable timeout (default 5 min)

**Acceptance:** spec criterion "Skill failures are isolated" verifiably true.

### Phase 2 Exit Gate (end of Week 2)

Required to proceed:
- [ ] All 21 skills functional (not stubbed) against testnet
- [ ] `logos-agent quickstart` produces working agent in ≤ 5 min on fresh VM
- [ ] Owner approval flow tested in all three states (approve / deny / timeout)
- [ ] At least 3 internal demo recordings of below-threshold and above-threshold flows
- [ ] CI green: `cargo test --workspace` passes, including integration tests against local sequencer

If any of these is red → push Phase 3 start by exactly the number of days needed; do not enter recruitment with broken software.

---

## Phase 3 — Week 3: A2A + recruitment kickoff

### Step 3.1 — A2A discovery (publish + fetch)

- [ ] Implement `AgentCard::sign` (ed25519 over canonicalized card)
- [ ] Implement `agent.card` returning the agent's signed card
- [ ] Implement card publishing to `/logos-a2a/0.1/cards`
- [ ] Implement `agent.discover`: subscribe to discovery topic, parse incoming cards, cache locally
- [ ] Verify signature on every incoming card; reject mismatches
- [ ] Test: two agents on different machines see each other's cards within 30 seconds

### Step 3.2 — A2A task lifecycle

- [ ] Implement `agent.task` (client side): build TaskRequest envelope, sign, publish to recipient's inbox
- [ ] Implement task receiver (server side): subscribe to own inbox, validate signature, route to skill
- [ ] Status updates: provider publishes `TaskStatus { task_id, seq, status }` to status topic
- [ ] Subscriber reorder buffer (Waku is unordered) — by `seq`
- [ ] Terminal states (COMPLETED/FAILED/CANCELED/REJECTED) close the stream
- [ ] Test: agent A invokes a skill on agent B end-to-end; A receives the result over the status/artifact topics

### Step 3.3 — LEZ payment integration

- [ ] Implement `Payment::Commit` flow: client transfers LEZ shielded, embeds tx_hash in envelope
- [ ] Server-side verification: provider checks tx_hash on-chain before beginning execution
- [ ] `Payment::Receipt` on completion
- [ ] `Payment::Refund` on cancellation
- [ ] Test: free skill (`price_lez = None`) executes without payment envelope
- [ ] Test: paid skill (`price_lez = 100`) rejects until valid payment seen, then executes

### Step 3.4 — LEZ programs

- [ ] Complete `programs/agent-registry/`: `new_definition`, `register(agent_npk, owner_sig, manifest_cid)`, `touch`, `get`, `list_recent`
- [ ] Compile to RISC-V via Risc0; verify `RISC0_DEV_MODE=0` build succeeds
- [ ] Deploy to LEZ testnet; record program ID in repo
- [ ] Complete `programs/task-escrow/`: `open`, `complete`, `cancel`, `dispute`, `resolve`
- [ ] Deploy to testnet; record program ID

### Step 3.5 — Cross-framework interop demo (LangChain)

- [ ] Write `examples/langchain-client/` — Python A2A client (using `qntx/ra2a` or hand-rolled)
- [ ] Demo: 30-line LangChain agent discovers our Logos agent via card, requests a task, pays in LEZ, receives result
- [ ] This is non-trivial differentiator vs Beach-Bum (his demos were all intra-Agora)
- [ ] Record video

### Step 3.6 — Recruitment kickoff (CRITICAL — see RECRUITMENT.md)

Run in parallel with engineering above. Do NOT delay these to Week 4.

- [ ] Day 1: Post in Logos Discord `#builder-hub` per template in RECRUITMENT.md
- [ ] Day 1: DM 5 personal candidates (other λPrize builders)
- [ ] Day 2: Cross-post to Logos forum
- [ ] Day 3: Post in `a2aproject/A2A` GitHub Discussions (standards angle)
- [ ] Day 5: Status check — if <3 verified deployers, escalate to backup pool
- [ ] Track all candidates in `docs/RECRUITMENT_TRACKER.md` (private; not committed to git)
- [ ] Build `scripts/verify-deployers.sh` that queries `agent-registry` on testnet and reports unique NPKs

### Phase 3 Exit Gate (end of Week 3)

- [ ] Two agents (different machines) execute full A2A task with LEZ payment
- [ ] Both LEZ programs deployed on testnet
- [ ] `agent-registry` has at least **3 verified outside-team registrations**
- [ ] LangChain interop demo recorded

If <3 deployers by EOW3 → recruitment is failing; consider escalation (offer $100 instead of $50, personal asks, etc.) before continuing.

---

## Phase 4 — Week 4: Demos, docs, submission

### Step 4.1 — Three use-case demo recordings

Each demo: narrated walkthrough, terminal visible showing `RISC0_DEV_MODE=0`, full E2E.

- [ ] **Demo 1 — Privacy-preserving notary:** owner sends doc → agent timestamps, stores on Logos Storage, records CID on LEZ → returns content address. Show terminal proof generation.
- [ ] **Demo 2 — Agent services marketplace:** agent A advertises translation skill at price N LEZ → agent B discovers, sends task, pays → A delivers translated text. Show on-chain payment + signed Cards.
- [ ] **Demo 3 — Cross-framework interop (LangChain):** 30-line LangChain client discovers Logos agent, requests task, pays in LEZ, receives result. Show this works without modifying our agent.

Each video: ≤ 10 minutes, hosted on YouTube + embedded in README + embedded on Vercel landing page.

### Step 4.2 — Final recruitment push

- [ ] Day 1-2: HN + Twitter posts (per RECRUITMENT.md templates)
- [ ] Day 3: Final outreach to candidates in funnel who haven't deployed yet
- [ ] Day 5: Cutoff for deployer count
- [ ] Pay all bounties within 24 hours of registration
- [ ] Verify on-chain registry shows ≥ 5 unique non-team NPKs

### Step 4.3 — Documentation finalization

- [ ] `DEPLOYMENT.md` — complete deploy walkthrough verified on Linux + macOS
- [ ] `SECURITY_MODEL.md` — finalized w/ what agent can/cannot do
- [ ] Per-skill CU cost table from real testnet measurements (`docs/CU_COSTS.md`)
- [ ] `OWNER_GUIDE.md` — how to operate the agent day-to-day
- [ ] `SKILL_SDK_GUIDE.md` — how third parties add skills (link to crates.io publish)
- [ ] All `<TBD>` placeholders resolved

### Step 4.4 — Reproducible demo script

- [ ] Write `./scripts/full-demo.sh` that:
  1. Boots local sequencer with `RISC0_DEV_MODE=0`
  2. Installs the module
  3. Runs `logos-agent quickstart`
  4. Executes one skill that touches wallet + storage + messaging + LEZ program
  5. Asserts success
- [ ] Test from a clean macOS VM
- [ ] Test from a clean Ubuntu VM
- [ ] Time the run; document expected duration

### Step 4.5 — Vercel demo site

- [ ] `gh repo create logos-agent-site --public`
- [ ] Next.js or Astro static site with:
  - Hero + value prop + install command
  - Embedded demo videos (Demo 1/2/3)
  - **Live Agent Directory** reading from on-chain `agent-registry`
  - Link to spec docs + GitHub repo
- [ ] Deploy to Vercel: `vercel --prod`
- [ ] Add custom domain if available (e.g., `logos-agent.dev`)

### Step 4.6 — Verification script for evaluators

Critical: the evaluator must be able to verify success criteria mechanically.

- [ ] `./scripts/verify-submission.sh` outputs a pass/fail report:
  ```
  ✓ Module loads in Logos Core
  ✓ Agent has shielded LEZ account
  ✓ All 21 skills registered
  ✓ A2A AgentCard valid + signed
  ✓ 7 outside-team deployers registered on-chain
  ✓ 3 demo recordings linked
  ✓ CI green
  ✓ RISC0_DEV_MODE=0 verified in last 24h
  ```
- [ ] Each ✓ is machine-checked; failure modes clearly explained

---

## Phase 5 — Pre-submission dress rehearsal (24 hours before opening PR)

**Do this exactly 24h before submitting. Treat as a real evaluator simulation.**

### Step 5.1 — Clean-environment test

- [ ] Spin up a fresh Ubuntu VM you've never used for this project
- [ ] Clone the repo from GitHub (do NOT copy your local working dir)
- [ ] Follow README from top to bottom, step by step
- [ ] Run `./scripts/full-demo.sh`
- [ ] Run `./scripts/verify-submission.sh`
- [ ] Note every place where you needed prior knowledge — fix the docs

### Step 5.2 — Spec compliance audit

Walk through every line of the LP-0008 success criteria. Mark each:

- [ ] Functionality: 11 items — every one verified by a script
- [ ] Usability: 2 items — SDK docs + Basecamp UI accessible
- [ ] Reliability: 3 items — failure isolation, owner unreachable, retry tests
- [ ] Performance: 1 item — CU cost table populated
- [ ] Supportability: 6 items — testnet deployed, CI green, README, demo script, video, reproducibility

If ANY single item is unchecked → do not submit. Use remaining time to fix.

### Step 5.3 — Reviewer-friendly PR description

Draft the PR body now, not at submission time:

- [ ] Lead with the 5-deployer verification: 1-line link to verification script output
- [ ] Then the 3 demo videos with timestamps for key moments
- [ ] Then the spec docs (positioning as standards contribution)
- [ ] Then the architecture diagram
- [ ] Close with the per-skill CU cost table
- [ ] Tag fryorcraken + relevant Logos team members

### Step 5.4 — Last competitive check

- [ ] `gh pr list --repo logos-co/lambda-prize --search "LP-0008"`
- [ ] If another submission opened in last 7 days, evaluate: do I beat it on the 5-deployer criterion? If yes, submit immediately. If no, regroup.
- [ ] Check Beach-Bum's repo for any update activity in last 30 days

---

## Phase 6 — Submission day

### Step 6.1 — Open the PR

- [ ] Fork `logos-co/lambda-prize`
- [ ] Create branch `solution/lp-0008-<your-handle>`
- [ ] Add `solutions/LP-0008.md` with the PR-description content
- [ ] Open PR with title: `Solution: LP-0008 — Logos Autonomous AI Agent Module`
- [ ] Post PR link in `#builder-hub` (Logos Discord) for visibility

### Step 6.2 — Same-day support

- [ ] Stay online for 4 hours after opening PR to respond to immediate questions
- [ ] If reviewer asks clarification, respond within 30 minutes
- [ ] If reviewer points out a fail, fix and push within 24 hours (don't wait a week)

### Step 6.3 — Monitor

- [ ] Check PR daily for reviewer activity
- [ ] If 7 days pass with no response, comment politely tagging fryorcraken
- [ ] If reviewer requests changes, treat as priority-zero work (don't let velocity stall)

---

## Post-merge

- [ ] Pay all deployer bounties (if not done already)
- [ ] Update README to "Status: Submitted ✓ Merged ✓ Paid $1,200 USDT"
- [ ] Thank deployers individually
- [ ] Cross-publish the binding spec to A2A community as merged reference implementation
- [ ] Update Logos forum threads with link to merged PR
- [ ] Update CLAUDE memory: `LP-0008 won 2026-MM-DD; standards-author position secured`
- [ ] Repurpose ~60% of code for Pulse trading-terminal skill SDK

---

## Risk register & abort gates

| Trigger | Decision |
|---------|----------|
| Phase 1 spec posts get no response in 5 days | Continue but de-rate the "standards play" value; module + recruitment becomes the only win path |
| Phase 2 exit gate fails (Week 2 EOD) | Slip Phase 3 by exact delay; do NOT enter recruitment with broken software |
| Phase 3 has <3 deployers by EOW3 | Escalate bounty to $100/deployer; activate personal backup pool; consider extending sprint by 1 week |
| Another submission opens during Phase 3-4 | Evaluate their 5-deployer count vs ours; if they have ≥5 → step back and don't burn a submission slot |
| Beach-Bum reopens his PR | Same evaluation; consider whether ours wins on completeness |
| fryorcraken signals he won't review for X weeks | Don't open PR; wait |
| Personal time conflicts with higher-rate work (Pulse, Godot, OOBE) | Honor the rate analysis; pause LP-0008 |

## Time budget summary

| Phase | Duration | Effort | Critical path |
|-------|----------|--------|--------------|
| 0 (done) | — | — | — |
| 1 (today) | 2h | low | spec posts, public repo |
| 2 | 5-7 days | high | wallet-ffi + quickstart |
| 3 | 5-7 days | high | A2A + recruitment in parallel |
| 4 | 5-7 days | medium | demos + final recruitment |
| 5 | 24h | high | dress rehearsal, no shortcuts |
| 6 | 1 day | medium | PR + visibility |

**Total ~3-4 weeks of focused effort, of which ~30% is non-engineering (recruitment, docs, demos).**

---

## Win conditions (single source of truth)

We win iff ALL of:

1. ✅ PR is opened in `logos-co/lambda-prize` with a complete `solutions/LP-0008.md`
2. ✅ `./scripts/verify-submission.sh` outputs all-green from a clean clone
3. ✅ ≥5 outside-team deployers verifiable on-chain via `agent-registry`
4. ✅ 3 demo videos publicly hosted, each showing `RISC0_DEV_MODE=0` in terminal
5. ✅ No competing PR has merged first
6. ✅ fryorcraken (or designated reviewer) marks all success criteria green
7. ✅ PR merged
8. ✅ USDT received

Anything less is a loss. Plan for all eight.
