# PTCL: early-exit on fully-opaque overdraw (issue #409)

Classic `vello`'s GPU coarse-rasterization stage (`coarse.wgsl`, echoed in Rust by
`vello_shaders/src/cpu/coarse.rs`) builds a per-tile command list (PTCL) by appending one
command per draw object that touches the tile, in painter's-algorithm order, with no
occlusion check. When many draws stack in the same small screen area (issue #409's example:
a zoomed-out scene), a tile's PTCL can carry hundreds of commands even though only the last
few opaque ones are ever visible - every earlier command still gets executed, uselessly, by
the per-pixel fine-rasterization stage. This candidate adds an occlusion check: when a later
`DRAWTAG_FILL_COLOR` draw both (a) fully covers the tile (no line segments cross it, i.e. the
existing `n_segs == 0` "solid" fast path) and (b) is fully opaque (alpha byte == 0xff), every
command queued so far for that tile is provably invisible and gets discarded by rewinding the
tile's own PTCL write cursor back to just after its header slot. The check is disabled for the
rest of a tile as soon as any clip is opened on it (`can_occlude = false`), since a clip changes
what "behind" means and its own blend-depth bookkeeping isn't undone by a rewind - a
deliberately conservative scope, not a fundamental limit of the technique.

Toolchain: `cargo 1.98.0 (797e8a9bc 2026-08-05)`, `rustc 1.98.0 (88d9e12ae 2026-08-18)`.

## What was tried

No existing project benchmark reached classic `vello`'s coarse/PTCL/fine GPU pipeline with a
controllable overdraw shape, so a standalone bench crate (`bench/`) was written: it builds a
1024x1024 scene of 400 fully-opaque, tile-aligned, full-canvas rectangles stacked on top of each
other (alternating red/blue) - the literal "many draws stack in the same small screen area, only
the last few are ever visible" shape from issue #409 - and renders it through the real
`vello::Renderer` (`render_to_texture`, real WGSL compute dispatch, not the CPU-only echo path),
timing wall-clock render+readback over 30 iterations after a 5-iteration warmup. This sandbox has
no real GPU; `wgpu` falls back to a software EGL/GLES (llvmpipe) adapter (Vulkan `request_adapter`
fails with `ERROR_INCOMPATIBLE_DRIVER`, confirmed via `RUST_LOG=debug`), so the absolute
millisecond values below are software-rasterizer numbers, not representative of real-hardware
GPU timings - but the *relative* delta directly reflects how much PTCL command volume the fine
stage has to walk per pixel, which is exactly the mechanism issue #409 describes.

24 logical cores were available; "single-core" below pins the whole process (host thread and any
worker threads llvmpipe spawns) with `taskset -c 0`, "multi-core" is unpinned/default.

| Variant | multi-core (unpinned) min / p50 / mean | single-core (`taskset -c 0`) min / p50 / mean | notes |
|---|---|---|---|
| baseline (3 runs) | 49.36 / 51.41 / 51.47 ms; 48.65 / 51.60 / 51.47 ms; 48.63 / 51.07 / 50.92 ms | 500.85 / 514.63 / 533.97 ms; 493.23 / 517.78 / 543.33 ms; 489.53 / 503.85 / 510.13 ms | pixel hash `5b6009d6283d0383` every run |
| patched (3 runs) | 11.58 / 14.62 / 15.45 ms; 12.12 / 14.35 / 14.20 ms; 12.81 / 25.87 / 24.45 ms (one noisy run, shared machine) | 46.43 / 49.65 / 49.93 ms; 48.06 / 50.41 / 50.58 ms; 47.74 / 51.20 / 51.61 ms | pixel hash `5b6009d6283d0383` every run |
| `just test` fresh-clone re-run (1 run each, for reproducibility) | baseline 41.99 / 51.20 / 50.65 ms; patched 11.19 / 13.29 / 13.41 ms | baseline 495.04 / 499.42 / 502.98 ms; patched 52.99 / 62.92 / 64.27 ms | same hash both times |

## Result

Time, multi-core: min render+readback time drops from ~48.6-49.4 ms (baseline, 3 runs) to
~11.6-12.8 ms (patched, 3 runs) - about a 3.8-4.2x reduction, with zero overlap between the two
groups' min values across every run taken (7 baseline-side and 7 patched-side samples across the
paired-run table and the independent `just test` re-run).

Time, single-core (`taskset -c 0`): min drops from ~489-501 ms (baseline) to ~46-53 ms
(patched) - about a 9.3-10.8x reduction. The larger single-core ratio (vs. ~4x multi-core) is
consistent with the baseline's much larger PTCL command volume benefiting more from llvmpipe's
internal worker-thread parallelism than the patched, already-small command volume does - not a
sign the two measurements disagree.

This is a wall-clock-only result; no memory/allocation metric was instrumented for this
candidate (the mechanism shortens the PTCL buffer's *used* region per tile, which would also
show up as a lower `BumpAllocators::ptcl` value, but that wasn't read back for this measurement -
noted as an open question below).

## Does it change behavior/output?

- **Bit-exact?** Yes. Every one of the 12+ bench runs above (baseline and patched, single- and
  multi-core, plus the independent `just test` fresh-clone run) produced the identical
  non-cryptographic pixel hash (`5b6009d6283d0383`) of the rendered RGBA output.
- Project test suite, run through both the CPU-echo path (`vello_shaders::cpu::coarse`) and the
  real GPU WGSL path (`coarse.wgsl`) since both were patched to keep them equivalent:
  - `cargo test -p vello -p vello_shaders -p vello_encoding -p vello_tests`: **95 passed, 0
    failed**, identical before and after the patch (includes `compare_gpu_cpu` - which renders
    the same scenes through both the CPU-echo and GPU pipelines and diffs them pixel-for-pixel -
    plus all snapshot/regression/property tests).
  - `cargo test -p vello_sparse_tests` (the unrelated `vello_cpu`/sparse-strips renderer, not
    touched by this patch): **89 passed, 3538 failed** identically before and after the patch.
    This is a pre-existing environment issue in this sandbox, not a regression from this
    candidate - every failure is `Unsupported(UnsupportedError { format: Unknown, ... })` /
    "Invalid PNG signature" while decoding reference snapshots, i.e. the missing `git-lfs`
    binary content (this repo's PNG snapshots are LFS pointer files here, confirmed via
    `git-lfs: not found` warnings), identical failure signature and identical counts on both
    baseline and patched trees.
- **Is this a breaking change?** No. It changes neither a public API nor the rendered output for
  any of the scenes exercised above; it only discards PTCL commands that were already guaranteed
  to be fully invisible.

## Why the result looks the way it does

`fine.wgsl`'s per-pixel loop walks every PTCL command for a tile unconditionally until `CMD_END`
- it has no way to know in advance that an early `CMD_SOLID`+`CMD_COLOR` pair was going to be
completely painted over 399 more times. Discarding the dead commands at PTCL-build time (coarse
stage, which already has full painter's-order visibility per tile) turns an O(layers) per-pixel
compositing cost into O(1) for the fully-covered-by-a-later-opaque-fill case, which is exactly
why the win scales with the number of stacked layers in the bench scene (400 layers here).

Note this exact problem is already solved for the *other*, newer `vello_cpu`/`sparse_strips`
rendering architecture via its own per-tile `DepthBuffer` occlusion mechanism
(`sparse_strips/vello_cpu/src/coarse/depth.rs`, added in #1701) - issue #409 predates that
rewrite and is specifically about the original GPU/PTCL pipeline in the `vello` crate, which had
no equivalent mechanism until this patch.

## What's not done / open questions

- The occlusion check only fires for `DRAWTAG_FILL_COLOR` (a plain solid fill with a
  statically-known packed color). It deliberately does not attempt to cover
  `DRAWTAG_BLURRED_ROUNDED_RECT`, gradients, or images, since those can be non-opaque in ways
  not visible from the draw tag alone (gradient stops, image alpha, rounded-rect anti-aliasing at
  the corners) - extending it there is future work, not attempted here.
- The check is disabled for the rest of a tile as soon as any clip is opened on it
  (conservative: a real win is likely still available for a fully-opaque fill that closes out a
  clip and covers the whole tile, but that wasn't attempted here to keep the change bounded and
  easy to reason about).
- `BumpAllocators::ptcl` (the PTCL bump-allocator high-water mark) was not read back and compared
  before/after; it should also drop for the same reason `fine`'s per-pixel work drops, and would
  be a natural memory-side companion metric to this time-side result.
- Benchmarked only on a software (llvmpipe) GPU fallback, since this sandbox has no real GPU
  adapter (confirmed via `RUST_LOG=debug`: Vulkan `request_adapter` fails with
  `ERROR_INCOMPATIBLE_DRIVER`, falls back to EGL/GLES surfaceless). The relative win should carry
  over to real hardware (same PTCL command count reduction, same fine-shader work reduction) but
  the absolute millisecond numbers here are not representative of real-GPU timings.
- Only one scene shape was tested (400 full-canvas, fully-opaque, tile-aligned overlapping
  rects). A more "realistic" zoomed-out scene (issue #409's own motivating case) with partial,
  irregular overlap between shapes was not benchmarked separately; the per-tile mechanism should
  still help proportionally to how much of each tile's history is covered by a later opaque fill,
  but that wasn't independently measured.
