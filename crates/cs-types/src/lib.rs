use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Id(String);
impl Id {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 96
            || !value
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ':' | '.'))
        {
            return Err("invalid identifier");
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RiskScore(f64);
impl RiskScore {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err("risk must be finite in [0,1]")
        }
    }
    pub const fn get(self) -> f64 {
        self.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_boundaries() {
        assert!(Id::new("agent:ops").is_ok());
        assert!(Id::new("bad value").is_err());
        assert!(RiskScore::new(1.1).is_err());
    }
}
