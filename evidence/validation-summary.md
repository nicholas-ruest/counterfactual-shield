# Validation summary

Candidate date: 2026-10-19 (America/Toronto daily slot)

## Observed structure

- 23 workspace crates
- 24 Rust source files
- 1,515 Rust source lines before evidence-only documents
- 23 source-level tests plus one public integration test
- 25 substantive ADRs plus index/README
- 12 substantive DDD documents plus index/README
- 50 local dependency edges
- zero library crates without a caller; `cs-cli` is the executable entry point
- two project-specific animated SVGs

## Observed gates

| Gate | Result | Raw receipt |
|---|---|---|
| `cargo fmt --all -- --check` | exit 0 | `raw/cargo-fmt.txt` |
| strict all-target Clippy | exit 0 | `raw/cargo-clippy.txt` |
| `cargo test --workspace --locked` | exit 0 | `raw/cargo-test.txt` |
| public end-to-end integration | exit 0 | `raw/end-to-end.txt` |
| `cargo deny check` | exit 0; advisories/bans/licenses/sources pass | `raw/cargo-deny.txt` |
| caller/entrypoint audit | 23 crates, 50 edges, zero orphans | `raw/dependency-direction.json` |
| Darwin suite | hash verified | `raw/darwin-suite.json` |
| Flywheel advisory gate | promotion true | `raw/flywheel-promotion.json` |
| release-mode benchmark | five warm runs, 102.5145.6 ms | `raw/benchmark.txt` |
| combined/ablation evaluation | combined 28; every removal lower | `raw/ablation.json` |

The supply-chain policy contains one explicit waiver: RUSTSEC-2025-0141 reports unmaintained `bincode 2.0.1`, pulled transitively by pinned `ruvector-core 2.4.0`; the advisory reports no vulnerability and no compatible upstream replacement is currently available. This limitation is not hidden.

## Semantic evidence review

- Cedar is constructed from batch-specific policies and invoked for every action through `Authorizer::is_authorized`.
- RuVector creates a real on-disk database file, inserts three incident vectors, and executes HNSW search in the public workload.
- RVF calls `create_witness_chain`, `shake256_256`, and `verify_witness_chain`; tampered and empty evidence are rejected.
- Counterfactual evaluation actually removes each action and recomputes critical-resource reach.
- The risk certificate computes a finite-sample upper quantile from frozen calibration losses and gates release.
- The verifier is a conjunction; the execution grant is nonce-bound and single-use.
- The decision ledger is hash-linked and revalidated after append.
- The combined-system objective is 28.0. NoRuVector/enterprise-only score 22; NoRvf 25; NoCedar/Ruvnet-only 24; NoRisk 23.
- No upstream boundary is mocked in the end-to-end test.

Structural checker output and exact candidate commit are appended only after the candidate is indexed and checked.
