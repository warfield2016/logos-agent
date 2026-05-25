# logos-agent — Autonomous AI Agent Module for Logos

> Reference implementation for [LP-0008](https://github.com/logos-co/lambda-prize/blob/master/prizes/LP-0008.md).
> 100% Rust. A2A v1.0.0 compatible. End-to-end encrypted. Privately settled in LEZ tokens. No central server.

`logos-agent` is a Logos Core module that runs an autonomous AI agent with native access to the full Logos stack: a shielded LEZ wallet, Logos Storage, and Logos Messaging. The agent has its own identity and funds, communicates with its owner over an end-to-end encrypted Messaging channel, and coordinates with other agents using the [Agent2Agent (A2A) protocol](https://a2a-protocol.org/) carried over Logos Messaging.

## Highlights

- **No custodian, no exposed API, no central server.** The agent holds its own NPK/ISK keypair and settles transactions directly on LEZ.
- **A2A v1.0.0 compatible.** Agents discover each other via signed AgentCards on a Logos Messaging discovery topic, and execute tasks following the canonical A2A task lifecycle.
- **Two standalone specs.** This repository ships [`spec/a2a-logos-messaging-binding.md`](spec/a2a-logos-messaging-binding.md) and [`spec/lez-payment-extension.md`](spec/lez-payment-extension.md), both URI-addressable A2A extensions intended for upstream adoption.
- **Pluggable skills and inference.** The [`agent-skill-sdk`](crates/agent-skill-sdk/) crate is a public, semver-stable surface for third-party skill authors. The LLM backend is a single Rust trait — default `llama.cpp`, with adapters for Ollama and OpenAI-compatible APIs.
- **Spending policy with owner approval.** Per-token, per-transaction, and per-period caps. Above-threshold transactions are queued on the owner channel for explicit approval. Includes an inverse income cap for anti-grooming.
- **First-class Logos Core integration.** Built with [`logos-co/logos-module-builder`](https://github.com/logos-co/logos-module-builder); the C++ Qt plugin shim is auto-generated from the cbindgen header — no hand-written C++.

## Architecture (one screen)

```
       Logos Core (Basecamp / logoscore headless)
              │
              ▼ loads .so / .dylib via Qt plugin system
   ┌──────────────────────────────────┐
   │  agent_module_plugin (auto-gen'd) │
   │  ├── Q_INVOKABLE bridge           │
   │  └── extern "C" → agent_core      │
   └──────────────────────────────────┘
              │ FFI: agent_init_runtime / agent_create
              ▼
   ┌──────────────────────────────────────────────────────────┐
   │  agent-core (Rust)                                       │
   │  ├── Runtime          Tokio + tracing, owns all state    │
   │  ├── Identity         NPK/ISK keypair (ed25519)          │
   │  ├── SpendingPolicy   per-token caps + approval queue    │
   │  ├── OwnerChannel     encrypted Messaging topic w/ owner │
   │  ├── A2A binding      Card publish, Task lifecycle       │
   │  ├── SkillRegistry    21 default skills + pluggable      │
   │  └── LLM backend      llama.cpp / Ollama / API           │
   │      │                                                    │
   │      └── calls wallet-ffi (Rust), storage_module,        │
   │          delivery_module via LogosAPI bridge             │
   └──────────────────────────────────────────────────────────┘
```

## Quickstart (5-minute target)

```bash
# 1. Install
cargo install logos-agent      # CLI
nix build .#module             # the Qt-loadable module

# 2. Initialize
logos-agent quickstart         # walks you through identity + faucet + owner config

# 3. Deploy module to Logos Core
cp ./result/lib/libagent_module.so \
   ~/.local/share/Logos/LogosBasecampDev/plugins/

# 4. Confirm
logos-agent status
```

The full deployment guide is in [`docs/DEPLOYMENT.md`](docs/DEPLOYMENT.md).

## Default skills (21)

| Category | Skills |
|----------|--------|
| Storage | `storage.upload`, `storage.download`, `storage.list`, `storage.share` |
| Messaging | `messaging.send`, `messaging.join`, `messaging.create_group` |
| Wallet | `wallet.balance`, `wallet.send`, `wallet.history` |
| Program (LEZ) | `program.query`, `program.call`, `program.deploy` |
| Agent (A2A) | `agent.card`, `agent.discover`, `agent.task`, `agent.subscribe`, `agent.cancel` |
| Meta | `meta.skills`, `meta.status`, `meta.configure` |

## Adding your own skill

```toml
# Cargo.toml
[dependencies]
agent-skill-sdk = "0.1"
async-trait = "0.1"
```

See [`crates/agent-skill-sdk/README.md`](crates/agent-skill-sdk/README.md) for a 30-line example.

## Repository layout

```
logos-agent/
├── crates/
│   ├── agent-skill-sdk/      # PUBLIC: third-party skill author SDK
│   ├── agent-core/           # runtime, FFI surface, default skills
│   └── agent-cli/            # `logos-agent` binary
├── programs/
│   ├── agent-registry/       # LEZ program: on-chain agent registry
│   └── task-escrow/          # LEZ program: A2A task escrow
├── spec/
│   ├── a2a-logos-messaging-binding.md   # A2A custom transport binding spec
│   ├── lez-payment-extension.md         # A2A payment extension spec
│   └── skill-interface.md               # Skill SDK contract
├── docs/
│   ├── DEPLOYMENT.md
│   ├── BUILD_PLAN.md         # 4-week sprint plan
│   ├── ARCHITECTURE.md
│   └── SECURITY_MODEL.md
├── flake.nix                 # nix build / dev shell
├── metadata.json             # Logos Core module manifest
└── Cargo.toml                # Rust workspace
```

## Status

| Component | Week | Status |
|-----------|------|--------|
| Workspace skeleton, SDK, default skill stubs | 1 | ✅ |
| wallet-ffi integration; owner channel; spending policy | 2 | 🚧 |
| A2A binding + 3 LEZ programs on testnet | 3 | 🚧 |
| Third-party deployments, demo videos, submission | 4 | 🚧 |

Full plan in [`docs/BUILD_PLAN.md`](docs/BUILD_PLAN.md).

## License

MIT OR Apache-2.0 (operator's choice).

## Acknowledgements

Builds on `logos-co/logos-module-builder`, `logos-co/logos-rust-sdk`, `logos-blockchain/logos-execution-zone` (wallet-ffi), `logos-co/logos-delivery-module` (messaging), `logos-co/logos-storage-module` (storage), and the [A2A Project](https://github.com/a2aproject/A2A).
