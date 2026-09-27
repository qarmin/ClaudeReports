# COARSE_RASTER_EPSILON_FUDGE_DEGENERATE_SPAN

`vello_shaders/shader/fine.wgsl`'s `fill_path` (the classic GPU pipeline's analytic-area
antialiasing) computes each pixel's coverage from a segment's per-scanline x-span `[xmin, xmax]` as
`a = (b + 0.5*(d*d - c*c) - xmin) / (xmax - xmin)`, first subtracting a fixed `1.0e-6` from `xmin`
to avoid a `0/0` division when the span is exactly zero (a perfectly vertical edge). The candidate
description (referencing linebender/vello#417, "two rectangle edges very close together") suspected
this fixed epsilon combined with the division is numerically fragile for a span that is nonzero but
very small - i.e. an edge that is *almost* but not exactly vertical, rather than only the exactly-
vertical case the epsilon is designed for.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual derivation: proved the formula (without the epsilon) is the exact closed-form average of the antialiasing "coverage" function `g(x) = 1 - clip(x,0,1)` over `[xmin, xmax]`, then derived the *design* error the epsilon introduces analytically | By hand + a Python script sweeping `xlo` in [-0.3, 1.0] and `delta = xmax-xmin` in [1e-9, 1.0], comparing the perturbed formula (f64) against the true integral (f64) | Max absolute error **~6e-7** across the whole domain - the epsilon-hack design itself is fine in f64/exact math; not the bug |
| Same sweep but evaluating the *actual* f32 shader arithmetic (the exact `d*d - c*c` catastrophic-cancellation-prone computation, in `f32`) against the f64 ground truth | Rust harness (`rustc -O`), same domain sweep | Max absolute error **~0.058** (14.8/255, ~5.8% of the coverage range) for `delta` roughly in `[1e-9, 1e-6]` - f32 catastrophic cancellation in `d*d - c*c`, amplified by the small denominator, is the real numerical-fragility mechanism (not the epsilon concept per se) |
| Faithfully replicated the upstream `startx + t*delta.x` computation (not just the final formula) to find realistic geometry that survives f32 rounding at each stage and lands a per-scanline `xmax-xmin` in the bad `[1e-9, 1e-6]` band | Rust harness, iterating coordinate magnitude / segment height / total shear until a concrete, ordinary-looking scene (small path coordinates, a shear of a few millionths of a device pixel over ~200 pixels of height) produced the target span | Found a realistic geometry: a rectangle whose left edge is sheared by `4e-6` device pixels over `200` device pixels of height (slope ~2e-8, i.e. visually indistinguishable from a perfectly vertical edge) reliably lands the per-scanline span in the bad band, giving a persistent **alternating banding pattern** (e.g. 175/180/182/255 cycling) along a column whose true coverage is provably constant (178/255) to well within 8-bit rounding for the entire height |
| End-to-end repro: rendered that exact scene through `vello`'s real public API (`Scene::fill` + `Renderer::render_to_texture`), driven by a real `wgpu` device (Mesa llvmpipe software rasterizer via the GL backend, `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1` - this sandbox has no real GPU) | release build, `repro/src/main.rs` | **Reproduced**: 131/190 sampled rows deviate from the true constant coverage by more than 2/255 levels (max deviation 4/255), with 186 row-to-row jumps of >=2 levels, along an edge whose real slope is far too small to legitimately change the rendered value even once |
| Repeatability (CONFIRMATION.md section 1) | Same binary, 5 runs total (1 initial + 4 repeats), release build, llvmpipe/GL | 5/5 reproduces, byte-for-byte identical banding pattern every run - fully deterministic, no layout/seed dependency |

## Status

**Confirmed**. Reproduces deterministically, 5/5 runs (1 initial + 4 repeats), release build,
against a real `wgpu` device (Mesa llvmpipe software rasterizer via the GL backend - this sandbox
has no real GPU hardware; any working `wgpu` backend, including a real Vulkan/Metal/DX12 GPU,
executes the exact same WGSL `fill_path` shader code, since the analytic-area formula and its
epsilon fudge are unconditional, not backend-specific).

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls the classic `vello` crate's real,
public `Scene::fill` and `Renderer::render_to_texture` entry points with an ordinary (if extreme)
path and `Affine::IDENTITY` - the same code path a real application embedding vello would use. This
is a single-participant bug (a scene handed to the renderer's genuine public API, no network peer
or second binary involved), so CONFIRMATION.md section 11's bar is satisfied trivially the same way
a malformed file handed to a parser's real public entry point would be.

## Class

logic

## Root cause

`fill_path`'s per-scanline, per-pixel-column coverage formula computes, for a segment's local
x-span `[xmin, xmax]` relative to a given pixel column, `a = (b + 0.5*(d*d - c*c) - xmin) / (xmax -
xmin)` where `b/c/d` are `xmax`/`xmax` clamped to `[0,1]`/`xmin` clamped to `>=0`. This is
mathematically the exact average of the antialiasing coverage function over `[xmin, xmax]` -
verified by hand and confirmed numerically to be accurate to ~6e-7 in f64/exact arithmetic,
*including* the fixed `1.0e-6` epsilon subtracted from `xmin` (added only to keep the division
defined when `xmax == xmin` exactly, i.e. a perfectly vertical edge).

The problem is f32 precision, not the epsilon design: whenever a segment's per-scanline x-span
(`xmax - xmin`, before the epsilon perturbation) is small but *not* exactly zero - i.e. an edge that
is very slightly, but not perfectly, non-vertical, entirely plausible for any nearly-vertical edge
after ordinary path transforms - the resulting denominator can be dominated by whichever is smaller,
the true span or the fixed 1e-6 fudge, and the numerator's `d*d - c*c` term subtracts two f32
values that are extremely close together (catastrophic cancellation), losing most or all of the
precision the true (small) span needed to compute a correct ratio. The result: for a per-scanline
span roughly in `[1e-9, 1e-6]` device pixels, the computed coverage can be off by several percent of
the full `[0,1]` range (up to ~5.8% in the worst case found), and - because exactly how much
precision is lost varies with the exact bit pattern of `xmin`/`xmax` from one scanline to the next
even though the true edge position moves by an imperceptible amount row-to-row - the visible symptom
is a persistent alternating banding pattern along what should be a perfectly smooth, constant-
coverage edge, rather than a one-off glitch.

This is a plausible concrete mechanism behind the "gap when two rectangle edges are close together"
symptom reported in linebender/vello#417 (a maintainer there called the general phenomenon
"conflation artifacts" and noted it also depends on scene zoom level, consistent with a near-but-
not-exactly-vertical edge's per-scanline span shrinking into this bad numerical range at high zoom)
- though that issue's own repro composites *two separate* paths at a shared boundary, an additional,
distinct mechanism (independent per-object antialiasing not summing to exactly 1 across the seam)
that this candidate did not attempt to reproduce or rule out; this candidate demonstrates the
epsilon/cancellation mechanism in isolation, within a *single* path's own edge.

## Introduced by

not determined - the exact formula, including the `1.0e-6` epsilon on `xmin`, is present verbatim
in this file's very first commit in this repository's history, b6c4963d "Initial commit of
piet-wgsl" (2022-10-24), under the original filename `piet-wgsl/shader/fine.twgsl` (confirmed via
`git show b6c4963d:piet-wgsl/shader/fine.twgsl`). It was therefore most likely ported from the
pre-existing `piet-gpu` project, whose history is not part of this repository, rather than
introduced by any commit trackable here.

## The fix

Replaced the always-perturbed, always-divided formula with a small branch: when `|xmax - xmin|`
(the *true*, unperturbed span) is below a `1.0e-4` threshold, use the mathematically exact limit of
the average directly - `1 - clamp(0.5*(xmin+xmax), 0, 1)`, the coverage function's own value at the
span's midpoint, valid because the average of a function over a vanishingly small interval is just
its value at that point - avoiding both the division-by-near-zero and the `d*d - c*c` cancellation
entirely. Above the threshold, the original ratio formula is used unchanged, but with the *true*
`xmin` (the artificial `-1.0e-6` fudge is no longer needed at all, since the new branch handles the
exactly-zero case that fudge existed for). Applied identically to both `vello_shaders/shader/fine.wgsl`
(the real WGSL shader) and `vello_shaders/src/cpu/fine.rs` (the CPU fallback implementation of the
same formula, used when a GPU lacks a needed feature - same bug, same fix, kept in sync since the
two are meant to compute identical results).

Verified numerically (f32 vs. f64 ground truth, same domain sweep as "What was tried" above): max
absolute error dropped from ~0.058 (original) to ~0.00055 (fixed) - a ~100x reduction, now well
below 8-bit output quantization (1/255 ~= 0.0039) everywhere, including right at the branch
threshold (checked for a visible seam at the `1e-4` cutover - none found, error stays under 0.00014
on both sides of it).

Re-ran `repro/` against the patched tree with the *original* triggering input: no longer
reproduces - the sampled column reads a constant 178/255 (0/190 rows off by more than 2 levels, 0
row-to-row jumps of >=2 levels), matching the analytically correct constant value to within normal
8-bit rounding.

Varied the input within the same bug class (CONFIRMATION.md section 7): re-ran with a different
fractional edge position and shear amount (`X0 = 0.65`, `DX_TOTAL = 1.3e-6`, vs. the original `X0 =
0.3`, `DX_TOTAL = 4e-6`) against a `git stash`-restored pristine tree first (confirmed it also
reproduces there - 23/190 rows off by more than 2 levels, max deviation 10 levels, 46 jumps), then
against the patched tree (confirmed no longer reproduces - 0 rows off by more than 2 levels).

- Project test suite (`cargo test -p vello_tests --release`, `WGPU_BACKEND=gl
  LIBGL_ALWAYS_SOFTWARE=1` - this is the workspace's real end-to-end rendering/snapshot suite for
  the classic GPU pipeline, and the one that actually exercises `fine.wgsl`/`cpu/fine.rs`): **64
  passed / 0 failed / 1 ignored, identical before and after** the patch (this checkout's
  `vello_tests/snapshots/*.png` LFS pointers were already smudged to real PNGs, so no LFS-related
  failures were encountered, unlike some other candidates' sessions in this workspace). Also ran
  `cargo test -p vello_shaders --release` and `cargo test -p vello --release`: 0 tests exist in
  either package, unaffected before/after.

## What's not done / open questions

- This candidate demonstrates the epsilon/cancellation mechanism within a *single* path's own
  near-vertical edge. linebender/vello#417's own reported repro is about two *separate* paths
  (different draw calls) sharing a boundary; whether/how much this specific mechanism contributes to
  that particular multi-path "conflation artifact" (as opposed to the general, harder,
  independent-per-object-antialiasing-doesn't-sum-to-1 problem a maintainer described there) was not
  investigated - a maintainer response on that issue treats the multi-path case as a distinct, open,
  no-current-fix problem, and this candidate does not claim to resolve it.
- The same formula does not exist in `sparse_strips` (the newer `vello_cpu`/`vello_hybrid`
  backends use an entirely different rasterization algorithm - confirmed via `grep` finding no
  match for the `d*d - c*c`/`xmax - xmin` pattern anywhere under `sparse_strips/`), so this bug and
  fix are specific to the classic `vello`/`vello_shaders` GPU pipeline.
- The `1.0e-4` branch threshold was chosen empirically (comfortably above where f32 cancellation
  becomes significant, per the domain sweep) rather than derived from a tight error bound; a
  maintainer may want to re-derive or justify it more rigorously, or benchmark the added branch's
  performance impact in the hot per-pixel-per-segment shader loop (not measured this session).
- This session's GPU testing used Mesa llvmpipe (software rasterizer) via wgpu's GL backend, since
  this sandbox has no real GPU; the bug is in shared WGSL compute-shader source compiled by
  `vello_shaders` for every wgpu backend, so it is expected to reproduce identically on a real
  GPU/Vulkan/Metal/DX12 backend, but this wasn't independently confirmed on real GPU hardware this
  session.
