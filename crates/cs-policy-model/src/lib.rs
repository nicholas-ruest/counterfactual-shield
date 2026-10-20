use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PolicyDecision {
    Permit,
    Deny,
    Error,
}
impl PolicyDecision {
    pub const fn permits(self) -> bool {
        matches!(self, Self::Permit)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyReceipt {
    pub decision: PolicyDecision,
    pub policy_count: usize,
    pub diagnostics: Vec<String>,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_permit_allows() {
        assert!(PolicyDecision::Permit.permits());
        assert!(!PolicyDecision::Error.permits());
    }
}
