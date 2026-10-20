# Validation contract

A release candidate is valid only when every command below succeeds against the exact indexed commit.

| Gate | Command | Why |
|---|---|---|
| Formatting | `cargo fmt --all -- --check` | Stable source form |
| Lint | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Warnings are failures |
| Tests | `cargo test --workspace --locked` | Unit, failure-path, and integration behavior |
| End to end | `cargo test -p cs-api --test end_to_end --locked` | Public facade crosses every production boundary |
| Supply chain | `cargo deny check` | Advisories, licenses, bans, and sources |
| Benchmark | `scripts/benchmark.sh 5` | Reproducible warm release-mode latency/RSS |
| Ablation | `cargo run --locked -q -p cs-cli -- evaluate` | Combined, ingredient-only, and removal variants |
| Structure | pinned Dream Machine checker | 23 crates, 25 ADRs, 12 DDDs, documents, tests, visuals |

The frozen workload contains three dependent privileged actions and five resources. Success requires a single-use grant only when Cedar permits every action, the conformal upper loss is within the configured ceiling, and the RVF witness verifies. The objective combines prevented critical reach, incident recall, policy enforcement, evidence integrity, and risk control. An ablation is successful only if its score is lower than the combined system.

Raw receipts live under `evidence/`; results must not be copied into this document until observed.
