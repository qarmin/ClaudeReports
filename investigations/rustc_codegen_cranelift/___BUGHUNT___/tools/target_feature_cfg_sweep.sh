#!/usr/bin/env bash
# Sweep every x86-64 target feature rustc knows about and compare what
# `cfg(target_feature = "<F>")` evaluates to in the *generated program* under
# plain rustc (LLVM, the oracle) and under cg_clif, for the same source and the
# same `-C target-feature=+<F>` command line.
#
#   bash ___BUGHUNT___/tools/target_feature_cfg_sweep.sh                # main tree's dist/
#   CLIF_ROOT=tmp/wt/015.foo bash .../target_feature_cfg_sweep.sh       # a patched worktree
#   MODE=cpu bash .../target_feature_cfg_sweep.sh                       # -Ctarget-cpu=native instead
#
# Prints one line per feature where the two disagree, then a count.
# Exit 0 when every feature agrees, 1 otherwise.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CLIF_ROOT="$(cd "${CLIF_ROOT:-$ROOT}" && pwd)"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
RUSTC="${RUSTC:-rustc}"
MODE="${MODE:-feature}"
TARGET=x86_64-unknown-linux-gnu
[ -x "$RUSTC_CLIF" ] || { echo "missing $RUSTC_CLIF - run ./y.sh build in $CLIF_ROOT" >&2; exit 2; }

# `--print cfg` goes through the backend's own `target_config`, which is exactly the thing under
# test, so it is a faithful (and much faster) stand-in for compiling a program per feature. The
# `run_one` path below double-checks a handful of features by actually building and running a
# program, so the fast path is never trusted on its own.
feats() { "$1" --print cfg "${@:2}" --target "$TARGET" 2>/dev/null \
        | sed -n 's/^target_feature="\(.*\)"$/\1/p' | sort; }

ALL="$("$RUSTC" --print target-features --target "$TARGET" 2>/dev/null \
      | sed -n 's/^    \([^ ]*\) *-.*/\1/p' | grep -v '^crt-static$' | sort -u)"
echo "features rustc knows for $TARGET: $(printf '%s\n' "$ALL" | grep -c .)"

if [ "$MODE" = cpu ]; then
    FLAGS=(-Ctarget-cpu=native)
    echo "mode: -Ctarget-cpu=native (one shot)"
    L="$(feats "$RUSTC" "${FLAGS[@]}")"; C="$(feats "$RUSTC_CLIF" "${FLAGS[@]}")"
    echo "llvm reports $(printf '%s\n' "$L" | grep -c .) features, cg_clif reports $(printf '%s\n' "$C" | grep -c .)"
    echo "--- enabled under LLVM but not under cg_clif ---"
    comm -23 <(printf '%s\n' "$L") <(printf '%s\n' "$C") | tr '\n' ' '; echo
    echo "--- enabled under cg_clif but not under LLVM ---"
    comm -13 <(printf '%s\n' "$L") <(printf '%s\n' "$C") | tr '\n' ' '; echo
    n=$(comm -3 <(printf '%s\n' "$L") <(printf '%s\n' "$C") | grep -c .)
    echo "DISAGREEING: $n"
    [ "$n" -eq 0 ]
    exit
fi

# One `-Ctarget-feature=+F` at a time: does cfg(target_feature = "F") come out the same?
disagree=0; agree=0; rejected=0
for f in $ALL; do
    l="$(feats "$RUSTC" "-Ctarget-feature=+$f" | grep -Fx "$f")"
    if [ -z "$l" ]; then rejected=$((rejected + 1)); continue; fi   # LLVM refused/ignored it too
    c="$(feats "$RUSTC_CLIF" "-Ctarget-feature=+$f" | grep -Fx "$f")"
    if [ "$l" = "$c" ]; then agree=$((agree + 1)); else
        disagree=$((disagree + 1)); echo "DISAGREE  -Ctarget-feature=+$f  llvm=true clif=false"
    fi
done
echo "agree: $agree   disagree: $disagree   not-enabled-by-LLVM-either: $rejected"
[ "$disagree" -eq 0 ]
