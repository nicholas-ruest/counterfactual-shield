use cs_types::Id;
use cs_verifier::VerificationReport;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionGrant {
    pub batch_id: Id,
    pub nonce: Id,
    pub scope_digest: [u8; 32],
}
#[derive(Default)]
pub struct ExecutionGate {
    used: BTreeSet<Id>,
}
#[derive(Debug, Error)]
pub enum GateError {
    #[error("verification denied")]
    Denied,
    #[error("nonce replay")]
    Replay,
}
impl ExecutionGate {
    pub fn issue(
        &mut self,
        batch_id: Id,
        nonce: Id,
        digest: [u8; 32],
        report: &VerificationReport,
    ) -> Result<ExecutionGrant, GateError> {
        if !report.release {
            return Err(GateError::Denied);
        }
        if !self.used.insert(nonce.clone()) {
            return Err(GateError::Replay);
        }
        Ok(ExecutionGrant {
            batch_id,
            nonce,
            scope_digest: digest,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_replay() {
        let mut g = ExecutionGate::default();
        let r = VerificationReport {
            policy_ok: true,
            risk_ok: true,
            evidence_ok: true,
            release: true,
        };
        let id = Id::new("x").unwrap();
        g.issue(id.clone(), id.clone(), [1; 32], &r).unwrap();
        assert!(matches!(
            g.issue(id.clone(), id, [1; 32], &r),
            Err(GateError::Replay)
        ));
    }
}
