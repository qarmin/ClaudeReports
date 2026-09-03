#!/usr/bin/env bash
# Sweep: does cg_clif's "<...> is not a constant" span_fatal (src/intrinsics/llvm_x86.rs and
# src/intrinsics/simd.rs) ever fire on real Rust?
#
# Compiles every generated call shape (gen_shapes.py) with dist/rustc-clif across a matrix of
# opt-levels / MIR-pass overrides and reports OK / FAIL / NOTCONST per cell. NOTCONST means the
# span_fatal fired.
#
#   bash run.sh                                  # main tree's dist/rustc-clif
#   CLIF_ROOT=tmp/wt/009.foo bash run.sh         # a worktree's dist/rustc-clif
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
RC="$(cd "${CLIF_ROOT:-$ROOT}" && pwd)/dist/rustc-clif"
[ -x "$RC" ] || { echo "missing $RC - run ./y.sh build first" >&2; exit 2; }

OUT="$(mktemp -d)"; trap 'rm -rf "$OUT"' EXIT
ulimit -c 0
python3 "$HERE/gen_shapes.py" >/dev/null
cp "$HERE/asm_poison.rs" "$HERE/shapes/s17_asm_in_same_body.rs"

FLAGSETS=(
  "-Copt-level=0"
  "-Copt-level=1"
  "-Copt-level=2"
  "-Copt-level=3"
  "-Copt-level=s"
  "-Copt-level=z"
  "-Copt-level=0 -Zinline-mir=yes"
  "-Copt-level=3 -Zmir-enable-passes=-GVN"
)

for f in "$HERE"/shapes/*.rs; do
  n="$(basename "$f" .rs)"
  for flags in "${FLAGSETS[@]}"; do
    # shellcheck disable=SC2086
    if $RC $flags -o "$OUT/$n" "$f" > "$OUT/log" 2>&1; then
      st=OK
    elif grep -q "is not a constant" "$OUT/log"; then
      st=NOTCONST
    else
      st="FAIL: $(grep -m1 '^error' "$OUT/log" | cut -c1-70)"
    fi
    printf '%-30s %-36s %s\n' "$n" "$flags" "$st"
  done
done
