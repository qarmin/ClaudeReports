# Sized initial capacity for tessellated meshes (hints from text/mesh shapes of the same clip-rect run)

Follow-up to candidate 010: instead of a fixed capacity, `tessellate_shapes` computes for each run of consecutive shapes with the same clip rect the number of vertices/indices that text shapes (`galley.num_vertices/num_indices`) and mesh shapes will add, and the new mesh for that run is created with that capacity.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metrics: time and allocation (counting allocator).

## What was tried

| Variant | demo_only_tessellate allocs/iter | bytes/iter | peak live delta | time patched/base (4 interleaved rounds) |
|---|---|---|---|---|
| baseline | 157.0 | 954,716 | 519,864 | 1.0 |
| sized hints | 121.0 | 999,776 | 627,016 | demo_only_tessellate 1.0025, 0.9925, 1.0035, 1.0039; demo_with_tessellate__realistic 0.9822, 0.9904, 1.0106, 1.0034 |

(`demo_run_ui+tessellate`: 1258 allocs / 1,306,196 bytes / 532,424 peak -> 1222 / 1,351,256 / 639,576.)

## Result

No time change (within noise). Allocations -36 per tessellation (-23%), but allocated bytes +4.7% and peak live +107 KB (+21%): text sizes over-estimate (rows culled by the clip rect are still counted, and non-text shapes in the same run are not counted so the vec still doubles after the hint).

## Does it change behavior/output?

- **Bit-exact?** Yes (capacity only). Breaking change: no (new private fields/functions only).
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed, `cargo test -p egui` 86 + 3 + 188 passed after; identical before.
- Real change to the tested project's source: `patch.diff` (+49/-2).

## Why the result looks the way it does

With the system allocator, realloc of a growing buffer is often an in-place extension or a cheap mremap/copy, so removing the early doublings saves allocation count but not measurable time; candidate 010's -4% was likely cache/page effects of the large fixed capacity rather than saved copies.

## What's not done / open questions

- Clamping the hint by culling (count only rows inside the clip rect), or reusing the previous frame's final sizes per primitive index, would remove the over-reservation; not tried.
