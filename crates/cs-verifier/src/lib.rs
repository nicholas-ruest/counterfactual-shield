use cs_policy_model::PolicyReceipt;
use cs_risk_control::RiskCertificate;
use cs_rvf::{EvidenceSeal, verify};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationReport {
    pub policy_ok: bool,
    pub risk_ok: bool,
    pub evidence_ok: bool,
    pub release: bool,
}
pub fn verify_release(
    policy: &PolicyReceipt,
    risk: &RiskCertificate,
    seal: &EvidenceSeal,
) -> VerificationReport {
    let policy_ok = policy.decision.permits();
    let risk_ok = risk.passes;
    let evidence_ok = verify(seal).is_ok();
    VerificationReport {
        policy_ok,
        risk_ok,
        evidence_ok,
        release: policy_ok && risk_ok && evidence_ok,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cs_plan::MitigationPlan;
    use cs_policy_model::PolicyDecision;
    #[test]
    fn all_conditions_required() {
        let p = PolicyReceipt {
            decision: PolicyDecision::Deny,
            policy_count: 1,
            diagnostics: vec![],
        };
        let r = RiskCertificate {
            upper_loss: 0.1,
            alpha: 0.2,
            samples: 5,
            passes: true,
        };
        let s = cs_rvf::seal(
            "b",
            &MitigationPlan {
                steps: vec![],
                spent: 0,
                expected_reduction: 0,
            },
        )
        .unwrap();
        assert!(!verify_release(&p, &r, &s).release);
    }
}
