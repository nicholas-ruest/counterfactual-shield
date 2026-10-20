use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShieldConfig {
    pub risk_ceiling: f64,
    pub alpha: f64,
    pub mitigation_budget: u32,
    pub recall_k: usize,
}
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid safety configuration")]
    Invalid,
}
impl Default for ShieldConfig {
    fn default() -> Self {
        Self {
            risk_ceiling: 0.35,
            alpha: 0.2,
            mitigation_budget: 3,
            recall_k: 3,
        }
    }
}
impl ShieldConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if !(0.0..=1.0).contains(&self.risk_ceiling)
            || !(0.0..1.0).contains(&self.alpha)
            || self.mitigation_budget == 0
            || self.recall_k == 0
        {
            return Err(ConfigError::Invalid);
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_zero_budget() {
        let c = ShieldConfig {
            mitigation_budget: 0,
            ..ShieldConfig::default()
        };
        assert!(c.validate().is_err());
    }
}
