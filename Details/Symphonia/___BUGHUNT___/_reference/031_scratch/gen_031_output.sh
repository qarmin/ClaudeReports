#!/usr/bin/env bash
# Regenerates ___BUGHUNT___/031.*/output.txt from a fresh clone.
set -u
cd "$(dirname "$0")/../___BUGHUNT___/031.CORE_AUDIOBUFFER_NEW_ZERO_CHANNEL_DIV_BY_ZERO" || exit 1
ulimit -c 0

RAW=$(mktemp)
RUNLOG=$(mktemp)
strip() { grep -v -E '^warning|^\s*-->|^\s*=\s*note:|^\s*\|\s*$|^[0-9]+\s*\||^\.\.\.\s*$|^\s*\^|^\s+[0-9]+ \|'; }

{
  echo "Captured output of repro/ for candidate 031."
  echo "Paths shown are this candidate folder's own ./clone (created by \`just clone\` from"
  echo "https://github.com/pdeljanov/Symphonia.git at ee35874b571a35a9a6e15d3bc9a3aaf8f11fbeee)."
  echo "Toolchain: rustc stable-x86_64-unknown-linux-gnu, Linux x86_64. Compiler warnings stripped."
  echo
} > "$RAW"

just clone >/dev/null 2>&1 || { echo "clone failed" >&2; exit 1; }

{
  echo "### BEFORE the patch (pristine clone at ee35874), release build, 3 consecutive runs"
  echo
  for i in 1 2 3; do
    echo "--- run $i"
    (cd repro && cargo run --release --quiet > "$RUNLOG" 2>&1); rc=$?
    strip < "$RUNLOG"
    echo "process exit code: $rc"
    echo
  done

  echo "### BEFORE the patch, debug build (debug assertions + overflow checks on), 3 consecutive runs"
  echo
  for i in 1 2 3; do
    echo "--- run $i"
    (cd repro && cargo run --quiet > "$RUNLOG" 2>&1); rc=$?
    strip < "$RUNLOG"
    echo "process exit code: $rc"
    echo
  done

  echo "### BEFORE the patch - uncaught panic with a real backtrace (repro/src/bin/uncaught.rs),"
  echo "### release build, RUST_BACKTRACE=1"
  echo
  (cd repro && RUST_BACKTRACE=1 cargo run --release --quiet --bin uncaught 2>&1 | strip)
  echo

  echo "### BEFORE the patch - boundary-value sweep (repro/src/bin/bounds.rs), release build"
  echo
  (cd repro && cargo run --release --quiet --bin bounds 2>&1 | strip)
  echo
} >> "$RAW" 2>&1

just apply >/dev/null 2>&1 || { echo "apply failed" >&2; exit 1; }

{
  echo "### AFTER applying patch.diff to the same clone, release build, 3 consecutive runs"
  echo
  for i in 1 2 3; do
    echo "--- run $i"
    (cd repro && cargo run --release --quiet > "$RUNLOG" 2>&1); rc=$?
    strip < "$RUNLOG"
    echo "process exit code: $rc"
    echo
  done

  echo "### AFTER applying patch.diff, debug build, 1 run"
  echo
  (cd repro && cargo run --quiet > "$RUNLOG" 2>&1); rc=$?
  strip < "$RUNLOG"
  echo "process exit code: $rc"
  echo

  echo "### AFTER applying patch.diff - boundary-value sweep, release build"
  echo
  (cd repro && cargo run --release --quiet --bin bounds 2>&1 | strip)
} >> "$RAW" 2>&1

python3 - "$RAW" <<'PY'
import sys
lines = open(sys.argv[1]).read().splitlines()
out = []
for line in lines:
    if line.strip() == '' and out and out[-1].strip() == '':
        continue
    out.append(line)
open('output.txt', 'w').write('\n'.join(out) + '\n')
PY
rm -f "$RAW" "$RUNLOG" repro/core repro/core.* core core.*
wc -l output.txt
