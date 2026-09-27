#!/usr/bin/env bash
# Regenerates output.txt. Assumes `just clone` has already run.
set -u
ulimit -c 0
cd "$(dirname "$0")" || exit 1
O=output.txt
: > "$O"

run() { # run <label> <cmd...>
  local label="$1"; shift
  { echo; echo "### $label"; echo "\$ $*"; } >> "$O"
  ( cd repro && "$@" ) >> "$O" 2>&1
  echo "(exit code: $?)" >> "$O"
}

{
  echo "### Environment"
  rustc -V
  cargo -V
  uname -srm
} >> "$O" 2>&1

( cd clone && git checkout -- . )
run "BEFORE the fix - pristine clone at base_commit, debug profile (overflow-checks = true)" cargo run
run "BEFORE the fix - same pristine clone, RELEASE profile (overflow checks off; note seek() now returns Ok with a garbage chunk offset instead of panicking)" cargo run --release

{ echo; echo "### Applying patch.diff"; echo "\$ cd clone && git apply ../patch.diff"; } >> "$O"
( cd clone && git apply ../patch.diff ) >> "$O" 2>&1
echo "(exit code: $?)" >> "$O"

run "AFTER the fix - debug profile" cargo run
run "AFTER the fix - RELEASE profile" cargo run --release

{
  echo
  echo "### Repeatability - 3 consecutive debug runs against the pristine clone"
} >> "$O"
( cd clone && git checkout -- . )
for i in 1 2 3; do
  { echo; echo "-- run $i"; } >> "$O"
  ( cd repro && cargo run --quiet ) >> "$O" 2>&1
  echo "(exit code: $?)" >> "$O"
done
( cd clone && git checkout -- . )
