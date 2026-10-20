# Core-Memory guidance map
|Source/SHA|Guidance|Decision|Evidence|State|
|---|---|---|---|---|
|AGENTS 0b2613acb2a5be8b5f47bfe1a89f9ecfafad64d3|20 crates; design first|23 mapped before Rust|design commit/metadata|APPLIED|
|standard a63fd1464eaf5eecbe919aac2e7699df24eabbdc|real composition|Cedar/RuVector/RVF/conformal|tests/receipts|APPLIED|
|CLAUDE 0178167276c981ff0ba503e2e564eee8d347e3f7|one writer, types, no secrets|isolated branch/bounds|git/security|APPLIED|
|task_start|empty unscreened is not success|LIVE_MEMORY_EMPTY_UNSCREENED|ledger|APPLIED|
|daily override|new daily system|novelty gate|daily ledger|APPLIED|
