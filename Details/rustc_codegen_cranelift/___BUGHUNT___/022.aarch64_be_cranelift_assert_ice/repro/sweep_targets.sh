#!/usr/bin/env bash
# Blast-radius sweep: for every target in `rustc --print target-list`, compile the tiny
# `sweep_nocore.rs` crate (no_core, so no per-target sysroot is needed) with plain rustc first as
# a go/no-go filter, then with cg_clif, and classify cg_clif's behaviour as
# CLIF_OK / ERROR (a clean diagnostic) / ICE.
#
#   CLIF_ROOT=<a built rustc_codegen_cranelift checkout> bash sweep_targets.sh > sweep.txt
#
# target_sweep_before.txt / target_sweep_after.txt in this folder are the outputs captured at
# 2bc05d45 without and with this candidate's patch.diff. Compare with:
#   diff <(cut -d'|' -f1,2 target_sweep_before.txt) <(cut -d'|' -f1,2 target_sweep_after.txt)
set -uo pipefail
ulimit -c 0

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIF_ROOT="${CLIF_ROOT:-$HERE/../clone}"
RUSTC_CLIF="$CLIF_ROOT/dist/rustc-clif"
[ -x "$RUSTC_CLIF" ] || { echo "MISSING: $RUSTC_CLIF (run ./y.sh build in $CLIF_ROOT)" >&2; exit 2; }

O=$(mktemp -d)
trap 'rm -rf "$O"' EXIT

for t in $(rustc --print target-list); do
    # rustc itself must accept the target, otherwise the comparison is meaningless.
    rustc --edition 2021 --crate-type lib --target "$t" --emit=obj -o "$O/l.o" \
        "$HERE/sweep_nocore.rs" >/dev/null 2>&1 || { echo "$t|LLVM_FAIL|"; continue; }

    out=$("$RUSTC_CLIF" --edition 2021 --crate-type lib --target "$t" --emit=obj -o "$O/c.o" \
        "$HERE/sweep_nocore.rs" 2>&1)
    if [ $? -eq 0 ]; then
        echo "$t|CLIF_OK|"
    elif grep -q "the compiler unexpectedly panicked" <<<"$out"; then
        echo "$t|ICE|$(grep -m1 -A1 'panicked at' <<<"$out" | tr '\n' ' ' | head -c 160)"
    else
        echo "$t|ERROR|$(grep -m1 '^error' <<<"$out" | head -c 150)"
    fi
done
