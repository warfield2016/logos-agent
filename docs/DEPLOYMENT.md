# Deployment Guide

## Prerequisites

- Rust 1.85+ (or use the pinned `rust-toolchain.toml`)
- Nix with flakes (recommended)
- A Logos Core installation: either [`logos-basecamp`](https://github.com/logos-co/logos-basecamp) (GUI) or `logoscore` (headless)
- LEZ testnet access (faucet URL TBD)

## Build

### With Nix (recommended)

```bash
git init -q && git add -A      # Nix requires git-tracked files
nix build .#module             # → ./result/lib/libagent_module.{so,dylib}
nix build .#default            # alias for .#module
```

### With Cargo (faster iteration)

```bash
cargo build --release --workspace
# cdylib: target/release/libagent_core.{so,dylib}
# Generated C header: target/include/agent_core.h
```

Note: a pure-Cargo build produces only the Rust cdylib. The Qt plugin shim
(`libagent_module.{so,dylib}`) requires the Nix build, which invokes
`logos-cpp-generator --from-c-header` over `agent_core.h`.

## Install into Logos Core

```bash
# Detect plugin directory
PLUGIN_DIR="$HOME/.local/share/Logos/LogosBasecampDev/plugins"  # Linux
# PLUGIN_DIR="$HOME/Library/Application Support/Logos/LogosBasecampDev/plugins"  # macOS

cp ./result/lib/libagent_module.* "$PLUGIN_DIR/"
```

Restart Basecamp / logoscore. The `agent_module` should appear in the loaded
modules list.

## Initialize the agent

```bash
logos-agent quickstart
```

This wizard:

1. Generates an NPK/ISK keypair (saved to `~/.logos-agent/identity.json`)
2. Requests testnet LEZ from the faucet
3. Prompts for owner NPK (the human-controlled wallet that approves above-threshold transactions)
4. Sets default spending policy (`per_tx = 10 LEZ`, `per_day = 100 LEZ`)
5. Publishes the agent's AgentCard on the discovery topic

## Manual configuration

`~/.logos-agent/config.toml`:

```toml
[identity]
path = "~/.logos-agent/identity.json"

[owner]
npk = "<owner-npk-hex>"

[spending]
per_tx_lez       = 10
per_day_lez      = 100
income_cap_lez   = 1000     # optional anti-grooming cap

[llm]
backend = "llama_cpp"       # or "ollama" or "api"
model_path = "~/.logos-agent/models/llama-3.1-8b-instruct.gguf"

[a2a]
discovery_topic = "/logos-a2a/0.1/cards"
publish_card    = true
card_ttl_secs   = 3600
```

## Owner interaction

Two interfaces:

### CLI (headless)

```bash
logos-agent chat               # opens encrypted TUI over Messaging
logos-agent status             # one-shot status snapshot
logos-agent skills             # list registered skills
logos-agent invoke wallet.balance
```

### Basecamp (GUI)

Open Basecamp → `agent_module` tile. The QML view provides:
- Chat pane (mirrors `logos-agent chat`)
- Pending-approvals widget for above-threshold transactions
- Live A2A task feed

## Remote deployment

```bash
logos-agent deploy --remote user@your-vps
```

Equivalent of:

```bash
rsync -avz ./result/ user@your-vps:~/logos-agent/
ssh user@your-vps 'systemctl --user restart logoscore'
```

## Verification

```bash
logos-agent status
# {
#   "version": "0.1.0",
#   "npk": "abc123...",
#   "owner_npk": "def456...",
#   "skill_count": 21,
#   "pending_approvals": 0,
#   "active_tasks": 0
# }
```

## Sequencer standalone mode (for testing)

```bash
git clone https://github.com/logos-blockchain/logos-execution-zone
cd logos-execution-zone
RISC0_DEV_MODE=0 cargo run --features standalone -p sequencer_service \
  sequencer/service/configs/debug
```

Then point the agent at the local sequencer via `LOGOS_AGENT_SEQUENCER=http://localhost:3001`.

For demos that satisfy the LP-0008 "RISC0_DEV_MODE=0" requirement, run the sequencer with that flag and screen-record the terminal showing the proof generation banner.

## Troubleshooting

| Symptom | Diagnosis | Fix |
|---------|-----------|-----|
| Module fails to load: "missing symbol agent_init_runtime" | Wrong library version copied | Verify `nm libagent_module.so | grep agent_init_runtime` resolves; rebuild |
| `wallet_ffi_create_new` returns `STORAGE_ERROR` | Storage path not writable | Check `~/.logos-agent/` permissions |
| `agent.discover` returns empty | Not subscribed to discovery topic | Confirm `[a2a].publish_card = true` and restart |
| Above-threshold tx silently dropped | Owner unreachable | Check owner channel: `logos-agent chat` — owner must reply |
