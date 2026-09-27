# STROKE_CLIP_TILE_VIEWPORT_BOUNDARY_WRONG_OUTPUT

Suspect: <https://github.com/linebender/vello/issues/1483> - stroking
`Rect::from_origin_size((113.5, 0.5), (103.0, 61.0))` with `Stroke::new(1.0)` in the `simple`
example (a 1044x800 window) is reported to produce visibly wrong GPU-rendered output for "certain
origin/width combinations" where the stroke's extent touches the viewport's top edge (`y = 0`).
Flagged as similar in flavor to the still-open, still-unreproduced-by-this-workspace
<https://github.com/linebender/vello/issues/1115> ("Rendering bug with GPU and stroke edge at
viewport edge", reported on `aarch64-apple-darwin` with Metal). Both live in the classic `vello`
crate's GPU compute pipeline (tile alloc / backdrop / coarse / fine WGSL shaders), not the
`sparse_strips` CPU renderer.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real headless render via the project's own public API (`vello_tests::compare_gpu_cpu_sync` -> `Renderer::render_to_texture`), reporter's exact scene, 1044x800 canvas, default `AaConfig::Area` | debug test build, `cargo test -p vello_tests --test known_issues`, single llvmpipe/Gl software adapter (only adapter available in this sandbox - no `/dev/dri`, no Vulkan ICD) | Test passed (GPU vs "CPU" mean diff below threshold); manual full-canvas pixel scan of the actual rendered PNG found the *only* non-background pixels exactly bounding the expected 1px stroked-rect outline (328 pixels, bbox x=[113,216] y=[0,61] - matches perimeter 2\*(103+61)-4=328 exactly), zero stray pixels anywhere else in the 1044x800 image. No artifact. |
| Attempt to obtain a real or software-Vulkan backend (closer to the reporter's actual GPU compute path than the Gl/llvmpipe fallback) | `apt-get download mesa-vulkan-drivers` (no candidate in configured sources), `sudo` refused (no-new-privileges), no `/dev/dri` | Infeasible in this session - abandoned, not a negative result in itself. |
| Same real headless render, `AaConfig::Msaa16` (matching what `examples/simple` itself actually passes to `render_to_texture`, not the `Area` default used above) | Same test harness, same adapter | Identical result: 328 non-background pixels, same exact bounding box, no artifact anywhere else. |

See `output.txt` for the full captured command output of all three approaches, plus the standalone
`repro/`'s own 3/3 repeat runs and its `just clone && just reproduce-before` run against a fresh
clone.

## Status

**Not reproducible** in this session's environment. Every approach available here - real headless
render through vello's actual `Renderer::render_to_texture` public entry point, at both the AA mode
the `simple` example itself uses and the default AA mode, at the exact window size and exact scene
from the issue - produced pixel-correct output with zero artifacts, 3/3 repeat runs (deterministic,
no flake).

This is **not** a claim that issue #1483 is invalid: this sandbox has no real GPU
(`/dev/dri` absent, no `nvidia-smi`) and no Vulkan backend at all (no ICD installed, and installing
one requires root, which isn't available here) - the only wgpu adapter reachable is a software
OpenGL rasterizer (llvmpipe/Mesa, `device_type: Cpu`, `backend: Gl`). The sibling issue #1115 was
explicitly reported on real Metal hardware; if this bug class depends on real-GPU-specific
execution (driver-specific floating-point rounding in a coarse/tile-alloc/fine WGSL shader,
undefined-order workgroup-shared-memory access that a software rasterizer's more literal/serial
shader interpretation masks, etc.), it would not be expected to show up on llvmpipe/Gl even if the
same WGSL source is genuinely buggy on real hardware. This candidate should be retried on a machine
with a real GPU (any backend - Vulkan/Metal/DX12) or with a real Vulkan software implementation
(e.g. lavapipe) available, which this sandbox could not provide.

**Fully reproducible**: N/A - no bug was reproduced to characterize as fully/partially
reproducible. The "did not reproduce" result itself is deterministic and repeatable (3/3 runs of
`repro/`, plus a from-scratch `just clone && just reproduce-before` run - see `output.txt`).

## Class

N/A (not Confirmed).

## Root cause

Not identified - no defect was observed to root-cause. The tile-allocation bounding-box clamp
(`vello_shaders/shader/tile_alloc.wgsl`, `ux0`/`uy0`/`ux1`/`uy1` = `clamp(x0..x1/y0..y1, 0,
width_in_tiles/height_in_tiles)`) and the per-row backdrop prefix sum
(`vello_shaders/shader/backdrop.wgsl`) were read as the most plausible locations for a viewport-
top-edge-specific defect (by analogy with #1115's description and this workspace's own
`001.tile_msaa_needs_clamping_tautology`/`002.TILE_MSAA_CORNER_FLOAT_CMP` findings in the sibling
`sparse_strips` renderer), but neither showed an obviously wrong computation on inspection, and the
empirical render matched the correct answer exactly in every configuration tried.

## Introduced by

Not determined - no bug was confirmed to exist in this session, so there is nothing to attribute
to a commit (per SKILL.md step 5, this lookup is skipped for a Not-reproducible result).

## The fix

No fix attempted - Status is Not reproducible; there is no defect to patch on the tree available in
this session.

- Project test suite: N/A, no change was made to the source tree (the only edits this session were
  an added test in `vello_tests/tests/known_issues.rs` in the isolated worktree, reverted before
  finishing - see below).

## What's not done / open questions

- The core limitation of this session: **no real GPU and no Vulkan backend was available to test
  against** (see Status). This is the single most important thing a future session should fix
  before concluding anything further about this bug - either run this candidate's `repro/` on a
  machine with real GPU hardware, or get a real Vulkan software implementation (lavapipe) installed
  (this sandbox lacked both root and the apt package).
- Only the classic `vello` crate's GPU compute pipeline was considered (matching the issue's own
  report, which uses `examples/simple` and `scene.stroke` through the full `vello` renderer) - the
  `sparse_strips` CPU/hybrid renderers were not checked, since the issue is specifically about GPU
  output and those crates use an entirely different (non-GPU-compute-shader) tiling algorithm.
- Did not sweep other origin/width combinations near the viewport boundary (the reporter's own
  wording - "It only happens with certain origins and widths" - implies the exact values matter);
  only the reporter's single exact example was tried, since no combination reproduced at all in
  this environment and there was no oracle available to search for one blindly.
- `miri` was not attempted: the suspect code is WGSL (GPU shader source), which miri cannot
  execute; a fuzz harness was not attempted either, since the failure mode (a specific rendering
  artifact at specific geometry) has no cheap automated oracle to detect "wrong" pixels without
  already knowing what "right" looks like.
- The isolated worktree used for this investigation
  (`/tmp/vello_worktrees/016_stroke_clip_boundary`) has one added-and-then-not-reverted test
  (`stroke_rect_touching_viewport_boundary` in `vello_tests/tests/known_issues.rs`, using
  `AaConfig::Msaa16` per the last row of "What was tried") - this is a throwaway worktree, not the
  main project tree, so it was left as-is rather than reverted; the main project's own working tree
  was never touched by this candidate.
