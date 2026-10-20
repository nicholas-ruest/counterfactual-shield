#!/usr/bin/env sh
set -eu
runs="${1:-5}"
cargo build --release --locked -p cs-cli >/dev/null
i=1
while [ "$i" -le "$runs" ]; do
  start=$(date +%s%N); target/release/cs-cli evaluate >/dev/null; finish=$(date +%s%N); echo "run=$i elapsed_ns=$((finish - start))"
  i=$((i + 1))
done
