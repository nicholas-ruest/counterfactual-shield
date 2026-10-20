use cs_api::ShieldService;
use cs_config::ShieldConfig;
use cs_evaluation::EvaluationMode;

#[test]
fn complete_system_outperforms_ingredient_ablations() {
    let service = ShieldService::new(ShieldConfig::default()).expect("valid configuration");
    let reports = service.evaluate().expect("real integrated evaluation");
    let combined = reports
        .iter()
        .find(|report| report.mode == EvaluationMode::Combined)
        .expect("combined report");
    assert!(combined.released);
    assert_eq!(combined.recalled, 3);
    assert!(combined.evidence && combined.policy_enforced && combined.robust);
    for report in reports
        .iter()
        .filter(|report| report.mode != EvaluationMode::Combined)
    {
        assert!(
            report.objective < combined.objective,
            "{:?} did not degrade",
            report.mode
        );
    }
}
