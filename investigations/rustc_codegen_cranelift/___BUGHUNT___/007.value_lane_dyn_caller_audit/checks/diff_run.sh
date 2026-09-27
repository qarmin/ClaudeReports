#!/usr/bin/env bash
# Self-contained LLVM-vs-cg_clif differential runner for this candidate's check programs.
#
#   bash checks/diff_run.sh <path-to-cg_clif-checkout> <file.rs> [more.rs ...]
#
# Compiles each single-file program twice - once with plain `rustc` (LLVM = the oracle) and once
# with <checkout>/dist/rustc-clif - runs both at opt-level 0 and compares the run-time output.
# Compile diagnostics are excluded from the comparison, since cg_clif legitimately warns about
# unsupported intrinsics where LLVM does not.
#
# Exit 0 = every program's two sides agreed. Exit 1 = at least one disagreed.
#
# opt-level 0 is deliberate: at -O rustc's MIR GVN const-folds SIMD intrinsic calls through the
# const-eval interpreter, so a miscompiling backend still prints the right answer.
set -uo pipefail

CLIF_ROOT="${1:?usage: diff_run.sh <cg_clif-checkout> <file.rs>...}"
shift
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
[ -x "$RUSTC_CLIF" ] || { echo "missing $RUSTC_CLIF - run ./y.sh build in $CLIF_ROOT first" >&2; exit 2; }

OUT="$(mktemp -d)"
trap 'rm -rf "$OUT"' EXIT
ulimit -c 0
rc_total=0

for SRC in "$@"; do
    BASE="$(basename "$SRC" .rs)"
    echo "########## $BASE ##########"
    for side in LLVM CLIF; do
        if [ "$side" = LLVM ]; then CC="${RUSTC:-rustc}"; else CC="$RUSTC_CLIF"; fi
        if ! "$CC" -o "$OUT/${BASE}_$side" "$SRC" > "$OUT/$side.compile" 2>&1; then
            echo "=== $side: COMPILE FAILED ==="
            cat "$OUT/$side.compile"
            printf 'COMPILE_FAILED\n' > "$OUT/$side.run"
            continue
        fi
        [ -s "$OUT/$side.compile" ] && { echo "=== $side: compile diagnostics ==="; cat "$OUT/$side.compile"; }
        "$OUT/${BASE}_$side" > "$OUT/$side.run" 2>&1
        echo "exit $?" > "$OUT/$side.hdr"
    done
    if diff -q "$OUT/LLVM.run" "$OUT/CLIF.run" > /dev/null 2>&1 \
       && diff -q "$OUT/LLVM.hdr" "$OUT/CLIF.hdr" > /dev/null 2>&1; then
        echo "RESULT $BASE: identical ($(cat "$OUT/LLVM.hdr"))"
        head -c 300 "$OUT/LLVM.run"; echo
    else
        echo "RESULT $BASE: DIFFERENT"
        echo "--- llvm ($(cat "$OUT/LLVM.hdr" 2>/dev/null)) ---"; head -c 300 "$OUT/LLVM.run" 2>/dev/null; echo
        echo "--- clif ($(cat "$OUT/CLIF.hdr" 2>/dev/null)) ---"; head -c 300 "$OUT/CLIF.run" 2>/dev/null; echo
        rc_total=1
    fi
    rm -f "$OUT"/*.hdr "$OUT"/*.run "$OUT"/*.compile
    echo
done

exit "$rc_total"
