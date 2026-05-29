# Win Path — Criterion → Action Mapping

> The evaluator's checklist on the left; our exact action on the right.
> If every action listed is complete, every criterion is satisfied. Submission becomes win-eligible.
>
> Companion to `SUBMISSION_CHECKLIST.md` (strategy) and `PRE_SUBMISSION_CHECKLIST.md` (operational).
> This doc is the **dependency-ordered execution plan**.

**Current score:** 1 / 23 satisfied, 4 / 23 partial, 18 / 23 not yet.

---

## How to read this

Each row is one LP-0008 success criterion (numbered to match the prize text). The "Flip action" column is the exact thing(s) needed to move it to ✅. The "Blocks/Blocked by" column shows the dependency graph.

Items marked **🔑** are critical-path: every other action either depends on them or runs in parallel.

---

## Functionality (11 items)

| # | Criterion | Current | Flip action(s) | Hours | Blocked by |
|---|-----------|---------|----------------|-------|-----------|
| 1 | Module loads in Logos Core alongside wallet/storage/messaging | ❌ | (a) `nix build .#module` produces .so/.dylib; (b) copy to Basecamp plugins dir; (c) confirm Basecamp loads it | 4 | #2, #3 (needs working module to load) |
| 2 | Agent has own shielded LEZ account; sends/receives tokens | ❌ | **🔑** Link `wallet-ffi` git dep; implement `WalletFfiAdapter`; replace `StubWallet`; integration test sends real tx | 12-16 | — (bedrock) |
| 3 | Single CLI deploy command (Logos Core headless) | ❌ | Implement `logos-agent quickstart` end-to-end: identity → faucet → owner config → policy → card publish | 6-8 | #2, #4 |
| 4 | Real-time owner ↔ agent chat via Logos Messaging | ❌ | **🔑** Implement `MessagingHandle` via delivery_module IPC; wire `OwnerChannel::request_approval` to publish + subscribe to inbox topic | 8-10 | — (parallel to #2) |
| 5 | Spending threshold holds above-threshold txs for approval | ⚠️ | Wire `SpendingPolicy::evaluate` → suspend tx → await owner reply → resume or reject. Tests for approve/deny/timeout paths | 4-6 | #2, #4 |
| 6 | All 21 default skills implemented + documented | ❌ | Replace `Err(Upstream(...))` in 18 of 21 skill bodies; wire to live handles | 12-14 | #2, #4, storage IPC |
| 7 | A2A coordination is A2A-compatible (Card + lifecycle + documented binding) | ⚠️ | Implement card signing, publication, subscriber; round-trip test with two agents | 6-8 | #2, #4 |
| 8 | Two agents discover, execute task, transfer LEZ — no owner intervention | ❌ | Implement `agent.task` request/receive, status streaming, LEZ payment commit/receipt, terminal-state handling | 14-18 | #7, #18 |
| 9 | ≥ 3 illustrative use cases demoed end-to-end on testnet | ❌ | Record Demo 1 (notary), Demo 2 (marketplace), Demo 3 (LangChain interop) | 11 | all functional items |
| 10 | ≥ 5 agents deployed by parties outside submitting team | ❌ | **🔑** Recruit via Discord/forum/HN + $50 bounty/deployer; verify via `agent-registry` LEZ program | 6-10 + $250 | #3, #18 (registry deployed) |
| 11 | Full documentation + clean public repo | ⚠️ | Finalize OWNER_GUIDE, SKILL_SDK_GUIDE, CU_COSTS; resolve all `<TBD>`; update README to "ready for submission" | 4-6 | all other items done |

## Usability (2)

| # | Criterion | Current | Flip action(s) | Hours | Blocked by |
|---|-----------|---------|----------------|-------|-----------|
| 12 | Documented skill interface that 3rd parties can use | ⚠️ | Add `examples/echo-skill/` working example using `agent-skill-sdk` from crates.io; publish `agent-skill-sdk = "0.1.0"` to crates.io | 3-4 | — |
| 13 | Owner-facing interface accessible from Basecamp | ❌ | Build QML view (chat pane + pending-approvals widget); register as `ui_qml` module; test in Basecamp | 8-12 | #4 |

## Reliability (3)

| # | Criterion | Current | Flip action(s) | Hours | Blocked by |
|---|-----------|---------|----------------|-------|-----------|
| 14 | Recovers from transient failures (network, restart) | ❌ | Add retry/backoff on transient wallet-ffi and delivery_module errors; persist pending tasks via sled; reload-on-restart test | 4-6 | #2, #4 |
| 15 | Above-threshold txs failing owner notification are NOT executed | ❌ | Implement owner-unreachable timeout path; 3-state test (approve/deny/timeout) | 3-4 | #4, #5 |
| 16 | Skill failures isolated (panic in one ≠ crash module) | ❌ | Wrap `Skill::invoke` in `catch_unwind`; add per-skill timeout; tests | 3-4 | — (independent) |

## Performance (1)

| # | Criterion | Current | Flip action(s) | Hours | Blocked by |
|---|-----------|---------|----------------|-------|-----------|
| 17 | Per-skill CU cost on LEZ testnet | ❌ | Measure CU for each on-chain op via sequencer telemetry; publish as `docs/CU_COSTS.md` table | 2-3 | #2, #18 |

## Supportability (6)

| # | Criterion | Current | Flip action(s) | Hours | Blocked by |
|---|-----------|---------|----------------|-------|-----------|
| 18 | Deployed on LEZ devnet/testnet | ❌ | **🔑** Build agent-registry + task-escrow LEZ programs; deploy via wallet-ffi; record program IDs in `docs/DEPLOYED_PROGRAMS.md` | 8-10 | #2 |
| 19 | E2E integration tests against sequencer in CI | ❌ | Add CI job: boot standalone sequencer container; run `cargo test --features integration`; cache to keep <15min | 4-6 | #18 |
| 20 | CI green on default branch | ✅ | (already satisfied) | 0 | — |
| 21 | README documents end-to-end usage | ⚠️ | Update README so commands shown work today (not "[TODO Week-N]"); test from clean VM | 2-3 | all functional items |
| 22 | Reproducible demo script with `RISC0_DEV_MODE=0` | ❌ | Replace `[Phase 1-2 placeholder]` in `full-demo.sh` with real assertions; tested on clean Ubuntu + macOS | 4-6 | #1, #18 |
| 23 | Recorded video proving `RISC0_DEV_MODE=0` | ❌ | Record terminal showing `RISC0_DEV_MODE=0` banner during demo recordings | included in #9 | #22 |

---

## Critical-path dependency graph

```
        ┌──────────────────────┐
        │  #2  wallet-ffi      │ ◄── bedrock; nothing else moves without this
        │  #4  messaging IPC   │ ◄── parallel to #2; bedrock for chat + A2A
        └──────────┬───────────┘
                   │
        ┌──────────▼───────────┐
        │ #3  quickstart       │ ◄── unblocks deployers
        │ #5  spending gate    │
        │ #6  21 skills wired  │
        │ #7  card pub/sub     │
        │ #14 retry + persist  │
        │ #15 owner timeout    │
        └──────────┬───────────┘
                   │
        ┌──────────▼───────────┐
        │ #18 LEZ programs     │ ◄── unblocks #10 (registry) and #8 (escrow)
        │     deployed         │
        └──────────┬───────────┘
                   │
        ┌──────────▼───────────┐
        │ #8  A2A round-trip   │
        │     w/ LEZ payment   │
        │ #10 5+ deployers     │ ◄── runs in PARALLEL during phases 2-3
        │ #17 CU costs measured│
        │ #19 CI integration   │
        └──────────┬───────────┘
                   │
        ┌──────────▼───────────┐
        │ #9  3 demo recordings│
        │ #22 reproducible     │
        │     demo script      │
        │ #23 RISC0=0 in video │
        │ #21 README accurate  │
        └──────────┬───────────┘
                   │
        ┌──────────▼───────────┐
        │ #11 docs finalized   │
        │ #1  module loads in  │
        │     Basecamp         │
        │ #13 QML owner UI     │
        └──────────┬───────────┘
                   │
                ┌──▼──┐
                │ PR  │ ◄── opens against logos-co/lambda-prize
                └─────┘
```

**Independent (run anytime):**
- #12 SDK example + crates.io publish
- #16 catch_unwind + skill timeout
- Vercel demo site (supports #9 hosting but doesn't block submission)

---

## Total effort

```
Functionality (#1-#11):   ~85-110 hours
Usability (#12-#13):      ~11-16 hours
Reliability (#14-#16):    ~10-14 hours
Performance (#17):        ~2-3 hours
Supportability (#18-#23): ~10-15 hours

TOTAL:                    ~118-158 hours + $250 bounty pool
```

Already matches the `PRE_SUBMISSION_CHECKLIST.md` Phase 2-6 estimate. No new work surfaced — just a different organizing principle.

---

## The single most important sequencing fact

**14 of the 22 unsatisfied criteria depend on action #2 (wallet-ffi linking).** This is the highest-leverage single block of work in the entire sprint.

If you commit to the full sprint, start with #2 + #4 in parallel (they're independent bedrock). Everything else cascades from those two. If wallet-ffi linking fails or takes >20 hours, that's the abort signal — the rest of the plan unravels around it.

---

## Verification (when each item is done)

Every flip is verifiable by a single command. The full set lives in `scripts/verify-submission.sh`, which currently reports `29 PASS / 0 FAIL / 7 WARN`. The 7 WARNs map exactly to criteria #6 (full), #9, #10, #17, plus the three optional docs. When `verify-submission.sh` reports `0 WARN`, the submission is win-eligible.

```bash
# The single command an evaluator runs:
./scripts/verify-submission.sh && ./scripts/verify-deployers.sh && ./scripts/full-demo.sh
```

If all three succeed from a clean clone, the submission is ready.
