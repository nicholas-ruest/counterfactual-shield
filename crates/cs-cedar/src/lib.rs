use cedar_policy::{Authorizer, Context, Decision, Entities, PolicySet, Request};
use cs_action::ActionIntent;
use cs_policy_model::{PolicyDecision, PolicyReceipt};
use std::str::FromStr;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum CedarError {
    #[error("invalid Cedar policy or request: {0}")]
    Invalid(String),
}
pub struct CedarAuthorizer {
    policies: PolicySet,
}
impl CedarAuthorizer {
    pub fn from_source(source: &str) -> Result<Self, CedarError> {
        PolicySet::from_str(source)
            .map(|policies| Self { policies })
            .map_err(|e| CedarError::Invalid(e.to_string()))
    }
    pub fn authorize(&self, a: &ActionIntent) -> Result<PolicyReceipt, CedarError> {
        let p = format!("User::\"{}\"", a.actor.as_str())
            .parse()
            .map_err(|e: cedar_policy::ParseErrors| CedarError::Invalid(e.to_string()))?;
        let v = format!("Action::\"{}\"", a.verb)
            .parse()
            .map_err(|e: cedar_policy::ParseErrors| CedarError::Invalid(e.to_string()))?;
        let r = format!("Resource::\"{}\"", a.resource.as_str())
            .parse()
            .map_err(|e: cedar_policy::ParseErrors| CedarError::Invalid(e.to_string()))?;
        let req = Request::new(p, v, r, Context::empty(), None)
            .map_err(|e| CedarError::Invalid(e.to_string()))?;
        let response = Authorizer::new().is_authorized(&req, &self.policies, &Entities::empty());
        let decision = if response.decision() == Decision::Allow {
            PolicyDecision::Permit
        } else {
            PolicyDecision::Deny
        };
        Ok(PolicyReceipt {
            decision,
            policy_count: self.policies.policies().count(),
            diagnostics: response
                .diagnostics()
                .errors()
                .map(ToString::to_string)
                .collect(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cs_types::Id;
    fn act(actor: &str) -> ActionIntent {
        ActionIntent {
            id: Id::new("a").unwrap(),
            actor: Id::new(actor).unwrap(),
            verb: "deploy".into(),
            resource: Id::new("service").unwrap(),
            criticality: 8,
            cost: 2,
            depends_on: vec![],
        }
    }
    #[test]
    fn real_cedar_permit_and_deny() {
        let c=CedarAuthorizer::from_source("permit(principal == User::\"ops\", action == Action::\"deploy\", resource == Resource::\"service\");").unwrap();
        assert!(c.authorize(&act("ops")).unwrap().decision.permits());
        assert!(!c.authorize(&act("other")).unwrap().decision.permits());
    }
}
