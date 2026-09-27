#!/usr/bin/env bash
set -euo pipefail
CLIF="${CLIF:-$(cd "$(dirname "$0")/../../.." && pwd)}"
HODOW="${HODOW:-$HOME/Projekty/Rust/hodow}"
# Features that make cg_clif fail to *compile* (hard "Unknown SIMD intrinsic" error) - excluded
# so the rest of the suite can be measured; counted separately as "not implemented".
EXCLUDE="${EXCLUDE-}"

FEATS=$(rustc --print cfg -Ctarget-cpu=native | grep '^target_feature' | sed 's/target_feature="//;s/"$//' | sort)
CFGS="-A explicit_builtin_cfgs_in_flags ${TARGET_CPU_FLAG-"-Ctarget-cpu=native"}"
for f in $FEATS; do
  skip=0
  for e in $EXCLUDE; do [ "$f" = "$e" ] && skip=1; done
  [ $skip -eq 1 ] && continue
  CFGS="$CFGS --cfg target_feature=\"$f\""
done

cd "$HODOW"
echo "== EXCLUDED: $EXCLUDE =="
CARGO_TARGET_DIR="${OUTDIR:-$CLIF/tmp/target-clif}" RUSTFLAGS="$CFGS" \
  "$CLIF/dist/cargo-clif" build -p hodow_cli --no-default-features 2>&1
