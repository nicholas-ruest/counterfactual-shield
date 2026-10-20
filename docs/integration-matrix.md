# Integration matrix
|Ingredient|Pin|Boundary|Crate|Test|Ablation|
|---|---|---|---|---|---|
|Cedar|4.14.0/34db4c6|policy/entities/request|cs-cedar|permit/forbid/error|unsafe permit|
|RuVector|2.4.0/46d816d|VectorDB insert/search/get|cs-ruvector|readback|zero recall|
|RVF|0.2.0/46d816d|witness create/verify|cs-rvf|tamper reject|no evidence|
|Conformal risk|Bates 2024|upper loss bound|cs-risk-control|coverage/fail closed|no robust gate|
|Ruflo|3.57.2/7be330b|coordination receipt|evidence|version/topology|authority none|
|MetaHarness|e0dfd44|Darwin/Flywheel receipts|evidence|hash/replay|authority none|
