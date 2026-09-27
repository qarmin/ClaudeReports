#!/usr/bin/env bash
# Differential test of a SINGLE-FILE Rust program: compile + run it twice, once with plain rustc
# (LLVM = the oracle) and once with cg_clif, then diff the two outputs.
#
# This is the fastest loop for finding/minimizing a cg_clif miscompilation: no cargo, no crate
# skeleton, ~1s per side.
#
#   bash ___BUGHUNT___/tools/diffcheck.sh repro.rs                  # against the main tree's dist/
#   CLIF_ROOT=tmp/wt/003.foo bash .../diffcheck.sh repro.rs         # against a worktree's dist/
#   RUSTFLAGS_EXTRA='-Ctarget-cpu=native' bash .../diffcheck.sh r.rs
#   OPT=1 bash .../diffcheck.sh repro.rs                            # -O on both sides
#
# Exit codes: 0 = outputs identical, 1 = outputs differ (or cg_clif failed to compile/ICEd), 2 = usage.
#
# NOTE: opt-level 0 by default and that matters. At -O, rustc's MIR GVN const-folds SIMD intrinsic
# calls through the const-eval interpreter, so a miscompiled backend still prints the right answer
# and the bug is invisible. Only pass OPT=1 when deliberately testing the optimized path.
set -uo pipefail

SRC="${1:?usage: diffcheck.sh <file.rs>}"
[ -f "$SRC" ] || { echo "no such file: $SRC" >&2; exit 2; }

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CLIF_ROOT="$(cd "${CLIF_ROOT:-$ROOT}" && pwd)"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
[ -x "$RUSTC_CLIF" ] || { echo "missing $RUSTC_CLIF - run ./y.sh build in $CLIF_ROOT first" >&2; exit 2; }

OUT="$(mktemp -d "${TMPDIR:-/tmp}/diffcheck.XXXXXX")"
trap 'rm -rf "$OUT"' EXIT
BASE="$(basename "$SRC" .rs)"
OPTFLAG=""
[ "${OPT:-0}" = "1" ] && OPTFLAG="-O"
EXTRA="${RUSTFLAGS_EXTRA:-}"
ulimit -c 0

run_side() {
    local label="$1" compiler="$2" bin="$OUT/${BASE}_$1"
    local clog="$OUT/$1.compile" rlog="$OUT/$1.run"
    # shellcheck disable=SC2086
    if ! "$compiler" $OPTFLAG $EXTRA -o "$bin" "$SRC" > "$clog" 2>&1; then
        echo "=== $label: COMPILE FAILED ==="
        cat "$clog"
        return 101
    fi
    if [ -s "$clog" ]; then
        echo "=== $label: compile diagnostics ==="
        cat "$clog"
    fi
    "$bin" > "$rlog" 2>&1
    local rc=$?
    echo "=== $label: exit $rc ==="
    cat "$rlog"
    return "$rc"
}

llvm_out="$(run_side LLVM "${RUSTC:-rustc}" 2>&1)"; llvm_rc=$?
clif_out="$(run_side CLIF "$RUSTC_CLIF" 2>&1)"; clif_rc=$?

printf '%s\n\n%s\n' "$llvm_out" "$clif_out"

# Compare only the run output (drop the compile-diagnostics noise, which legitimately differs:
# cg_clif warns about unsupported intrinsics).
strip_diag() { printf '%s\n' "$1" | sed -n '/^=== [A-Z]*: exit /,$p'  | sed '1s/^=== [A-Z]*: /=== /'; }
if [ "$(strip_diag "$llvm_out")" = "$(strip_diag "$clif_out")" ]; then
    echo
    echo "RESULT: identical (exit $llvm_rc)"
    exit 0
fi

echo
echo "RESULT: DIFFERENT (llvm exit $llvm_rc, clif exit $clif_rc)"
echo "--- diff (llvm vs clif, run output only) ---"
diff <(strip_diag "$llvm_out") <(strip_diag "$clif_out") || true
exit 1
