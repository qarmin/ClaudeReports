# Reserve each tessellated mesh at last frame's size (per-viewport hints kept in `Context`)

`Context::tessellate` creates one `Mesh` per clip-rect/texture run and grows its `vertices`/`indices` by doubling reallocs (66 reallocs >= 2 KB per demo frame, ~2% of widget-scene time in `memmove`/`finish_grow` inside `fill_closed_path`/`stroke_and_fill_path`). Candidate 010 (fixed capacity) and 011 (hints from this frame's text shapes) were tried before; this one reserves each mesh at the exact size the same primitive index had in the previous tessellation of the same viewport. New public API (additive): `Tessellator::with_mesh_size_hints`, `Tessellator::take_mesh_size_hints`, `tessellator::mesh_sizes`; `Context` stores the hints per `ViewportId`. A wrong hint (UI changed) only costs the old behavior.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metrics: time (interleaved A/B, pinned core) and allocations (`harness/alloc_count`, system malloc).

## What was tried

Time, ratios patched/base:

| Benchmark | ratios |
|---|---|
| demo_only_tessellate (4 rounds) | 0.9359, 0.9247, 0.9305, 0.9332 |
| demo_with_tessellate__realistic | 0.9791, 0.9743, 0.9805, 0.9874 |
| scene_buttons_300 (scenes harness, 3 rounds) | 0.9186, 0.9103, 0.8915 |
| scene_widget_mix_100 | 0.9626, 0.9523, 0.9617 |
| scene_painter_shapes_2000 | 0.8355, 0.8468, 0.8513 |

Allocation (`alloc_count`, per iteration):

| Scene | allocs base -> patched | bytes base -> patched | peak live delta base -> patched |
|---|---|---|---|
| demo_only_tessellate | 157 -> 99 | 954,716 -> 755,648 (-20.8%) | 519,864 -> 576,804 (+11%) |
| demo_run_ui+tessellate | 1258 -> 1200 | 1,306,196 -> 1,107,128 (-15.2%) | 532,424 -> 589,364 (+10.7%) |

## Result

Time: tessellation -7% (demo), whole demo frame -2.3%, widget-heavy scenes -4% to -9%, painter-shape-heavy scene -15%. Allocation: -58 allocations and -199 KB allocated per frame (-21% bytes). Peak live memory within a tessellation is up by about 57 KB (+11%) in the demo, because meshes are allocated at their final size from the start instead of growing in steps; steady-state retained memory is not larger than before (capacity equals length, where doubling left up to 2x slack).

## Does it change behavior/output?

- **Bit-exact output?** Yes, only Vec capacities change.
- Breaking change: no. New public methods/function are additive; `ContextImpl` is private.
- Tests: `cargo test -p epaint -p egui --all-features`: egui 93 + 3 + 188, epaint 95 + 4 (1 ignored) passed, 0 failed.
- Real change to the tested project's source: `patch.diff` (+55/-8).

## Why the result looks the way it does

UIs change slowly between frames, so the previous frame's per-primitive sizes are an almost exact prediction; avoiding the doubling chain removes the realloc copies and allocator calls (candidate 011's this-frame text-only estimate was too coarse; 010's fixed size wasted memory).

## What's not done / open questions

- Hints are indexed by primitive position; an insertion early in the primitive list shifts the later hints by one for a frame (harmless, perf-only).
- The same idea could apply to `Context`'s other per-frame buffers (shape lists in `PaintList`, `Vec<ClippedShape>`).
