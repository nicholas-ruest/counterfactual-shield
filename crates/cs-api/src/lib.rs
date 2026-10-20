use cs_config::ShieldConfig;
use cs_evaluation::EvaluationReport;
use cs_orchestrator::{ShieldError, ShieldOrchestrator, ShieldOutcome};
pub struct ShieldService {
    orchestrator: ShieldOrchestrator,
}
impl ShieldService {
    pub fn new(config: ShieldConfig) -> Result<Self, ShieldError> {
        Ok(Self {
            orchestrator: ShieldOrchestrator::new(config)?,
        })
    }
    pub fn example(&self) -> Result<ShieldOutcome, ShieldError> {
        self.orchestrator
            .run(cs_evaluation::EvaluationMode::Combined)
    }
    pub fn evaluate(&self) -> Result<Vec<EvaluationReport>, ShieldError> {
        self.orchestrator.evaluate()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn facade_runs_vertical_slice() {
        let s = ShieldService::new(ShieldConfig::default()).unwrap();
        assert!(s.example().unwrap().verification.release);
    }
}
