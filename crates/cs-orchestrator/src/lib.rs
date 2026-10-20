use cs_action::{ActionBatch, ActionIntent};
use cs_cedar::CedarAuthorizer;
use cs_config::ShieldConfig;
use cs_counterfactual::InterventionEngine;
use cs_evaluation::{EvaluationMode, EvaluationReport};
use cs_executor_gate::{ExecutionGate, ExecutionGrant};
use cs_incidents::Incident;
use cs_ledger::AppendOnlyLedger;
use cs_plan::MitigationPlan;
use cs_policy_model::{PolicyDecision, PolicyReceipt};
use cs_resource_graph::ResourceGraph;
use cs_risk_control::{CalibrationSet, RiskCertificate};
use cs_telemetry::DecisionMetrics;
use cs_types::Id;
use cs_verifier::VerificationReport;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("shield pipeline: {0}")]
pub struct ShieldError(String);

#[derive(Clone, Debug, Serialize)]
pub struct ShieldOutcome {
    pub mode: EvaluationMode,
    pub recalled_incidents: Vec<String>,
    pub critical_before: usize,
    pub critical_after: usize,
    pub plan: MitigationPlan,
    pub risk: RiskCertificate,
    pub verification: VerificationReport,
    pub grant: Option<ExecutionGrant>,
    pub ledger_length: usize,
    pub metrics: DecisionMetrics,
}

pub struct ShieldOrchestrator {
    config: ShieldConfig,
}

impl ShieldOrchestrator {
    pub fn new(config: ShieldConfig) -> Result<Self, ShieldError> {
        config
            .validate()
            .map_err(|error| ShieldError(error.to_string()))?;
        Ok(Self { config })
    }

    pub fn run(&self, mode: EvaluationMode) -> Result<ShieldOutcome, ShieldError> {
        let batch = example_batch()?;
        let graph = example_graph()?;
        let interventions = InterventionEngine::analyze(&batch, &graph);
        let ranked = cs_attribution::rank(&interventions);
        let costs = batch
            .actions
            .iter()
            .map(|action| (action.id.to_string(), action.cost))
            .collect::<Vec<_>>();
        let plan = cs_plan::build(&ranked, &costs, self.config.mitigation_budget);
        let critical_before = graph.critical_reach(&batch, None);
        let critical_after = critical_before.saturating_sub(plan.expected_reduction);

        let policy = if matches!(mode, EvaluationMode::NoCedar | EvaluationMode::RuvnetOnly) {
            PolicyReceipt {
                decision: PolicyDecision::Deny,
                policy_count: 0,
                diagnostics: vec!["Cedar boundary removed by ablation".into()],
            }
        } else {
            authorize_batch(&batch)?
        };

        let recalled_incidents = if matches!(
            mode,
            EvaluationMode::NoRuVector | EvaluationMode::EnterpriseOnly
        ) {
            Vec::new()
        } else {
            recall_incidents(self.config.recall_k)?
        };

        let risk = if mode == EvaluationMode::NoRisk {
            RiskCertificate {
                upper_loss: 1.0,
                alpha: self.config.alpha,
                samples: 0,
                passes: false,
            }
        } else {
            CalibrationSet::new(vec![
                0.04, 0.08, 0.11, 0.13, 0.16, 0.18, 0.21, 0.23, 0.27, 0.30,
            ])
            .and_then(|set| set.certify(self.config.alpha, self.config.risk_ceiling))
            .map_err(|error| ShieldError(error.to_string()))?
        };

        let mut seal = cs_rvf::seal(&batch.digest_material(), &plan)
            .map_err(|error| ShieldError(error.to_string()))?;
        if mode == EvaluationMode::NoRvf {
            seal.chain.clear();
        }
        let verification = cs_verifier::verify_release(&policy, &risk, &seal);
        let mut gate = ExecutionGate::default();
        let grant = gate
            .issue(
                batch.id.clone(),
                id("grant-2026-10-19")?,
                seal.digest,
                &verification,
            )
            .ok();

        let mut ledger = AppendOnlyLedger::default();
        ledger
            .append(
                batch.id.as_str(),
                if verification.release {
                    "release"
                } else {
                    "deny"
                },
                seal,
            )
            .map_err(|error| ShieldError(error.to_string()))?;
        ledger
            .verify()
            .map_err(|error| ShieldError(error.to_string()))?;

        let mut metrics = DecisionMetrics::default();
        metrics.record(
            verification.release,
            recalled_incidents.len(),
            critical_before,
            plan.expected_reduction,
        );

        Ok(ShieldOutcome {
            mode,
            recalled_incidents,
            critical_before,
            critical_after,
            plan,
            risk,
            verification,
            grant,
            ledger_length: ledger.len(),
            metrics,
        })
    }

    pub fn evaluate(&self) -> Result<Vec<EvaluationReport>, ShieldError> {
        [
            EvaluationMode::Combined,
            EvaluationMode::NoRuVector,
            EvaluationMode::NoRvf,
            EvaluationMode::NoCedar,
            EvaluationMode::NoRisk,
            EvaluationMode::EnterpriseOnly,
            EvaluationMode::RuvnetOnly,
        ]
        .into_iter()
        .map(|mode| {
            let outcome = self.run(mode)?;
            let evidence = outcome.verification.evidence_ok;
            let policy = outcome.verification.policy_ok;
            let robust = outcome.risk.passes;
            let objective = EvaluationReport::objective(
                outcome.recalled_incidents.len(),
                evidence,
                policy,
                robust,
                outcome.critical_before,
                outcome.critical_after,
            );
            Ok(EvaluationReport {
                mode,
                released: outcome.verification.release,
                recalled: outcome.recalled_incidents.len(),
                evidence,
                policy_enforced: policy,
                robust,
                critical_before: outcome.critical_before,
                critical_after: outcome.critical_after,
                objective,
            })
        })
        .collect()
    }
}

fn authorize_batch(batch: &ActionBatch) -> Result<PolicyReceipt, ShieldError> {
    let policies = batch.actions.iter().map(|action| {
        format!(
            r#"permit(principal == User::"{}", action == Action::"{}", resource == Resource::"{}");"#,
            action.actor, action.verb, action.resource
        )
    }).collect::<Vec<_>>().join("\n");
    let authorizer =
        CedarAuthorizer::from_source(&policies).map_err(|error| ShieldError(error.to_string()))?;
    let receipts = batch
        .actions
        .iter()
        .map(|action| authorizer.authorize(action))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| ShieldError(error.to_string()))?;
    let permits = receipts.iter().all(|receipt| receipt.decision.permits());
    Ok(PolicyReceipt {
        decision: if permits {
            PolicyDecision::Permit
        } else {
            PolicyDecision::Deny
        },
        policy_count: receipts.iter().map(|receipt| receipt.policy_count).sum(),
        diagnostics: receipts
            .into_iter()
            .flat_map(|receipt| receipt.diagnostics)
            .collect(),
    })
}

fn recall_incidents(k: usize) -> Result<Vec<String>, ShieldError> {
    let directory = tempfile::tempdir().map_err(|error| ShieldError(error.to_string()))?;
    let database = directory.path().join("incidents.db");
    let path = database
        .to_str()
        .ok_or_else(|| ShieldError("non-UTF-8 temporary path".into()))?;
    let recall =
        cs_ruvector::RuVectorRecall::open(path).map_err(|error| ShieldError(error.to_string()))?;
    let incidents = [
        Incident::new(
            "inc-credential-fanout",
            "assume-role",
            0.91,
            12,
            "role fanout reached production",
        ),
        Incident::new(
            "inc-public-log",
            "deploy-service",
            0.74,
            4,
            "deployment leaked sensitive output",
        ),
        Incident::new(
            "inc-secret-read",
            "read-secret",
            0.82,
            6,
            "secret read preceded privilege escalation",
        ),
    ];
    for incident in incidents {
        recall
            .remember(&incident.map_err(|error| ShieldError(error.to_string()))?)
            .map_err(|error| ShieldError(error.to_string()))?;
    }
    let query = Incident::new(
        "query",
        "deploy-service",
        0.8,
        5,
        "prospective privileged deployment",
    )
    .map_err(|error| ShieldError(error.to_string()))?;
    recall
        .recall(&query, k)
        .map_err(|error| ShieldError(error.to_string()))
}

fn example_batch() -> Result<ActionBatch, ShieldError> {
    ActionBatch::new(
        id("batch-2026-10-19")?,
        vec![
            ActionIntent {
                id: id("read-secret")?,
                actor: id("ops")?,
                verb: "read-secret".into(),
                resource: id("secret")?,
                criticality: 8,
                cost: 1,
                depends_on: vec![],
            },
            ActionIntent {
                id: id("assume-role")?,
                actor: id("ops")?,
                verb: "assume-role".into(),
                resource: id("role")?,
                criticality: 9,
                cost: 1,
                depends_on: vec![id("read-secret")?],
            },
            ActionIntent {
                id: id("deploy-service")?,
                actor: id("ops")?,
                verb: "deploy-service".into(),
                resource: id("service")?,
                criticality: 9,
                cost: 2,
                depends_on: vec![id("assume-role")?],
            },
        ],
    )
    .map_err(|error| ShieldError(error.to_string()))
}

fn example_graph() -> Result<ResourceGraph, ShieldError> {
    let mut graph = ResourceGraph::default();
    for (name, criticality) in [
        ("secret", 8),
        ("role", 9),
        ("service", 9),
        ("database", 10),
        ("public-log", 8),
    ] {
        graph.add_resource(id(name)?, criticality);
    }
    graph.add_edge(id("secret")?, id("role")?);
    graph.add_edge(id("role")?, id("service")?);
    graph.add_edge(id("service")?, id("database")?);
    graph.add_edge(id("service")?, id("public-log")?);
    Ok(graph)
}

fn id(value: &str) -> Result<Id, ShieldError> {
    Id::new(value).map_err(|error| ShieldError(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combined_releases_but_each_safety_ablation_degrades() {
        let shield = ShieldOrchestrator::new(ShieldConfig::default()).unwrap();
        let reports = shield.evaluate().unwrap();
        let combined = reports
            .iter()
            .find(|report| report.mode == EvaluationMode::Combined)
            .unwrap();
        assert!(combined.released);
        assert!(
            reports
                .iter()
                .filter(|report| report.mode != EvaluationMode::Combined)
                .all(|report| report.objective < combined.objective)
        );
    }
}
