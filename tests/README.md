# Integration tests

The executable integration test is `crates/cs-api/tests/end_to_end.rs`. It invokes the public service facade and exercises Cedar authorization, RuVector insertion/search, intervention attribution, conformal risk certification, RVF witness verification, execution grants, and the append-only ledger in one workload. Run it with:

```sh
cargo test -p cs-api --test end_to_end --locked
```
