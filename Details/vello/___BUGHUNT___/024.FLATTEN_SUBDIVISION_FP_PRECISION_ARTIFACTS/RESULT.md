# FLATTEN_SUBDIVISION_FP_PRECISION_ARTIFACTS

Two linked issue reports (linebender/vello#1139, #775) both describe visible flattening artifacts
attributed to floating-point precision in the "classic" GPU-pipeline curve flattener
(`vello_shaders`/`vello_encoding`, the compute-shader based renderer under `vello/`, as opposed to
the newer `sparse_strips`/`vello_cpu` renderer). This candidate investigates #1139 (stroke
flattening artifact on a real-world DWG-derived line, under a transform with large absolute
translation) and checks whether it shares a root cause with #775 (a fill/winding artifact near a
near-degenerate cusp).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `flatten_euler` (`vello_shaders/src/cpu/flatten.rs`) and the linked issue's own diagnosis + maintainer/reporter comments | Code reading, ~30 min | Confirmed the design: for strokes (`offset != 0`), curve evaluation happens in *local* (pre-transform) coordinates, using `f32` (`vello_shaders::cpu::util::Vec2`); for fills (`offset == 0`), evaluation happens in device space. Reporter's own workaround (apply the affine to the point in `f64` before stroking, pass `Affine::IDENTITY`) independently confirms the local-space `f32` path is the culprit for #1139. |
| Real end-to-end reproduction: `vello::Scene::stroke` (public API) -> `vello_encoding::Resolver::resolve` (real resolve step) -> the real production `vello_shaders::cpu::{pathtag_reduce, pathtag_scan, bbox_clear, flatten}` functions, run in the same order `vello/src/render.rs` dispatches them (no GPU available in this sandbox - no `mesa-vulkan-drivers`/lavapipe installed and no permission to install packages, so the wgpu-backed `Renderer` itself could not be exercised; these CPU functions are not a reimplementation, they are the literal functions `vello::Renderer` itself calls as its CPU-shader fallback and are logically identical to `flatten.wgsl`) | Release build, exact input from #1139 (`Line::new((41550.906, 4632.2827), (41550.926, 4632.273))`, `Stroke::new(2.0)`, the reported `Affine`) | **Reproduced**: emitted `LineSoup` line segments deviate from the true (independently, `f64`-computed) straight offset edge by up to 2.51 device pixels, an order of magnitude past the algorithm's own 0.25px tolerance budget, in an alternating high/low pattern across the 10 emitted segments matching the reported "jagged" look. See `repro/`, `output.txt`. |
| Read #775's own issue thread for a shared-root-cause check | Reading only (no separate repro built this session - see "What's not done" below) | **Not the same root cause.** Project maintainer raphlinus's own comment on #775 (2025-01-08) identifies it as a *tile-allocation coordinate-snapping* robustness issue ("the current approach is to try to minimize the number of tiles when a line falls exactly on a tile boundary... apparently not [correct]"), i.e. a bug in the coarse/tile-allocation stage's handling of points that land exactly on a tile grid line - unrelated to `flatten_euler`'s local-space `f32` stroke-offset computation that #1139 hits. The two issues are both filed under "numerical robustness in path rendering" but are distinct bugs in different pipeline stages. |

## Status

**Confirmed** (for #1139's flattening artifact). Reproduces 3/3 runs, bit-for-bit identical
output each time (deterministic - no randomness anywhere in this computation), release build, no
GPU involved. #775 was read but not independently re-reproduced or root-caused this session - see
"What's not done".

**Fully reproducible**: **Yes**. This is a single-process, single-participant bug (a scene handed
to `vello::Scene::stroke`'s real public API *is* the genuine external input - there is no second
participant whose reaction is being faked here; see `CONFIRMATION.md` section 11's own carve-out
for exactly this shape of finding). `repro/` drives the real `vello::Scene::stroke` public entry
point, the real `vello_encoding::Resolver::resolve`, and the real, unmodified
`vello_shaders::cpu::{pathtag_reduce,pathtag_scan,bbox_clear,flatten}` production functions in the
exact order `vello/src/render.rs` dispatches them - not a hand-built stand-in for any of those
stages. The one substitution is dispatch mechanism (direct Rust function call instead of a wgpu
compute-shader dispatch), forced by this sandbox having no usable GPU/Vulkan ICD and no permission
to install one (`mesa-vulkan-drivers`/lavapipe absent, apt/dpkg locked to root); the dispatched
function bodies themselves are the real production code, not reimplemented, and
`vello_shaders::cpu::flatten` is literally what `vello::Renderer` itself calls whenever a GPU
lacks a feature the primary shader needs (see `vello/src/shaders.rs`'s `CpuShaderType::Present`).

## Class

logic (rendering-precision artifact, not a crash/UB)

## Root cause

`vello_shaders/src/cpu/flatten.rs`'s `flatten_euler` (mirrored exactly by
`vello_shaders/shader/flatten.wgsl`, the real GPU compute shader) flattens stroke offset curves
(`offset != 0`) in *local*, pre-transform coordinates, entirely in `f32`
(`vello_shaders::cpu::util::Vec2`). Path coordinates are already narrowed from `kurbo`'s `f64` to
`f32` even earlier, at encode time (`vello_encoding::path::PathEncoder::move_to`/`line_to` take
`x: f32, y: f32`). For CAD/DWG-style input, the *local* coordinate system can have very large
absolute magnitude (here, ~41550) while the actual geometric feature size is tiny (the reported
line is only ~0.02 local units long). `f32` at that magnitude has an absolute ULP of roughly
0.005, so subtracting two nearby large `f32` coordinates to get a ~0.02-unit tangent/derivative
(as `eval_cubic_and_deriv`, `cubic_start_tangent`/`cubic_end_tangent`, and the Euler-spiral
curvature (`k0`/`k1`) computation in `flatten_euler` all do) suffers catastrophic cancellation:
the *relative* error in the recovered derivative is tens of percent. That corrupted
curvature/derivative feeds directly into the subdivision-count (`n`) and per-segment position
(`s`/`inv`) computation later in the same function, so the final offset-curve points land
noticeably off the true parallel-offset line - by several device pixels in the reproduced case,
alternating between segments (because the corruption's sign/magnitude depends on which specific
`f32` values happen to be involved at each subdivision step), producing the reported jagged look.
Flattening for *fills* (`offset == 0`) does not have this specific failure mode as badly, because
it evaluates in device space (`local_to_device.apply(...)` applied up front): device-space
coordinates are typically much smaller in magnitude than arbitrary CAD/world local coordinates,
so the same absolute `f32` ULP produces much less relative error - matching why the reporter's own
workaround (apply the affine transform to the point in `f64`, then pass `Affine::IDENTITY` to
`stroke`) resolves the symptom for their case: it forces evaluation to happen in the
smaller-magnitude device-space range instead of raw large-magnitude local space.

#775 is not the same bug: per the maintainer's own comment on that issue, its cause is a
tile-boundary-snapping robustness issue in the coarse/tile-allocation stage, not this
local-coordinate `f32` stroke-flattening cancellation.

## Introduced by

fe3273ab5418e33a5471227ce845a39d60d6768b "Checkpoint of Euler spiral based stroke expansion (#496)"
(2024-03-13) - the commit that introduced the Euler-spiral-based GPU stroke-expansion design,
including the "Flatten in local coordinates if this is a stroke. Flatten in device space
otherwise." choice quoted verbatim in the still-open issue #1139. Confirmed via `git log -S`
across full history (`git log --all -S"Flatten in local coordinates"`); the code has not
materially changed on this point since (the file was later moved as part of a workspace
restructuring in d2bd9514 "Migrate the repository to a virtual workspace. (#590)", not modified).

## The fix

**No bounded fix was attempted/applied this session.** The precision loss is architectural, not a
localized bug in `flatten_euler`'s own arithmetic:

- The damage begins even earlier than `flatten_euler` - at path-encoding time
  (`vello_encoding::path::PathEncoder::move_to`/`line_to`), where every path coordinate is already
  narrowed from `kurbo`'s `f64` to `f32` and stored in that absolute-magnitude, un-recentered form.
  By the time `flatten_euler` runs, the original `f64` precision needed to recover the true
  ~0.02-unit tangent is already gone; no amount of algorithmic improvement purely inside
  `flatten.rs`/`flatten.wgsl` can undo that.
- A real fix needs one of: (a) widening the stored path-coordinate format to `f64` throughout the
  encoding + GPU buffer layout + WGSL shader interfaces (a cross-cutting wire-format and
  performance-affecting change spanning both the CPU reference implementation and every WGSL
  shader that reads path data), or (b) recentering each path's local coordinate origin into its
  per-path transform at encode time (subtract the shape's own bounding-box origin before encoding,
  fold that translation into the transform record used only for that path's geometry) - which
  needs care to avoid disturbing the separately-recorded brush transform, glyph-run encoding, and
  any code that assumes a path's encoded coordinates are exactly what the caller supplied.
  Implementing and correctness-verifying either approach (including checking it doesn't regress
  the project's existing stroke/fill snapshot tests) is beyond what could be done safely within
  one candidate session - this matches the maintainers' own year-plus-unresolved discussion on
  #1139 itself, where no fix has landed and the "technical barriers" question is explicitly still
  open.
- `patch.diff` is intentionally omitted for this reason.

- Project test suite: not run - no change was made to the source tree this session (main project
  working tree was never modified; `git status` in the main project's own checkout shows no
  diff from this candidate).

## What's not done / open questions

- #775 was read (via its GitHub comments) but not independently re-reproduced or root-caused this
  session; per the maintainer's own comment it is a distinct tile-allocation/coarse-stage
  robustness bug, not the same class as this candidate's finding. It would be a reasonable
  separate candidate if a future session wants to build a real repro for it specifically.
  Suggested for `IDEAS.md`.
- The *fill* path (`offset == 0` branch of `flatten_euler`) was not stress-tested for the same
  large-local-coordinate-magnitude failure mode; it's structurally less exposed (it evaluates in
  device space, not local space) but a fill under a transform that maps device space back to a
  huge range (an extreme zoom-out, for instance) could plausibly hit a related failure - not
  checked this session.
- The `vello_common`/`sparse_strips` flatten path (`sparse_strips/vello_common/src/flatten.rs`,
  `flatten_simd.rs` - the newer CPU-first renderer, already covered by unrelated candidates 004 and
  012 in this workspace) was not checked for the same large-local-coordinate-magnitude failure
  mode; it has a different internal representation and was out of scope for this candidate, which
  targeted the two issues' actual reported pipeline (classic `vello`/`vello_shaders`).
- A longer fuzzing campaign (via `fuzz-bug-repro`, run separately) specifically generating
  large-magnitude local coordinates paired with small relative feature sizes, across both stroke
  and fill paths and both the classic and sparse-strips renderers, would likely turn up further
  instances of this same precision-budget violation class.
