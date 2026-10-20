use cs_action::ActionBatch;
use cs_types::Id;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResourceGraph {
    criticality: BTreeMap<Id, u8>,
    edges: BTreeMap<Id, BTreeSet<Id>>,
}
impl ResourceGraph {
    pub fn add_resource(&mut self, id: Id, criticality: u8) {
        self.criticality.insert(id, criticality.min(10));
    }
    pub fn add_edge(&mut self, from: Id, to: Id) {
        self.edges.entry(from).or_default().insert(to);
    }
    pub fn reachable(&self, start: &Id) -> BTreeSet<Id> {
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::from([start.clone()]);
        while let Some(n) = q.pop_front() {
            if !seen.insert(n.clone()) {
                continue;
            }
            if let Some(next) = self.edges.get(&n) {
                q.extend(next.iter().cloned());
            }
        }
        seen
    }
    pub fn critical_reach(&self, batch: &ActionBatch, skip: Option<&Id>) -> usize {
        batch
            .actions
            .iter()
            .filter(|a| skip != Some(&a.id))
            .flat_map(|a| self.reachable(&a.resource))
            .collect::<BTreeSet<_>>()
            .iter()
            .filter(|r| self.criticality.get(*r).copied().unwrap_or_default() >= 7)
            .count()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cycles_terminate() {
        let a = Id::new("a").unwrap();
        let b = Id::new("b").unwrap();
        let mut g = ResourceGraph::default();
        g.add_edge(a.clone(), b.clone());
        g.add_edge(b, a.clone());
        assert_eq!(g.reachable(&a).len(), 2);
    }
}
