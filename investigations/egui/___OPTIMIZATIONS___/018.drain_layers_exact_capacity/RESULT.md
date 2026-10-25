# `GraphicLayers::drain`: allocate the output `Vec<ClippedShape>` at its final size

At the end of every pass `GraphicLayers::drain` collected all layers' shapes into an empty `Vec` with repeated `append`s, i.e. a doubling realloc chain of 96-byte `ClippedShape`s every frame. Now it first sums the lengths of all lists and allocates once.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metrics: allocation (`harness/alloc_count`, system malloc) and time (scenes harness).

## What was tried

| Scene | allocs/iter base -> patched | bytes/iter | peak live delta |
|---|---|---|---|
| demo_run_ui | 1113 -> 1111 | 379,224 -> 341,208 (-10.0%) | 43,428 -> 33,348 (-23%) |
| demo_run_ui+tessellate | 1258 -> 1256 | 1,306,196 -> 1,268,180 (-2.9%) | 532,424 -> 522,344 |

Time, interleaved A/B (patched/base, 4 rounds): scene_buttons_300 0.9938, 1.0022, 0.9986, 1.0082; scene_grid_labels_1000 0.9986, 1.0054, 0.9923, 0.9955; scene_painter_shapes_2000 0.9920, 1.0028, 1.0008, 1.0021.

## Result

Memory: -38 KB allocated (-10%) and -10 KB peak live (-23%) per UI pass of the demo; two fewer allocations (the realloc chain is a few big reallocs). Time: no measurable change (mimalloc makes the realloc chain cheap).

## Does it change behavior/output?

- **Bit-exact?** Yes (capacity only). Breaking change: no.
- Tests: `cargo test -p egui` 86 + 3 + 188 passed, same as unpatched.
- Real change to the tested project's source: `patch.diff` (+7/-1).

## Why the result looks the way it does

With a doubling Vec, the final allocation is between 1x and 2x the needed size and each growth copies; exact sizing removes the slack and copies, but the byte-level win is small compared with the rest of the pass.

## What's not done / open questions

- Per-layer lists (`PaintList`) still grow by doubling within a frame when first filled after being freed; they keep capacity across frames otherwise.
