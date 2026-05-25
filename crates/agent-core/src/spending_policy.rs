//! Spending policy — per-tx and per-period caps. Above-threshold transactions
//! are routed to the owner channel for explicit approval; below-threshold pass
//! through autonomously.
//!
//! Per-token caps (LEZ, USDC-on-LEZ, ...) — Beach-Bum used a flat cap; per-token
//! shows nuance and matches how serious DeFi-aware agents actually need to work.

use crate::runtime::SpendingConfig;
use agent_skill_sdk::TokenAmount;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PeriodCap {
    pub limit: TokenAmount,
    pub spent: TokenAmount,
    pub period_start: DateTime<Utc>,
}

impl PeriodCap {
    fn new(limit: TokenAmount) -> Self {
        Self {
            limit,
            spent: TokenAmount(0),
            period_start: Utc::now(),
        }
    }

    fn would_exceed(&self, amount: TokenAmount) -> bool {
        self.spent.0.saturating_add(amount.0) > self.limit.0
    }

    fn record(&mut self, amount: TokenAmount) {
        self.spent = TokenAmount(self.spent.0.saturating_add(amount.0));
    }

    fn reset_if_period_elapsed(&mut self, period_secs: i64) {
        let elapsed = (Utc::now() - self.period_start).num_seconds();
        if elapsed >= period_secs {
            self.spent = TokenAmount(0);
            self.period_start = Utc::now();
        }
    }
}

#[derive(Debug)]
pub struct SpendingPolicy {
    per_tx_cap: HashMap<String, TokenAmount>,
    daily_cap: HashMap<String, PeriodCap>,
    income_cap: HashMap<String, TokenAmount>,
    period_secs: i64,
}

impl SpendingPolicy {
    pub fn from_config(cfg: &SpendingConfig) -> Self {
        let mut per_tx = HashMap::new();
        let mut daily = HashMap::new();
        let mut income = HashMap::new();
        per_tx.insert("LEZ".into(), TokenAmount(cfg.per_tx_lez));
        daily.insert("LEZ".into(), PeriodCap::new(TokenAmount(cfg.per_day_lez)));
        if let Some(cap) = cfg.income_cap_lez {
            income.insert("LEZ".into(), TokenAmount(cap));
        }
        Self {
            per_tx_cap: per_tx,
            daily_cap: daily,
            income_cap: income,
            period_secs: 86_400,
        }
    }

    /// Decide whether an outgoing transfer is autonomous or requires approval.
    pub fn evaluate(&mut self, token: &str, amount: TokenAmount) -> SpendDecision {
        if let Some(cap) = self.per_tx_cap.get(token) {
            if amount.0 > cap.0 {
                return SpendDecision::OwnerApprovalRequired(format!(
                    "tx {} exceeds per-tx cap {} on {}",
                    amount.0, cap.0, token
                ));
            }
        }
        if let Some(daily) = self.daily_cap.get_mut(token) {
            daily.reset_if_period_elapsed(self.period_secs);
            if daily.would_exceed(amount) {
                return SpendDecision::OwnerApprovalRequired(format!(
                    "tx {} would exceed daily cap {} on {} (spent so far: {})",
                    amount.0, daily.limit.0, token, daily.spent.0
                ));
            }
        }
        SpendDecision::Autonomous
    }

    /// Record a successfully executed transaction.
    pub fn record_spend(&mut self, token: &str, amount: TokenAmount) {
        if let Some(daily) = self.daily_cap.get_mut(token) {
            daily.record(amount);
        }
    }

    /// Evaluate an incoming payment against the inverse "income cap" policy.
    /// Useful for anti-grooming / anti-spam on paid skills.
    pub fn evaluate_incoming(&self, token: &str, amount: TokenAmount) -> IncomeDecision {
        if let Some(cap) = self.income_cap.get(token) {
            if amount.0 > cap.0 {
                return IncomeDecision::Refuse(format!(
                    "incoming {} exceeds income cap {} on {}",
                    amount.0, cap.0, token
                ));
            }
        }
        IncomeDecision::Accept
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpendDecision {
    Autonomous,
    OwnerApprovalRequired(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomeDecision {
    Accept,
    Refuse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn under_per_tx_cap_is_autonomous() {
        let mut p = SpendingPolicy::from_config(&SpendingConfig {
            per_tx_lez: 100,
            per_day_lez: 1000,
            income_cap_lez: None,
        });
        assert_eq!(
            p.evaluate("LEZ", TokenAmount(50)),
            SpendDecision::Autonomous
        );
    }

    #[test]
    fn over_per_tx_cap_requires_approval() {
        let mut p = SpendingPolicy::from_config(&SpendingConfig {
            per_tx_lez: 100,
            per_day_lez: 1000,
            income_cap_lez: None,
        });
        assert!(matches!(
            p.evaluate("LEZ", TokenAmount(200)),
            SpendDecision::OwnerApprovalRequired(_)
        ));
    }

    #[test]
    fn daily_cap_enforced_across_calls() {
        let mut p = SpendingPolicy::from_config(&SpendingConfig {
            per_tx_lez: 100,
            per_day_lez: 150,
            income_cap_lez: None,
        });
        assert_eq!(
            p.evaluate("LEZ", TokenAmount(80)),
            SpendDecision::Autonomous
        );
        p.record_spend("LEZ", TokenAmount(80));
        assert!(matches!(
            p.evaluate("LEZ", TokenAmount(80)),
            SpendDecision::OwnerApprovalRequired(_)
        ));
    }

    #[test]
    fn income_cap_refuses_large_incoming() {
        let p = SpendingPolicy::from_config(&SpendingConfig {
            per_tx_lez: 0,
            per_day_lez: 0,
            income_cap_lez: Some(100),
        });
        assert!(matches!(
            p.evaluate_incoming("LEZ", TokenAmount(500)),
            IncomeDecision::Refuse(_)
        ));
    }
}
