use cs_action::ActionBatch;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Compensation {
    pub action_id: String,
    pub command: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub steps: Vec<Compensation>,
}
pub fn plan(batch: &ActionBatch) -> RecoveryPlan {
    let steps = batch
        .actions
        .iter()
        .rev()
        .map(|a| Compensation {
            action_id: a.id.to_string(),
            command: format!("compensate:{}", a.verb),
        })
        .collect();
    RecoveryPlan { steps }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cs_action::ActionIntent;
    use cs_types::Id;
    #[test]
    fn reverses_order() {
        let id = |s| Id::new(s).unwrap();
        let a = |s| ActionIntent {
            id: id(s),
            actor: id("u"),
            verb: s.into(),
            resource: id("r"),
            criticality: 1,
            cost: 1,
            depends_on: vec![],
        };
        let b = ActionBatch::new(id("b"), vec![a("a"), a("z")]).unwrap();
        assert_eq!(plan(&b).steps[0].action_id, "z");
    }
}
