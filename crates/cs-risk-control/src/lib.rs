use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalibrationSet {
    losses: Vec<f64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskCertificate {
    pub upper_loss: f64,
    pub alpha: f64,
    pub samples: usize,
    pub passes: bool,
}
#[derive(Debug, Error)]
pub enum RiskError {
    #[error("invalid calibration")]
    Invalid,
}
impl CalibrationSet {
    pub fn new(losses: Vec<f64>) -> Result<Self, RiskError> {
        if losses.len() < 4
            || losses
                .iter()
                .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
        {
            return Err(RiskError::Invalid);
        }
        Ok(Self { losses })
    }
    pub fn certify(&self, alpha: f64, ceiling: f64) -> Result<RiskCertificate, RiskError> {
        if !(0.0..1.0).contains(&alpha) || !(0.0..=1.0).contains(&ceiling) {
            return Err(RiskError::Invalid);
        }
        let mut x = self.losses.clone();
        x.sort_by(f64::total_cmp);
        let raw = ((x.len() + 1) as f64 * (1.0 - alpha)).ceil() as usize;
        let idx = raw.saturating_sub(1).min(x.len() - 1);
        let upper = x[idx];
        Ok(RiskCertificate {
            upper_loss: upper,
            alpha,
            samples: x.len(),
            passes: upper <= ceiling,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gates_high_risk() {
        let low = CalibrationSet::new(vec![
            0.05, 0.1, 0.12, 0.15, 0.18, 0.2, 0.22, 0.25, 0.27, 0.3,
        ])
        .unwrap();
        assert!(low.certify(0.2, 0.35).unwrap().passes);
        let high = CalibrationSet::new(vec![0.1, 0.2, 0.4, 0.7]).unwrap();
        assert!(!high.certify(0.2, 0.35).unwrap().passes);
    }
}
