# FILTER_DATA_SNAPPED_PADDING_U16_SATURATION

`FilterData::new`'s `snapped_padding` helper (`sparse_strips/vello_common/src/filter/mod.rs:207-227`)
computes each side of a filter's device-space padding as `snap_up(expansion, tile_size) as u16`,
where `expansion` is `3 * std_deviation * transform_scale` (an unbounded, caller-controlled `f32`
`std_deviation` - reachable via the public `FilterFunction::Blur`/`DropShadow` - times a
transform's extracted scale). The `f64 as u16` cast saturates rather than erroring for values past
`u16::MAX`. The candidate's original suspicion was that this silent clamp merely produces a
"quality regression" (under-blurred edges); digging further found it also causes a real,
deterministic panic through the real public rendering pipeline.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: called the real public `vello_common::filter::FilterData::new` directly (the exact function `vello_cpu::render.rs`'s `push_layer` and `vello_hybrid`'s equivalent call) via a standalone probe crate, across std_deviation/transform-scale pairs straddling the `u16::MAX` expansion boundary | release build, single run, values `{(100, 10), (100, 218), (100, 220), (1e6, 1e6)}` | Confirmed the cast saturates exactly as suspected: `true_padding=66000 -> observed=65535` and `true_padding=3e12 -> observed=65535`, with no panic/diagnostic at this level alone - matching the candidate's own "quality regression" framing, nothing more, at this level of investigation. |
| Full real pipeline, actual crate: drove `vello_cpu::RenderContext::push_layer(..., filter: Some(Filter::from_function(FilterFunction::Blur { radius })))` + `fill_path` + `pop_layer` + `render_with` - the exact public sequence `sparse_strips/vello_cpu/examples/basic.rs` uses for the rest of its API - on an ordinary small canvas (200x200) with `std_deviation=30_000.0` and an identity transform (device-space expansion = 90,000 > `u16::MAX`) | release build, 3 separate runs | **Panicked deterministically 3/3 runs**: `thread 'main' panicked at .../vello_common/src/util.rs:380:51: called \`Option::unwrap()\` on a \`None\` value`, inside `vello_common::util::strip_bbox`, reached via `RenderContext::fill_path` -> `SingleThreadedDispatcher::record_fill`. A control run with the same canvas and a moderate `std_deviation=20.0` (no saturation) rendered without panicking, isolating the panic to the saturation case specifically, not something incidental to the pipeline itself. |
| Varied input within the same class (CONFIRMATION.md section 7): instead of an extreme `std_deviation` with an identity transform, used a moderate `std_deviation=50.0` combined with a large transform scale (`Affine::scale(500.0)`) - device-space expansion = 3*50*500 = 75,000, reached via the multiplication in `transform_blur_params`, not via `std_deviation` alone | release build, 1 run against pristine code | Panicked identically at the same location, confirming the bug is about the *resulting* saturated padding value, not specific to how the expansion arithmetic reaches it. |

Also checked, as a sanity control, that a canvas the user *deliberately* creates at exactly
`u16::MAX` width panics too (`RectU16::snap_to_tile_coordinates`, `util.rs:188`) - but that one is
already documented in-source ("This method will panic if we have a viewport of size u16::MAX and
draw at the very edge, but better than returning a wrong result") as an accepted, deliberate
trade-off for a user's own explicit choice of canvas size. The `strip_bbox` panic this candidate
reproduces is different in kind: the user's canvas is an ordinary 200x200/300x300, and the
`u16::MAX`-sized surface is constructed *internally and invisibly* by the filter machinery as a
side effect of the saturating cast - nothing the caller did looks unusual.

## Status

**Confirmed.** Reproduces 3/3 runs (release build) for the original trigger (extreme
`std_deviation`, identity transform) and 1/1 for a differently-shaped input in the same class
(moderate `std_deviation`, large transform scale) - fully deterministic, no layout/seed dependency
(a plain arithmetic-boundary bug, not memory-layout-dependent UB).

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls only the real, public,
production entry point a real application uses - `vello_cpu::RenderContext::push_layer` (with a
`Filter` built from the public `FilterFunction::Blur`), `fill_path`, `pop_layer`, `render_with` -
the same sequence `vello_cpu`'s own `examples/basic.rs` uses for the rest of its API. The
caller-supplied `std_deviation`/transform are a single external artifact (an attacker/caller
controlled filter parameter) with no second participant to fake, the same class CONFIRMATION.md
section 11 says a single-process repro against the real public entry point is sufficient for.

## Class

crash

## Root cause

Three pieces combine:

1. `FilterData::new`'s `snapped_padding` (`filter/mod.rs:207-227`) computes each padding side as
   `snap_up(expansion, tile_size) as u16` where `expansion = 3 * std_deviation * transform_scale`
   (via `transform_blur_params`, `gaussian_blur.rs:26-33`). This `f64 -> u16` cast **saturates**
   (Rust's `as` semantics) rather than erroring once `expansion` exceeds `u16::MAX` (65,535) -
   reachable with an ordinary-looking `std_deviation` (tens of thousands) or an ordinary
   `std_deviation` combined with a large-but-plausible transform scale (hundreds), both fields
   fully public and documented only as "must be non-negative", with no upper bound.
2. `ViewportState::push_root_viewport` (`sparse_strips/vello_common/src/viewport.rs:88-107`)
   builds the filter layer's own internal render surface as
   `current_width.saturating_add(padding.left).saturating_add(padding.right)` (same for height).
   Once `padding.left`/`padding.right` are each saturated to 65,535, this sum itself saturates to
   exactly `u16::MAX`, with no clamp to keep the result tile-aligned.
3. A viewport whose width/height is exactly `u16::MAX` (65,535) is not a multiple of
   `Tile::WIDTH`/`Tile::HEIGHT` (4) - its rightmost/bottommost tile is partial, and that tile's
   far edge, expressed in pixels, is `16384 * 4 = 65,536`, which overflows `u16`. This exact
   invariant is already known and guarded elsewhere in the codebase (`Tile::new_clamped` clamps
   tile coordinates to `u16::MAX / Tile::WIDTH` "to make sure x and y stay in range when
   multiplying with the tile width and height", and `RectU16::snap_to_tile_coordinates` has an
   explicit "will panic ... but better than returning a wrong result" comment for exactly this
   case) - but `vello_common::util::strip_bbox`'s own pixel-unit reconversion
   (`tile_bbox.x1.checked_mul(Tile::WIDTH).unwrap()`, `util.rs:380`) has no equivalent guard or
   documentation, and `push_root_viewport` never prevents the input dimension from reaching this
   dangerous exact value in the first place. Drawing ordinary content (a small rect near the
   filter layer's own origin, itself shifted into this oversized surface by
   `FilterData::source_shift()`) reaches into that unrepresentable partial tile and panics.

The saturating cast in `filter/mod.rs` (the candidate's original suspicion) is the trigger that
makes this reachable from ordinary filter usage; the missing tile-alignment clamp in
`viewport.rs`'s `push_root_viewport` is what turns that saturation into an actual panic.

## Introduced by

87072800 "sparse_strips: Add more new/refactored code into `vello_common` (#1745)" - `git log -S
"fn push_root_viewport"` on `viewport.rs`/`clip.rs` (this logic lived in `clip.rs` before a later
refactor moved it into `viewport.rs`) points to this commit as the one that introduced the
unclamped `saturating_add` chain that builds a filter layer's internal render surface. This is the
piece that actually turns a saturated padding value into the specific `u16::MAX`-sized surface that
panics; the earlier, contributing precondition (the saturating cast itself in `filter/mod.rs`,
computing padding with no upper bound) predates it, across the filter-effects code's own history
(`0961df0b`, `93162ef3`, `af32d531`) - the crash requires both together, but the missing clamp is
the more specific "introducing" change since equivalent unbounded padding without this clamp is
what specifically permits the exact-u16::MAX viewport size.

## The fix

Added a tile-aligned upper clamp (`MAX_SAFE_DIMENSION = u16::MAX - (u16::MAX % Tile::WIDTH)` =
65,532) to both the width and height computed in `ViewportState::push_root_viewport`
(`sparse_strips/vello_common/src/viewport.rs`), so a filter's internal padded viewport can never
reach the dangerous exact-`u16::MAX` size regardless of how large the (attacker-controlled) filter
padding saturates to. This keeps the fix at the point where the dangerous value is actually
constructed, consistent with the same tile-alignment invariant `Tile::new_clamped` and
`RectU16::snap_to_tile_coordinates` already assume elsewhere in this codebase, rather than papering
over the specific `strip_bbox` call site alone (which would leave the same unrepresentable-size
viewport reachable by any other code path that later touches tile-unit bbox math).

Verified against both inputs found in this session (per CONFIRMATION.md section 7 - not just the
one that first triggered it): both the "extreme std_deviation, identity transform" case and the
"moderate std_deviation, large transform scale" case render without panicking after the patch (see
`output.txt` for the full before/after transcript, captured from `repro/` run against a fresh
clone at `base_commit`, before and after `patch.diff`).

- Project test suite (`cargo test --lib --release`, run once against the pristine tree, once with
  `patch.diff` applied, `git checkout --` between the two in this worktree's own copy of the main
  project):
  - `vello_common`: 159 passed / 0 failed / 1 ignored, both before and after.
  - `vello_cpu`: 81 passed / 0 failed, both before and after.
  - `vello_hybrid`: 122 passed / 0 failed, both before and after.
  No regressions, no new failures.

## What's not done / open questions

- Only `ViewportState::push_root_viewport` was patched. Whether any other code path can construct
  a `StripGenerator`/root viewport whose width or height lands in the dangerous
  `(65,532, 65,535]` range through some other route (not via a filter's padding at all - e.g. a
  user directly requesting `RenderContext::new(65535, ...)` or `65534`/`65533`) was not swept; the
  sibling `RectU16::snap_to_tile_coordinates` panic at `util.rs:188` for a *user-chosen*
  `u16::MAX`-sized canvas is explicitly documented as an accepted trade-off in-source and was left
  alone (out of scope for this candidate, which is specifically about the filter-padding
  saturation making this reachable *invisibly*, not about a user's own explicit canvas-size
  choice).
- Did not check whether `vello_hybrid`'s own filter/viewport code (which also calls `FilterData::new`
  and has its own filter-layer sizing logic, per `sparse_strips/vello_hybrid/src/schedule/allocate.rs`)
  has an analogous unclamped-sum path that could reach the same or a similar panic through a
  different call site; only the shared `vello_common::viewport::ViewportState::push_root_viewport`
  (used by `vello_cpu`) was fixed and verified.
- Whether `FilterData::new` itself should instead reject/clamp an out-of-range `expansion` value
  with a documented, explicit bound (rather than silently saturating per-side padding at all) is a
  broader API-design question left to the maintainers; the fix applied here prevents the specific
  panic without changing `FilterData::new`'s existing saturating-cast behavior, which may still
  produce a visually under-blurred result for such extreme inputs (the "quality regression" the
  original candidate suspected) - that quality-only aspect (no panic, just less blur spread than
  a mathematically unbounded implementation would produce) was not treated as a separate defect
  here, since every pixel-coordinate type in this codebase (`RectU16`, viewport width/height) is
  already `u16`-bounded by design.
