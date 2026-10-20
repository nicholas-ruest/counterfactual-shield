# Counterfactual Shield

![Counterfactual Shield overview](docs/assets/overview.svg)

Counterfactual Shield is a prospective authorization control plane for autonomous agents. Before an agent can execute a multi-step privileged tool chain, the system evaluates the full typed action DAG against enterprise policy, recalls similar incidents, estimates each action's causal contribution to blast radius, chooses mitigations within budget, certifies residual risk, seals the decision into RVF evidence, and issues a single-use grant only when every assurance holds.

This is not a policy wrapper or a risk-scoring CLI. It is a 23-crate Rust system whose public service path executes Cedar, RuVector, RVF cryptography, counterfactual analysis, conformal risk control, recovery, and append-only evidence together.

## Why it exists

Individual controls answer narrow questions: Cedar says whether an action matches policy; vector search recalls similar history; a resource graph models reachability; an evidence container detects tampering. Autonomous action chains need a stronger answer: *should this exact dependent chain execute now, what is likely to go wrong, which action causes the exposure, what mitigation fits the budget, and can the decision be independently verified later?*

### End-to-end workload

A deployment agent proposes:

1. read a production secret;
2. assume a privileged role;
3. deploy a service connected to a database and public log sink.

Counterfactual Shield validates the dependency DAG, evaluates each action with Cedar, indexes and searches incidents in RuVector, measures marginal critical-resource reach when each action is removed, constructs a mitigation plan, computes a finite-sample conformal upper loss bound, seals the batch and plan with an RVF witness chain, verifies all receipts, and emits a nonce-bound grant. A hash-linked decision ledger preserves the result.

## Quickstart

Requirements: Rust 1.99+ and a POSIX shell.

```sh
git clone https://github.com/nicholas-ruest/counterfactual-shield.git
cd counterfactual-shield
cargo run --locked -p cs-cli -- example
cargo run --locked -p cs-cli -- evaluate
```

The first command prints the complete decision, including recalled incidents, critical reach before and after mitigation, risk certificate, evidence verification, grant, ledger length, and metrics. The second runs the frozen combined system and six ablations.

## What release means

A grant exists only when:

```text
all Cedar receipts permit
AND conformal upper loss <= configured ceiling
AND RVF witness chain verifies
AND grant nonce has not been consumed
```

Invalid identifiers, empty/cyclic DAGs, malformed policy, risk-control misconfiguration, RVF tampering, empty evidence, and nonce replay are explicit errors or denials.

## Architecture

![Counterfactual Shield execution architecture](docs/assets/execution-flow.svg)

| Context | Crates | Production responsibility |
|---|---|---|
| Decision | `cs-types`, `cs-action`, `cs-resource-graph`, `cs-policy-model`, `cs-cedar` | Validated language, DAG invariants, reachability, receipts, real Cedar authorization |
| Intelligence | `cs-incidents`, `cs-embedding`, `cs-ruvector`, `cs-counterfactual`, `cs-attribution`, `cs-plan` | Incident memory, real vector search, interventions, responsibility, budgeted mitigation |
| Assurance | `cs-risk-control`, `cs-rvf`, `cs-verifier`, `cs-executor-gate`, `cs-recovery` | Conformal upper loss, real RVF witness, release conjunction, nonce grant, rollback order |
| Runtime and evidence | `cs-ledger`, `cs-config`, `cs-telemetry`, `cs-evaluation`, `cs-orchestrator`, `cs-api`, `cs-cli` | Audit chain, validated configuration, metrics, ablations, orchestration, interfaces |

The full dependency and caller map, public APIs, ADR/DDD references, implementation tasks, tests, and per-crate acceptance criteria are in [docs/architecture.md](docs/architecture.md).

## Executed composition

| Ingredient | Pin | Executed boundary | Evidence and ablation |
|---|---|---|---|
| Ruvnet RuVector | `ruvector-core 2.4.0`, upstream `46d816d` | `cs-ruvector` creates an on-disk 4D index, inserts incident vectors, and searches the real HNSW API | Unit and end-to-end tests; `NoRuVector` removes recall |
| Ruvnet RVF | `rvf-crypto 0.2.0`, upstream `46d816d` | `cs-rvf` calls `create_witness_chain`, SHAKE256, and `verify_witness_chain` | Tamper/empty-chain tests; `NoRvf` denies release |
| Ruvnet Ruflo | `3.57.2`, upstream `7be330b` | Governed build coordination and provenance, never application logic | Command receipt and explicit zero runtime authority |
| Ruvnet MetaHarness | `e0dfd44` | Darwin structural evaluation and Flywheel evidence capture | Source-bound execution ledger |
| AWS Cedar | `cedar-policy 4.14.0`, `34db4c6`, Apache-2.0 | `cs-cedar` parses policy, constructs typed requests, invokes `Authorizer` | Permit/deny test; `NoCedar` and enterprise-only baseline |
| Risk-controlling prediction sets | Bates et al., JACM 2024 / arXiv:2101.02703 | `cs-risk-control` uses a frozen finite-sample upper loss quantile as a release gate | Calibration failure test; `NoRisk` denial |

Exact repositories, revisions, licenses, APIs, tests, and ablations are recorded in [docs/integration-matrix.md](docs/integration-matrix.md).

## Evaluation

The dataset is deliberately small and frozen so every result is reproducible and reviewable: one three-action workload, five resources, three historical incidents, ten calibration losses, and no network calls.

| Mode | Expected capability loss |
|---|---|
| Combined | Release with policy, recall, evidence, and calibrated risk |
| NoRuVector | Incident recall becomes zero |
| NoRvf | Evidence verification fails; no grant |
| NoCedar | Policy condition fails; no grant |
| NoRisk | Robustness condition fails; no grant |
| EnterpriseOnly | Cedar remains, but Ruvnet recall is absent |
| RuvnetOnly | Recall/evidence remain, but enterprise policy is absent |

Run `cargo run --locked -q -p cs-cli -- evaluate` for the source-bound JSON. Raw observed results and benchmark receipts are in `evidence/`.

## Validation

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test -p cs-api --test end_to_end --locked
cargo deny check
scripts/benchmark.sh 5
```

The exact validation contract is [docs/validation.md](docs/validation.md). The pinned Dream Machine checker is a structural floor; semantic review additionally inspects real upstream calls, dependency edges, public APIs, failure paths, evidence, and README claims.

## Design record

- [Specification](docs/specification.md)
- [Architecture and 23-crate contract](docs/architecture.md)
- [Research and rejected alternatives](docs/research.md)
- [Implementation contract](docs/implementation-contract.md)
- [Traceability](docs/traceability.md)
- [Core-memory guidance map](docs/core-memory-guidance-map.md)
- [25 ADRs](docs/adrs/index.md)
- [12 DDD records](docs/ddd/index.md)
- [Validation](docs/validation.md)
- [Daily build ledger](docs/daily-build-ledger.md)

## Maturity and limitations

Counterfactual Shield is a research-grade reference implementation, not a production authorization service. The embedding is deterministic and intentionally inspectable, not semantic. The evaluation corpus is synthetic and small. The conformal certificate assumes exchangeability of calibration losses and does not establish safety under distribution shift. The current action/resource model does not ingest live cloud inventories or execute tools; it authorizes a frozen representative workload. RuVector is local and single-process. The append-only ledger is hash-linked but not externally timestamped. Cedar entities are intentionally minimal. Production adoption requires organization-specific schemas, policy review, external key custody, durable RVF storage, distributed nonce consumption, live incident ingestion, load testing, and security assessment.

## License

Apache-2.0. Third-party components retain their own licenses; see [docs/research.md](docs/research.md).
