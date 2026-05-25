#!/usr/bin/env bash
# verify-submission.sh — mechanically check every LP-0008 success criterion.
#
# Output: ASCII-table pass/fail report, screenshot-ready for the PR description.
# Designed for the evaluator: they clone the repo and run one command.
#
# Exit code: 0 if all criteria pass, 1 if any fail.

set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# Colors (disabled if not a tty)
if [ -t 1 ]; then
    GREEN='\033[0;32m'; RED='\033[0;31m'; YELLOW='\033[0;33m'; NC='\033[0m'
else
    GREEN=''; RED=''; YELLOW=''; NC=''
fi

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0

check() {
    local label="$1"; local cmd="$2"
    printf "%-55s " "$label"
    if eval "$cmd" > /tmp/verify.log 2>&1; then
        printf "${GREEN}✓ PASS${NC}\n"
        PASS_COUNT=$((PASS_COUNT + 1))
    else
        printf "${RED}✗ FAIL${NC}\n"
        printf "    → see /tmp/verify.log\n"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
}

warn() {
    local label="$1"; local cmd="$2"
    printf "%-55s " "$label"
    if eval "$cmd" > /tmp/verify.log 2>&1; then
        printf "${GREEN}✓ PASS${NC}\n"
        PASS_COUNT=$((PASS_COUNT + 1))
    else
        printf "${YELLOW}⚠ WARN${NC}\n"
        WARN_COUNT=$((WARN_COUNT + 1))
    fi
}

section() {
    printf "\n${YELLOW}━━━ %s ━━━${NC}\n" "$1"
}

# ────────────────────────────────────────────────────────────────────
section "Build & test"
# ────────────────────────────────────────────────────────────────────
check "cargo check --workspace" \
    "cargo check --workspace --message-format=short"
check "cargo test --workspace" \
    "cargo test --workspace"
check "cargo fmt --check" \
    "cargo fmt --all -- --check"
check "cargo clippy (no warnings)" \
    "cargo clippy --workspace --all-targets -- -D warnings"

# ────────────────────────────────────────────────────────────────────
section "Repository hygiene"
# ────────────────────────────────────────────────────────────────────
check "LICENSE-MIT present" \
    "[ -f LICENSE-MIT ]"
check "LICENSE-APACHE present" \
    "[ -f LICENSE-APACHE ]"
check "README.md present" \
    "[ -f README.md ]"
check "CONTRIBUTING.md present" \
    "[ -f CONTRIBUTING.md ]"
check "No <TBD> placeholders in published files" \
    "! grep -rln '<TBD>' README.md spec/ Cargo.toml crates/*/Cargo.toml crates/agent-skill-sdk/README.md CONTRIBUTING.md 2>/dev/null"

# ────────────────────────────────────────────────────────────────────
section "Spec docs (standards-author positioning)"
# ────────────────────────────────────────────────────────────────────
check "A2A binding spec exists" \
    "[ -f spec/a2a-logos-messaging-binding.md ]"
check "LEZ payment extension spec exists" \
    "[ -f spec/lez-payment-extension.md ]"
check "Skill interface spec exists" \
    "[ -f spec/skill-interface.md ]"
check "Binding URI declared in metadata.json" \
    "grep -q 'a2a_binding' metadata.json"

# ────────────────────────────────────────────────────────────────────
section "Functionality (LP-0008 spec §4)"
# ────────────────────────────────────────────────────────────────────
check "21 default skills registered" \
    "grep -c 'registry.register' crates/agent-core/src/skills/mod.rs | grep -q '^21$'"
check "All 21 skills exposed via meta.skills" \
    "grep -c 'registry.register' crates/agent-core/src/skills/mod.rs | xargs -I{} test {} -eq 21"
check "FFI surface complete (agent_init_runtime, _create, _invoke_skill, etc.)" \
    "grep -E 'pub.*extern.*fn agent_' crates/agent-core/src/ffi.rs | wc -l | xargs -I{} test {} -ge 7"
check "cbindgen header generated" \
    "cargo build --release -p agent-core --message-format=short > /dev/null && [ -f target/include/agent_core.h ]"
check "Spending policy enforced per-token + per-period" \
    "cargo test --package agent-core spending_policy"

# ────────────────────────────────────────────────────────────────────
section "A2A coordination"
# ────────────────────────────────────────────────────────────────────
check "AgentCard type exists" \
    "grep -q 'pub struct AgentCard' crates/agent-core/src/a2a/agent_card.rs"
check "Task lifecycle state machine" \
    "grep -q 'pub enum TaskStatus' crates/agent-core/src/a2a/task.rs"
check "All A2A task states covered" \
    "grep -E '(Submitted|Working|InputRequired|AuthRequired|Completed|Failed|Canceled|Rejected)' crates/agent-core/src/a2a/task.rs | wc -l | xargs -I{} test {} -ge 8"
check "Logos Messaging topic schema declared" \
    "grep -q 'mod topics' crates/agent-core/src/a2a/mod.rs"
check "LEZ payment envelope schema" \
    "grep -q 'pub enum PaymentEnvelope' crates/agent-core/src/a2a/payment.rs"

# ────────────────────────────────────────────────────────────────────
section "5-deployer requirement (PHASE 3+ — see verify-deployers.sh)"
# ────────────────────────────────────────────────────────────────────
warn "5+ outside-team deployers on agent-registry" \
    "[ -x scripts/verify-deployers.sh ] && scripts/verify-deployers.sh --count-only | grep -qE '^[5-9]|^[0-9]{2,}'"

# ────────────────────────────────────────────────────────────────────
section "Demo recordings (Phase 4)"
# ────────────────────────────────────────────────────────────────────
warn "Demo 1 (privacy-preserving notary) linked" \
    "grep -q 'Demo 1' README.md"
warn "Demo 2 (agent services marketplace) linked" \
    "grep -q 'Demo 2' README.md"
warn "Demo 3 (LangChain interop) linked" \
    "grep -q 'Demo 3' README.md"
warn "RISC0_DEV_MODE=0 verified in demos" \
    "grep -q 'RISC0_DEV_MODE=0' README.md docs/DEPLOYMENT.md"

# ────────────────────────────────────────────────────────────────────
section "Documentation completeness"
# ────────────────────────────────────────────────────────────────────
check "Deployment guide" "[ -f docs/DEPLOYMENT.md ]"
check "Security model" "[ -f docs/SECURITY_MODEL.md ]"
check "Sprint build plan" "[ -f docs/BUILD_PLAN.md ]"
check "Recruitment playbook" "[ -f docs/RECRUITMENT.md ]"
check "Submission checklist" "[ -f docs/SUBMISSION_CHECKLIST.md ]"
warn "Per-skill CU cost table" "[ -f docs/CU_COSTS.md ]"
warn "Owner usage guide" "[ -f docs/OWNER_GUIDE.md ]"
warn "Skill SDK guide" "[ -f docs/SKILL_SDK_GUIDE.md ]"

# ────────────────────────────────────────────────────────────────────
# Summary
# ────────────────────────────────────────────────────────────────────
printf "\n${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
printf "  PASS: ${GREEN}%d${NC}    FAIL: ${RED}%d${NC}    WARN: ${YELLOW}%d${NC}\n" \
    "$PASS_COUNT" "$FAIL_COUNT" "$WARN_COUNT"
printf "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

if [ "$FAIL_COUNT" -gt 0 ]; then
    printf "\n${RED}Submission NOT ready: %d FAIL(s) must be resolved.${NC}\n" "$FAIL_COUNT"
    exit 1
fi
if [ "$WARN_COUNT" -gt 0 ]; then
    printf "\n${YELLOW}Submission has %d WARNing(s) — typically Phase 3-4 work.${NC}\n" "$WARN_COUNT"
fi
printf "\n${GREEN}All required criteria pass.${NC}\n"
exit 0
