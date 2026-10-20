# Architecture
## Thesis
Policy, memory, causal intervention, risk control and cryptographic evidence form one prospective execution boundary.
## Flow
Validate DAG -> graph -> Cedar -> RuVector recall -> interventions -> responsibility -> risk -> mitigation -> RVF seal -> verify -> one-use grant -> recovery.
## Workspace map
|#|Crate|Context|Typed API|Callers|Acceptance|Trace|
|---:|---|---|---|---|---|---|
|1|`cs-types`|Core|`IDs/RiskScore`|all|unit+E2E|ADR-0001/DDD-0001|
|2|`cs-action`|Intake|`ActionIntent/Batch`|orchestrator/api|unit+E2E|ADR-0002/DDD-0002|
|3|`cs-resource-graph`|Graph|`ResourceGraph`|counterfactual/plan|unit+E2E|ADR-0003/DDD-0003|
|4|`cs-policy-model`|Auth|`PolicyDecision`|cedar/gate|unit+E2E|ADR-0004/DDD-0004|
|5|`cs-cedar`|Auth|`CedarAuthorizer`|orchestrator|unit+E2E|ADR-0005/DDD-0005|
|6|`cs-incidents`|Memory|`IncidentCorpus`|embedding/eval|unit+E2E|ADR-0006/DDD-0006|
|7|`cs-embedding`|Memory|`IncidentEmbedding`|ruvector|unit+E2E|ADR-0007/DDD-0007|
|8|`cs-ruvector`|Memory|`RuVectorRecall`|orchestrator|unit+E2E|ADR-0008/DDD-0008|
|9|`cs-counterfactual`|Causal|`InterventionEngine`|attribution/plan|unit+E2E|ADR-0009/DDD-0009|
|10|`cs-attribution`|Causal|`ResponsibilityScore`|plan/eval|unit+E2E|ADR-0010/DDD-0010|
|11|`cs-risk-control`|Risk|`RiskCertificate`|plan/gate|unit+E2E|ADR-0011/DDD-0011|
|12|`cs-plan`|Planning|`MitigationPlan`|orchestrator|unit+E2E|ADR-0012/DDD-0012|
|13|`cs-rvf`|Evidence|`EvidenceSeal`|ledger/verifier|unit+E2E|ADR-0013/DDD-0001|
|14|`cs-ledger`|Evidence|`DecisionLedger`|orchestrator/api|unit+E2E|ADR-0014/DDD-0002|
|15|`cs-verifier`|Evidence|`VerificationReport`|gate|unit+E2E|ADR-0015/DDD-0003|
|16|`cs-executor-gate`|Execution|`ExecutionGrant`|orchestrator/api|unit+E2E|ADR-0016/DDD-0004|
|17|`cs-recovery`|Recovery|`RecoveryPlan`|orchestrator|unit+E2E|ADR-0017/DDD-0005|
|18|`cs-config`|Runtime|`ShieldConfig`|orchestrator/api/cli|unit+E2E|ADR-0018/DDD-0006|
|19|`cs-telemetry`|Runtime|`DecisionMetrics`|orchestrator/eval|unit+E2E|ADR-0019/DDD-0007|
|20|`cs-evaluation`|Evaluation|`EvaluationReport`|api/cli|unit+E2E|ADR-0020/DDD-0008|
|21|`cs-orchestrator`|Application|`ShieldOrchestrator`|api/cli|unit+E2E|ADR-0021/DDD-0009|
|22|`cs-api`|Interface|`ShieldService`|cli|unit+E2E|ADR-0022/DDD-0010|
|23|`cs-cli`|Interface|`binary`|entrypoint|unit+E2E|ADR-0023/DDD-0011|

Every crate has domain tests and real callers. Domain crates never import adapters. Orchestration owns sequencing. Interfaces call the service facade. Every library has a reverse caller; cs-cli is the executable root.
## Failure model
Malformed graphs, Cedar errors, empty recall, excess risk, corruption, timeout and replay fail closed. Recovery is not permission.
## Runtime
Linux-first Rust 1.99, deterministic seeds, bounded inputs, environment configuration and secret-free telemetry.
