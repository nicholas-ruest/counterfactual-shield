use cs_action::ActionBatch;
use cs_resource_graph::ResourceGraph;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InterventionResult {
    pub action_id: String,
    pub baseline_critical: usize,
    pub without_action: usize,
    pub marginal_reduction: usize,
}
pub struct InterventionEngine;
impl InterventionEngine {
    pub fn analyze(batch: &ActionBatch, graph: &ResourceGraph) -> Vec<InterventionResult> {
        let base = graph.critical_reach(batch, None);
        batch
            .actions
            .iter()
            .map(|a| {
                let without = graph.critical_reach(batch, Some(&a.id));
                InterventionResult {
                    action_id: a.id.to_string(),
                    baseline_critical: base,
                    without_action: without,
                    marginal_reduction: base.saturating_sub(without),
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cs_action::ActionIntent;
    use cs_types::Id;
    #[test]
    fn removal_reduces_blast() {
        let id = |s| Id::new(s).unwrap();
        let a = ActionIntent {
            id: id("a"),
            actor: id("u"),
            verb: "x".into(),
            resource: id("r"),
            criticality: 8,
            cost: 1,
            depends_on: vec![],
        };
        let b = ActionBatch::new(id("b"), vec![a]).unwrap();
        let mut g = ResourceGraph::default();
        g.add_resource(id("r"), 8);
        let x = InterventionEngine::analyze(&b, &g);
        assert_eq!(x[0].marginal_reduction, 1);
    }
}
