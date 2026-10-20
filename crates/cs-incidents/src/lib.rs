use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub action_kind: String,
    pub severity: f32,
    pub affected: u32,
    pub summary: String,
}
impl Incident {
    pub fn new(
        id: &str,
        kind: &str,
        severity: f32,
        affected: u32,
        summary: &str,
    ) -> Result<Self, &'static str> {
        if id.is_empty()
            || kind.is_empty()
            || !severity.is_finite()
            || !(0.0..=1.0).contains(&severity)
            || summary.len() > 512
        {
            return Err("invalid incident");
        }
        Ok(Self {
            id: id.into(),
            action_kind: kind.into(),
            severity,
            affected,
            summary: summary.into(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates() {
        assert!(Incident::new("i", "deploy", 0.8, 3, "rollback").is_ok());
        assert!(Incident::new("i", "", 0.8, 3, "x").is_err());
    }
}
