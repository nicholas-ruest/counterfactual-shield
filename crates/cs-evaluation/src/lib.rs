use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EvaluationMode {
    Combined,
    NoRuVector,
    NoRvf,
    NoCedar,
    NoRisk,
    EnterpriseOnly,
    RuvnetOnly,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationReport {
    pub mode: EvaluationMode,
    pub released: bool,
    pub recalled: usize,
    pub evidence: bool,
    pub policy_enforced: bool,
    pub robust: bool,
    pub critical_before: usize,
    pub critical_after: usize,
    pub objective: f64,
}
impl EvaluationReport {
    pub fn objective(
        recalled: usize,
        evidence: bool,
        policy: bool,
        robust: bool,
        before: usize,
        after: usize,
    ) -> f64 {
        (before.saturating_sub(after) as f64 * 10.0)
            + (recalled as f64 * 2.0)
            + f64::from(u8::from(evidence)) * 3.0
            + f64::from(u8::from(policy)) * 4.0
            + f64::from(u8::from(robust)) * 5.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rewards_composition() {
        assert!(
            EvaluationReport::objective(2, true, true, true, 3, 1)
                > EvaluationReport::objective(0, false, true, false, 3, 1)
        );
    }
}
