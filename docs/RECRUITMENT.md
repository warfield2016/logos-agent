# 5-Deployer Recruitment Playbook

> The single hardest criterion in LP-0008 is *"at least 5 agents deployed on LEZ testnet by parties outside the submitting team."* Beach-Bum failed at this — he spun up his own 5 instances named alpha/beta/gamma/delta/epsilon, which doesn't satisfy "outside the submitting team." This document is the plan to actually solve it.

## Budget

- $250 cash bounty pool ($50 × 5 verified deployers)
- ~10 hours of recruitment effort across Weeks 3–4

ROI math: Beach-Bum lost the full $1,200. We spend $250 + 10 hours to win it. Net: $950 minus 10 hours ≈ $95/hr just on the social spend.

## Target deployers

Looking for **5–7 verified deployers**. Aim for 7 to safely clear the 5 minimum (some will flake).

Ideal personas, in order of likely yield:

1. **Other λPrize builders** — active on `logos-co/lambda-prize`. Check the contributor list and DM the ones who have shipped at least one PR (mmlado, bristinWild, Gmin2, Tranquil-Flow, syafiqeil). They understand the program, are reachable, and can deploy in <30 minutes.
2. **Logos Discord `#builder-hub` regulars** — drop a post offering testnet NOM airdrop + $50 USDT for a verified deployment.
3. **Status / Waku contributors** — overlapping community; many would test a Waku-based agent module just for the kick.
4. **A2A community on GitHub** — file issues/discussions on `a2aproject/A2A` pointing to the Logos Messaging binding spec; some will deploy out of standards-curiosity.
5. **Indie Rust devs on Twitter/HN** — broader net; lower conversion but no recruitment cost beyond a post.

## Verification mechanism

Every deployer registers on the `agent-registry` LEZ program with an owner attestation:

```
register(agent_npk, owner_attestation_sig, manifest_cid)
```

The attestation signature proves the owner_npk is distinct from the submitting team. The submission PR includes a one-liner script that queries the registry and reports the 5+ unique non-team NPKs with first-seen block heights.

Evaluator workflow:
```bash
git clone <our-repo>
./scripts/verify-deployers.sh
# → "VERIFIED 7 deployers — NPKs: 0xabc..., 0xdef..., ..."
```

## Recruitment artifacts

### Discord post (drop in `#builder-hub` Week 3, Day 1)

```
🤖 Looking for 5 testers for LP-0008 (Logos Autonomous AI Agent Module)

I'm shipping the reference implementation for the LP-0008 prize and need
5 testnet deployers from outside my team to satisfy the success criterion.

What's in it for you:
  • $50 USDT per verified deployment (paid on PR merge)
  • Testnet NOM airdrop to your agent's wallet
  • Free listing in the Logos Agent Directory (built into the repo)
  • A working sovereign AI agent on your own infra

What you do:
  • cargo install logos-agent
  • logos-agent quickstart      (5 min wizard)
  • Reply with your agent's NPK so I can register the attestation

Repo: <link>
Spec: <link to A2A-over-Logos-Messaging binding>

DM if interested.
```

### Forum post (Logos forum, Week 3, Day 2)

Same content, formatted for forum. Cross-link from Discord post.

### A2A community post (GitHub Discussions on `a2aproject/A2A`, Week 3, Day 3)

Frame as: *"Custom A2A transport binding over Logos Messaging — looking for early implementers to test interop."* Don't lead with the bounty; lead with the spec and standards angle. The $50 USDT incentive is mentioned in a follow-up comment for serious responders.

### Hacker News post (Week 4, Day 1, after polishing)

*"Show HN: Sovereign AI agents with native crypto payments, no central server."*

Body: the 30-line LangChain interop demo. Repo link. Spec link.

### Twitter / X post (timed with HN)

Single thread: the architecture diagram, the demo GIF, the spec links, the bounty.

## Conversion mechanics

1. **Onboarding must be ≤ 5 minutes from interest to deployed agent.** Anything longer kills conversion. `logos-agent quickstart` MUST cover identity, faucet, owner config, AgentCard publication in one wizard.
2. **A working starter skill** so the deployer gets something useful, not a demo. Candidate: `pricefeed.eth_usd` — pulls free CoinGecko, returns signed price. Useful AND demonstrative.
3. **Persistent attribution.** The Logos Agent Directory (a static page rendered from the registry) lists every deployer's agent name + NPK. Public credit is its own incentive for some.
4. **Same-day bounty payment.** Pay the $50 within 24 hours of attestation registration. Reputation is the long game; speed of payment is the marketing.

## Anti-flake measures

- Pre-DM 10 candidates from persona #1 before opening the public posts. Get 3 committed before the bounty pool depletes.
- A deployer who registers but disappears within 24h is still a verified deployer per the criterion text — we don't need ongoing liveness, just the registration.
- Keep a backup pool of 3 personally-known devs willing to deploy as a favour (no bounty), as last-resort to clear the floor.

## Timeline

| Day | Action |
|-----|--------|
| Wk3 D1 | Discord post + DM 5 personal candidates |
| Wk3 D2 | Forum post + follow up on DMs |
| Wk3 D3 | A2A community post (standards angle) |
| Wk3 D5 | Check progress; if <3 verified, escalate to backup pool |
| Wk4 D1 | HN + Twitter post (assumes good demo video by then) |
| Wk4 D3 | Final verification sweep; pay all bounties |
| Wk4 D5 | Submit PR with verification script + on-chain attestations |
