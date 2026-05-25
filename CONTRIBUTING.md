# Contributing to logos-agent

Thanks for your interest. This repo is the reference implementation for
[LP-0008](https://github.com/logos-co/lambda-prize/blob/master/prizes/LP-0008.md).

## Quickest path to contribute

1. **Add a skill.** The Skill SDK is small. See [`crates/agent-skill-sdk/README.md`](crates/agent-skill-sdk/README.md). 30 lines and you have a skill running.
2. **Deploy an agent on testnet.** We need 5+ outside-team deployers for the prize. `cargo install logos-agent && logos-agent quickstart`. Open an issue with `[deployment]` in the title to claim your $50 USDT bounty (Week 3+).
3. **Review the A2A specs.** [`spec/a2a-logos-messaging-binding.md`](spec/a2a-logos-messaging-binding.md) and [`spec/lez-payment-extension.md`](spec/lez-payment-extension.md). Open Discussions for feedback.

## Development setup

```bash
git clone https://github.com/warfield2016/logos-agent
cd logos-agent
cargo check --workspace      # 4-5 sec on warm cache
cargo test --workspace       # 6 unit tests pass
```

For the full module build (Qt plugin shim auto-generated):

```bash
nix build .#module
```

See [`docs/DEPLOYMENT.md`](docs/DEPLOYMENT.md) for the full deployment flow.

## Code conventions

- **Rust style.** `cargo fmt --all` before committing. CI checks this.
- **Clippy clean.** `cargo clippy --workspace -- -D warnings`. CI also checks this.
- **One-line tests preferred over comments.** If something needs explaining, write a test that demonstrates the behaviour.
- **No `unwrap()` in non-test code.** Use `?` with `anyhow::Result` or a domain error.
- **Skill naming.** `<category>.<verb>` (lowercase, dot-separated). See [`spec/skill-interface.md`](spec/skill-interface.md) §6.

## Commit conventions

We loosely follow Conventional Commits:

```
feat(skills): add translate.en_to_ja skill
fix(spending): per-day cap resets correctly across midnight
docs(a2a): clarify seq ordering requirements
```

Type prefixes: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`, `ci`.

## Pull requests

- Open against `main`.
- Reference any related Issue.
- CI must be green.
- For substantive changes, include a short "why" in the PR description, not just "what."

## Spec changes

The two A2A specs (binding + payment extension) are versioned (`/0.1/` in the URI). Breaking changes require a new version, not edits to v0.1. Open an RFC issue with `[spec]` in the title.

## Security

For security issues, please email w4rfield.euro@gmail.com rather than opening a public issue. We'll respond within 72 hours.

## License

By contributing, you agree your contributions are licensed under MIT OR Apache-2.0 (operator's choice).
