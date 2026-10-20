use cs_types::Id;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionIntent {
    pub id: Id,
    pub actor: Id,
    pub verb: String,
    pub resource: Id,
    pub criticality: u8,
    pub cost: u32,
    pub depends_on: Vec<Id>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionBatch {
    pub id: Id,
    pub actions: Vec<ActionIntent>,
}
#[derive(Debug, Error)]
pub enum BatchError {
    #[error("empty batch")]
    Empty,
    #[error("duplicate or missing dependency")]
    InvalidReference,
    #[error("dependency cycle")]
    Cycle,
}
impl ActionBatch {
    pub fn new(id: Id, actions: Vec<ActionIntent>) -> Result<Self, BatchError> {
        if actions.is_empty() {
            return Err(BatchError::Empty);
        }
        let map: BTreeMap<_, _> = actions.iter().map(|a| (a.id.clone(), a)).collect();
        if map.len() != actions.len()
            || actions
                .iter()
                .any(|a| a.depends_on.iter().any(|d| !map.contains_key(d)))
        {
            return Err(BatchError::InvalidReference);
        }
        fn visit(
            id: &Id,
            map: &BTreeMap<Id, &ActionIntent>,
            open: &mut BTreeSet<Id>,
            done: &mut BTreeSet<Id>,
        ) -> bool {
            if done.contains(id) {
                return true;
            }
            if !open.insert(id.clone()) {
                return false;
            }
            for d in &map[id].depends_on {
                if !visit(d, map, open, done) {
                    return false;
                }
            }
            open.remove(id);
            done.insert(id.clone());
            true
        }
        let mut done = BTreeSet::new();
        for id in map.keys() {
            if !visit(id, &map, &mut BTreeSet::new(), &mut done) {
                return Err(BatchError::Cycle);
            }
        }
        Ok(Self { id, actions })
    }
    pub fn digest_material(&self) -> String {
        self.actions
            .iter()
            .map(|a| format!("{}:{}:{}", a.id, a.verb, a.resource))
            .collect::<Vec<_>>()
            .join("|")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn id(s: &str) -> Id {
        Id::new(s).unwrap()
    }
    #[test]
    fn rejects_cycle() {
        let a = ActionIntent {
            id: id("a"),
            actor: id("u"),
            verb: "x".into(),
            resource: id("r"),
            criticality: 1,
            cost: 1,
            depends_on: vec![id("b")],
        };
        let b = ActionIntent {
            id: id("b"),
            depends_on: vec![id("a")],
            ..a.clone()
        };
        assert!(matches!(
            ActionBatch::new(id("q"), vec![a, b]),
            Err(BatchError::Cycle)
        ));
    }
}
