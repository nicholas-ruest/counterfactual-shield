use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DecisionMetrics {
    pub evaluated: u64,
    pub permitted: u64,
    pub denied: u64,
    pub recalled: u64,
    pub critical_before: u64,
    pub critical_after: u64,
}
impl DecisionMetrics {
    pub fn record(&mut self, release: bool, recall: usize, before: usize, reduced: usize) {
        self.evaluated += 1;
        if release {
            self.permitted += 1
        } else {
            self.denied += 1
        }
        self.recalled += recall as u64;
        self.critical_before += before as u64;
        self.critical_after += before.saturating_sub(reduced) as u64;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accounts() {
        let mut m = DecisionMetrics::default();
        m.record(true, 2, 3, 1);
        assert_eq!((m.permitted, m.recalled, m.critical_after), (1, 2, 2));
    }
}
