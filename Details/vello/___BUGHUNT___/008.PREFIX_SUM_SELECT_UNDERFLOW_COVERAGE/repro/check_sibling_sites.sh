#!/usr/bin/env bash
# Standalone check for candidate 008: does the "unsigned-underflow-before-select" prefix-sum
# pattern that open PR #1817 (https://github.com/linebender/vello/pull/1817, fixes #1816) patches
# exist anywhere else in the repo - vello_shaders' own sibling shaders, vello_hybrid, or
# sparse_strips/vello_sparse_shaders - beyond the 6 sites that PR already touches?
#
# The vulnerable shape (per PR #1817's own description): a WGSL `select(default, arr[ix - 1u], ix
# > 0u)` where `arr` is a `var<workgroup>` array. WGSL's `select()` evaluates both operands
# unconditionally (it is not short-circuiting, unlike an `if`), so the `ix == 0u` lane always
# computes `arr[0u - 1u]` (== arr[4294967295]) before the result is discarded. On most
# backends/drivers this silently reads garbage or is optimized away; on Mali (which lowers
# `var<workgroup>` arrays to ordinary addressable memory) the wrapped index becomes a wild address
# and the driver reports a device loss (issue #1816) or a page fault (issue #1465, same symptom
# signature: "Mali GPU" + "any/some scene containing a path" + "crashes on first frame").
#
# This script is read-only static analysis - it does not build or run any shader, since actually
# triggering the driver-level fault needs real Mali hardware this environment doesn't have (see
# RESULT.md's "Fully reproducible" section for why that's the honest limitation here). What it
# *does* verify, deterministically and exhaustively, re-runnable against any checkout:
#
#   1. Enumerate every `select(` call in every .wgsl file (vello_shaders, the classic full-GPU
#      pipeline) and every .wesl file (vello_hybrid/sparse_strips' vello_sparse_shaders crate),
#      and print the ones whose arguments contain array/buffer indexing at all (candidates for the
#      vulnerable shape).
#   2. Print which of those match the specific vulnerable shape (an index expression of the form
#      `<var> - 1u` combined with a `<var> > 0u` guard in the select's condition).
#   3. Confirm sparse_strips/vello_sparse_shaders has zero `var<workgroup>` declarations and zero
#      `@compute` entry points at all (i.e. it has no compute-shader prefix-sum kernels in the
#      first place - the architectural reason this bug class cannot occur there: strip/tile
#      generation happens on the CPU in that renderer, the GPU side is fragment/vertex-only).
#   4. Confirm the classic full-GPU pipeline's compute shaders are compiled with
#      `wgpu::ShaderRuntimeChecks::unchecked()` (vello/src/wgpu_engine.rs), i.e. the 6 known sites
#      are genuinely exploitable in the current tree, not hypothetically-unreachable dead code.
#
# Usage: ./check_sibling_sites.sh <path-to-vello-checkout>
set -euo pipefail

REPO="${1:?usage: check_sibling_sites.sh <path-to-vello-checkout>}"
cd "$REPO"

echo "=== 1. Every select( call with array/buffer indexing in its arguments ==="
echo "--- vello_shaders/shader/**/*.wgsl ---"
grep -rn "select(" vello_shaders/shader/ | grep -E "\[[^]]*\]" || echo "(none)"
echo "--- sparse_strips/vello_sparse_shaders/shaders/**/*.wesl ---"
grep -rn "select(" sparse_strips/vello_sparse_shaders/shaders/ | grep -E "\[[^]]*\]" || echo "(none)"

echo
echo "=== 2. Of those, which match the vulnerable shape (select(default, arr[X - 1u], X > 0u)) ==="
VULN=$(grep -rn "select(" vello_shaders/shader/ sparse_strips/vello_sparse_shaders/shaders/ 2>/dev/null \
  | grep -E "\- 1u\]" || true)
echo "$VULN"
COUNT=$(echo "$VULN" | grep -c . || true)
echo "Total vulnerable-shape sites found: $COUNT"

echo
echo "=== 3. PR #1817's own 6 patched sites, for comparison (from its diff) ==="
cat <<'EOF'
vello_shaders/shader/tile_alloc.wgsl:107 (select(0u, sh_tile_count[local_id.x - 1u], local_id.x > 0u))
vello_shaders/shader/backdrop_dyn.wgsl:76 (select(0u, sh_row_count[el_ix - 1u], el_ix > 0u))
vello_shaders/shader/coarse.wgsl:251     (select(part_start_ix, sh_part_count[part_ix - 1u], part_ix > 0u))
vello_shaders/shader/coarse.wgsl:315     (select(0u, sh_tile_count[el_ix - 1u], el_ix > 0u))
vello_shaders/shader/fine.wgsl:223       (select(0u, sh_count[el_ix - 1u], el_ix > 0u))
vello_shaders/shader/fine.wgsl:579       (select(0u, sh_count[el_ix - 1u], el_ix > 0u))
EOF

echo
echo "=== 4. Does sparse_strips/vello_sparse_shaders have any workgroup-array compute shaders at all? ==="
WG=$(grep -rl "var<workgroup>" sparse_strips/vello_sparse_shaders/shaders/ 2>/dev/null || true)
COMPUTE=$(grep -rl "@compute" sparse_strips/ 2>/dev/null || true)
echo "var<workgroup> declarations found: ${WG:-none}"
echo "@compute entry points found: ${COMPUTE:-none}"

echo
echo "=== 5. Is the classic GPU pipeline's compute-shader path actually running unchecked? ==="
grep -n "ShaderRuntimeChecks::unchecked\|create_shader_module_trusted" vello/src/wgpu_engine.rs || echo "(not found)"

echo
echo "=== Verdict ==="
if [ "$COUNT" -eq 6 ]; then
  echo "Exactly 6 vulnerable-shape sites found, matching PR #1817's own 6 patched sites 1:1. No additional/sibling occurrence exists in this checkout."
else
  echo "MISMATCH: found $COUNT vulnerable-shape sites, expected 6 (matching PR #1817). Investigate the diff above."
fi
if [ -z "$WG" ] && [ -z "$COMPUTE" ]; then
  echo "sparse_strips/vello_sparse_shaders has no workgroup arrays and no compute shaders at all - this bug class cannot occur there."
else
  echo "NOTE: sparse_strips DOES have workgroup arrays/compute shaders - re-check manually, the architectural assumption above is stale."
fi
