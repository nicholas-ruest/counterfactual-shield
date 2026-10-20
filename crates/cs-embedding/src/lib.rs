use cs_incidents::Incident;
pub const DIM: usize = 4;
pub fn embed(i: &Incident) -> [f32; DIM] {
    let kind = i
        .action_kind
        .bytes()
        .fold(0u32, |a, b| a.wrapping_add(u32::from(b)));
    [
        i.severity,
        (i.affected.min(100) as f32) / 100.0,
        (kind % 101) as f32 / 100.0,
        (i.summary.len() % 101) as f32 / 100.0,
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic() {
        let i = Incident::new("i", "deploy", 0.8, 3, "rollback").unwrap();
        assert_eq!(embed(&i), embed(&i));
    }
}
