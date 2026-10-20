use cs_plan::MitigationPlan;
use rvf_crypto::{WitnessEntry, create_witness_chain, shake256_256, verify_witness_chain};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceSeal {
    pub chain: Vec<u8>,
    pub digest: [u8; 32],
}
#[derive(Debug, Error)]
pub enum SealError {
    #[error("invalid evidence")]
    Invalid,
}
pub fn seal(batch_material: &str, plan: &MitigationPlan) -> Result<EvidenceSeal, SealError> {
    if batch_material.is_empty() {
        return Err(SealError::Invalid);
    }
    let payload = format!(
        "{batch_material}:{}:{}",
        plan.spent, plan.expected_reduction
    );
    let digest = shake256_256(payload.as_bytes());
    let entry = WitnessEntry {
        prev_hash: [0; 32],
        action_hash: digest,
        timestamp_ns: 1,
        witness_type: 21,
    };
    Ok(EvidenceSeal {
        chain: create_witness_chain(&[entry]),
        digest,
    })
}
pub fn verify(s: &EvidenceSeal) -> Result<(), SealError> {
    if s.chain.is_empty() {
        return Err(SealError::Invalid);
    }
    verify_witness_chain(&s.chain)
        .map(|_| ())
        .map_err(|_| SealError::Invalid)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tamper_rejected() {
        let p = MitigationPlan {
            steps: vec![],
            spent: 0,
            expected_reduction: 0,
        };
        let mut s = seal("batch", &p).unwrap();
        verify(&s).unwrap();
        s.chain[0] ^= 1;
        assert!(verify(&s).is_err());
    }
}
