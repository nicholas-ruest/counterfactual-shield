use cs_rvf::EvidenceSeal;
use rvf_crypto::shake256_256;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub sequence: u64,
    pub batch_id: String,
    pub decision: String,
    pub previous: [u8; 32],
    pub record_hash: [u8; 32],
    pub seal: EvidenceSeal,
}
#[derive(Default)]
pub struct AppendOnlyLedger {
    records: Vec<DecisionRecord>,
}
#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("sequence or hash-chain violation")]
    Invalid,
}
impl AppendOnlyLedger {
    pub fn append(
        &mut self,
        batch_id: &str,
        decision: &str,
        seal: EvidenceSeal,
    ) -> Result<&DecisionRecord, LedgerError> {
        let sequence = self.records.len() as u64 + 1;
        let previous = self.records.last().map_or([0; 32], |r| r.record_hash);
        let record_hash =
            shake256_256(format!("{sequence}:{batch_id}:{decision}:{previous:?}").as_bytes());
        self.records.push(DecisionRecord {
            sequence,
            batch_id: batch_id.into(),
            decision: decision.into(),
            previous,
            record_hash,
            seal,
        });
        Ok(self.records.last().expect("just pushed"))
    }
    pub fn verify(&self) -> Result<(), LedgerError> {
        let mut prev = [0; 32];
        for (rn, r) in self.records.iter().enumerate() {
            let seq = rn as u64 + 1;
            if r.sequence != seq
                || r.previous != prev
                || r.record_hash
                    != shake256_256(
                        format!("{seq}:{}:{}:{prev:?}", r.batch_id, r.decision).as_bytes(),
                    )
            {
                return Err(LedgerError::Invalid);
            }
            prev = r.record_hash;
        }
        Ok(())
    }
    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cs_plan::MitigationPlan;
    #[test]
    fn chain_verifies() {
        let p = MitigationPlan {
            steps: vec![],
            spent: 0,
            expected_reduction: 0,
        };
        let mut l = AppendOnlyLedger::default();
        l.append("b", "deny", cs_rvf::seal("b", &p).unwrap())
            .unwrap();
        l.verify().unwrap();
    }
}
