# Competitive Intelligence — LP-0008

## Field state (as of 2026-05-24)

- **One prior attempt.** `Beach-Bum/Agora` (PR #34, self-closed 2026-04-28). No reviews recorded, no comments.
- **No Large prize merged** through `logos-co/lambda-prize` to date.
- **Reviewer:** fryorcraken (Logos / Waku core). Cares about standards alignment.

## What Beach-Bum did right (copy this)

1. **Comprehensive skill coverage.** 21 skills across 5 categories — matched the prize spec exactly. We do the same.
2. **Pluggable LLM with fallback chain.** Daemon-AI → Ollama → API → Mock. Right pattern, wrong primary backend.
3. **3 LEZ programs (identity, escrow, reputation).** Right call to put state on-chain rather than off.
4. **Formal protocol docs in a separate `docs/PROTOCOL.md`.** Standards orientation.

## What Beach-Bum did wrong (avoid this)

1. **❌ Self-deployed his own 5 agents** (alpha/beta/gamma/delta/epsilon) instead of recruiting outside-team deployers. This is the *de-facto disqualifier*. See [`RECRUITMENT.md`](RECRUITMENT.md).
2. **❌ Python-heavy stack** (51.7% Python). Reads as amateur for a privacy/crypto stack. We're 100% Rust via [`logos-co/logos-rust-sdk`](https://github.com/logos-co/logos-rust-sdk).
3. **❌ Tied LLM branding to `daemon-ai`** — a single-vendor Japanese Mamba SSM research project. Future-dep risk. We default to `llama.cpp` (broad ecosystem).
4. **❌ Underspecified protocol parameters** (alpha ≈ 18%, `slash_bps`). We specify every value verbatim and version-prefix all topics (`/logos-a2a/0.1/...`).
5. **❌ macOS-only deployment** with hard Qt 6 + Basecamp v0.1 requirement. We target Linux/macOS/Windows via Nix + cross-compilation.
6. **❌ Self-closed without engaging reviewers.** Suggests he knew the deployment criterion was unmet. We recruit deployers in Week 3, before the PR is opened.

## Other builders to watch

| Handle | Activity | Threat level |
|--------|----------|--------------|
| `mmlado` | Won LP-0009 + LP-0010 (small) | Low — focuses on smaller prizes |
| `bristinWild` | Won LP-0012, leading LP-0013 | Medium — productive, but seems to stay in their lane |
| `Gmin2` | Won LP-0012 | Low |
| `Tranquil-Flow` | Multi-prize iterator | Medium |
| `syafiqeil` | 4 attempts on LP-0016, none merged | Low (struggles) |

None are working on LP-0008 as of the recon date.

## A2A custom-binding implementations to study

These exist and we can mine for patterns; none compete directly:

- `id/skitter` — A2A-over-MQTT in Go. Cleanest broker-as-transport reference.
- `qntx/ra2a` — full A2A v1.0 SDK in Rust. Their `ClientFactory` is the extension point for adding a Waku transport.
- `EmilLindfors/a2a-rs` + `a2a-ap2` — Agent Payments Protocol crate. We frame `lez-payment` as an AP2-aligned extension.
- `google-agentic-commerce/a2a-x402` — A2A's official payment extension (HTTP 402). Our LEZ-Payment is the analogous extension for on-chain settlement.
- `a2aproject/a2a-go` — reference `preferredTransport` + `additionalInterfaces` patterns.

## Strategic differentiators we're shipping

1. **Two URI-addressable A2A specs** as standalone artifacts. Positions submission as a contribution to the A2A ecosystem, not just an agent.
2. **Pure Rust** for the runtime (Beach-Bum used Python). Matches reviewer idiom (Status/Waku are Nim+Rust shops).
3. **Inverse income cap** (anti-grooming). Novel, zero-effort, zero submissions in the field have it.
4. **30-line LangChain interop demo.** Demonstrates cross-framework value the spec explicitly mentions. Beach-Bum's demos were all intra-Agora.
5. **On-chain `agent-registry`** with owner attestations. Makes the 5-deployer verification on-chain and machine-verifiable.
6. **Per-token + per-period spending caps** with passing unit tests. Beach-Bum was vague.

## Time pressure

First-to-merge wins. As long as no other submission is opened during Weeks 1–3, we have a clear runway. Daily PR polling starting Week 3.
