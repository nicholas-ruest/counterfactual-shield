use cs_counterfactual::InterventionResult;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponsibilityScore {
    pub action_id: String,
    pub score: f64,
    pub reduction: usize,
}
pub fn rank(results: &[InterventionResult]) -> Vec<ResponsibilityScore> {
    let total = results
        .iter()
        .map(|r| r.marginal_reduction)
        .sum::<usize>()
        .max(1);
    let mut out = results
        .iter()
        .map(|r| ResponsibilityScore {
            action_id: r.action_id.clone(),
            score: r.marginal_reduction as f64 / total as f64,
            reduction: r.marginal_reduction,
        })
        .collect::<Vec<_>>();
    out.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.action_id.cmp(&b.action_id))
    });
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ranks_marginal_cause() {
        let a = InterventionResult {
            action_id: "a".into(),
            baseline_critical: 3,
            without_action: 1,
            marginal_reduction: 2,
        };
        let b = InterventionResult {
            action_id: "b".into(),
            baseline_critical: 3,
            without_action: 2,
            marginal_reduction: 1,
        };
        assert_eq!(rank(&[b, a])[0].action_id, "a");
    }
}
