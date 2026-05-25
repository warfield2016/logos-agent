#!/usr/bin/env bash
# verify-deployers.sh — query the on-chain agent-registry and report the
# unique non-team NPKs registered on LEZ testnet.
#
# This is THE artifact the evaluator uses to verify the "5 outside-team
# deployments" success criterion. Output is intentionally simple and
# screenshot-ready.
#
# Usage:
#   scripts/verify-deployers.sh                # full report
#   scripts/verify-deployers.sh --count-only   # just the count (for CI gating)
#   scripts/verify-deployers.sh --json         # machine-readable
#
# Env:
#   LEZ_SEQUENCER_URL   — defaults to http://localhost:3001
#   AGENT_REGISTRY_ID   — program ID of the deployed agent-registry
#   TEAM_NPK_FILE       — file with one team-member NPK per line (excluded)

set -uo pipefail

SEQUENCER="${LEZ_SEQUENCER_URL:-http://localhost:3001}"
REGISTRY_ID="${AGENT_REGISTRY_ID:-<deployed-program-id>}"
TEAM_FILE="${TEAM_NPK_FILE:-./team-npks.txt}"

MODE="report"
case "${1:-}" in
    --count-only) MODE="count" ;;
    --json)       MODE="json" ;;
    --help)
        sed -n '2,15p' "$0"
        exit 0 ;;
esac

# Placeholder until LEZ programs are deployed in Phase 3.
# The real implementation will use the LEZ RPC interface to query the
# `agent-registry` program's `list_recent(n)` instruction and filter out
# any NPKs present in $TEAM_FILE.
#
# Pseudo-code:
#
#   registrations = lez_query(sequencer=$SEQUENCER,
#                             program=$REGISTRY_ID,
#                             instruction='list_recent',
#                             params={'limit': 100})
#   team_npks = read_lines($TEAM_FILE)
#   outside_npks = unique(r.owner_npk for r in registrations
#                                     if r.owner_npk not in team_npks)
#   print outside_npks, len(outside_npks)

if [ "$REGISTRY_ID" = "<deployed-program-id>" ]; then
    case "$MODE" in
        count) echo "0" ;;
        json)  echo '{"count": 0, "deployers": [], "status": "pre_phase3"}' ;;
        report)
            cat <<'EOF'
═══════════════════════════════════════════════════════════════
  LP-0008 Third-Party Deployer Verification
═══════════════════════════════════════════════════════════════

  STATUS: Pre-Phase-3
  The agent-registry LEZ program has not yet been deployed.
  This script will produce verified deployer counts after the
  registry is live (target: Week 3, per docs/BUILD_PLAN.md).

  Set AGENT_REGISTRY_ID environment variable once deployed.

═══════════════════════════════════════════════════════════════
EOF
        ;;
    esac
    exit 0
fi

# Live path (post-Phase-3) — placeholder for the real RPC call.
# Replace this block with the actual `curl` to the sequencer RPC.
echo "[live verification not yet implemented]"
exit 1
