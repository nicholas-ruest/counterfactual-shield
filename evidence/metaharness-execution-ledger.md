# MetaHarness execution ledger

## Darwin

- Package: `@metaharness/darwin@0.12.0`
- Commands:
  - `bench create /home/ruv/work/counterfactual-shield --out evidence/raw/darwin-suite.json`
  - `bench verify evidence/raw/darwin-suite.json`
- Result: one repo-native task; suite hash `e66652cfd99f&`; verification exit 0.
- Authority: advisory structural evidence only.

## Flywheel

- Package: `@metaharness/flywheel@0.1.12`
- Boundary: library import of `meetsPromotionRule` and `gateFingerprint`.
- Evidence: combined objective 28 versus enterprise-only objective 22; a frozen advisory promotion comparison with no-op rate 0 versus 1 and cost-per-win 0.8 versus 1.
- Result: `promote:true`; frozen gate fingerprint `c5942e7c9bf8f28fe7af47a6ec3887d11eeb3f8c8e8601cc6a7a54969ea0b981`.
- Raw receipt: `evidence/raw/flywheel-promotion.json`.
- Authority: advisory only. Rust evaluation and exact command receipts remain the source for product claims.
