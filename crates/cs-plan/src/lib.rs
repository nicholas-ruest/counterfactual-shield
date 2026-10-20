use cs_attribution::ResponsibilityScore;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MitigationStep {
    pub action_id: String,
    pub cost: u32,
    pub expected_reduction: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MitigationPlan {
    pub steps: Vec<MitigationStep>,
    pub spent: u32,
    pub expected_reduction: usize,
}
pub fn build(
    ranked: &[ResponsibilityScore],
    costs: &[(String, u32)],
    budget: u32,
) -> MitigationPlan {
    let mut scored = ranked
        .iter()
        .filter_map(|r| {
            costs
                .iter()
                .find(|(id, _)| id == &r.action_id)
                .map(|(_, c)| (r, *c))
        })
        .collect::<Vec<_>>();
    scored.sort_by(|(a, ac), (b, bc)| {
        let ar = a.reduction as f64 / f64::from((*ac).max(1));
        let br = b.reduction as f64 / f64::from((*bc).max(1));
        br.total_cmp(&ar)
            .then_with(|| a.action_id.cmp(&b.action_id))
    });
    let mut out = MitigationPlan {
        steps: vec![],
        spent: 0,
        expected_reduction: 0,
    };
    for (r, c) in scored {
        if out.spent + c <= budget && r.reduction > 0 {
            out.spent += c;
            out.expected_reduction += r.reduction;
            out.steps.push(MitigationStep {
                action_id: r.action_id.clone(),
                cost: c,
                expected_reduction: r.reduction,
            });
        }
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn respects_budget() {
        let r = vec![ResponsibilityScore {
            action_id: "a".into(),
            score: 1.0,
            reduction: 3,
        }];
        let p = build(&r, &[("a".into(), 2)], 2);
        assert_eq!(p.spent, 2);
        assert_eq!(p.expected_reduction, 3);
    }
}
