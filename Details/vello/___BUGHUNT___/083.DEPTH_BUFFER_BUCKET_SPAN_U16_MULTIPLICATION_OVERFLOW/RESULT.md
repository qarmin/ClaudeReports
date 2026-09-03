# DEPTH_BUFFER_BUCKET_SPAN_U16_MULTIPLICATION_OVERFLOW

`sparse_strips/vello_cpu/src/coarse/depth.rs`'s `BucketRange::span` (~line 63) and the free
function `bucket_span` (~line 286) both multiply two `u16` values (a bucket count times
`DEPTH_BUCKET_WIDTH == 128`) with plain `u16` arithmetic. `DepthBuffer::new` sizes its bucket
array as `buffer_width.div_ceil(128)`, and for a (tile-aligned) render-target width in
`[65409, 65532]` this yields exactly 512 buckets - a full-buffer bucket range then computes
`512u16 * 128u16 == 65536`, one past `u16::MAX`. This looked suspicious because `buffer_width`
comes straight from `RenderContext::new(width, height)`'s public, fully caller-controlled `u16`
argument, with no upper-bound validation anywhere in between.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, in-tree: unit tests directly calling `bucket_span(0, 512)` and `DepthBuffer::new(65535)` + `for_each_unset_run_and_write` with a full-buffer `BucketRange` | debug build (`dev` profile, default `overflow-checks = true`), single run each | Both panic: `attempt to multiply with overflow` at `depth.rs:288` and `depth.rs:64` respectively - confirms the raw arithmetic overflows, but doesn't yet show it's reachable from a real scene. |
| Manual boundary-value construction against the real public API: `RenderContext::new(width, height)` at the same extreme width, exercised several scene shapes in-tree (temporary tests, reverted before finalizing) to find which one is actually reachable | debug build, single run each | A single full-canvas **opaque** fill does *not* trigger it (`split_opaque_span`'s own bucket-range math can never reach the last, 512th bucket, since the canvas literally cannot extend that far given `u16::MAX`). A **completely blank canvas** (no draws at all) *does* trigger it: `Fine::init_uncovered_range` unconditionally queries `DepthBuffer::for_each_unset_run` every strip row, and an all-zero depth buffer at this width returns the whole `0..512` range as one run. |
| Debug-assertions build + release build, both against the real public API (`RenderContext::new` -> optional `fill_rect` -> `flush` -> `render`), with a two-row canvas (row 0 opaque-filled, row 1 left blank) to also probe the release-mode (no overflow-checks) behavior | debug build (panic) vs. release build (no overflow-checks), 3 repeat runs each | Debug: panics 3/3 runs, deterministic. Release: doesn't panic, but 3/3 runs show row 1 (which should be fully transparent) with row 0's opaque red content bled through every one of its 65532 pixels - a real, silent logic-corruption manifestation of the same overflow, not just a masked panic. |

## Status

**Confirmed**. Reproduces 3/3 runs in a debug build (panic, `overflow-checks = true`, this
crate's default `dev` profile) and 3/3 runs in a release build (silent data corruption,
`overflow-checks = false`) - fully deterministic, no memory-layout/seed dependency (plain integer
arithmetic on a fixed input).

**Fully reproducible**: **Yes**. `repro/src/main.rs` drives only the real, public
`vello_cpu::RenderContext` entry points (`new`, `set_paint`, `fill_rect`, `flush`, `render`) - the
same sequence any real caller uses - with a single external artifact (an extreme but entirely
legitimate `u16` render-target width) as the sole caller-supplied input. There is no second
participant to fake here (see `CONFIRMATION.md` section 11): the caller directly picks the
render-target dimensions via the ordinary constructor.

## Class

crash

(Debug build: panics via a checked-arithmetic overflow trap. Release build: no crash, but a real
logic bug - see "Root cause" below - so this candidate covers both a `crash` and a `logic`
manifestation of the same underlying overflow; `crash` is used as the single Class label because
it's the more severe of the two observed manifestations, not because the release-mode corruption
is any less real.)

## Root cause

Three pieces combine:

1. `DepthBuffer::new(buffer_width)` (`depth.rs:171-175`) sizes its internal per-bucket-draw-id
   array as `buffer_width.div_ceil(DEPTH_BUCKET_WIDTH)` (`DEPTH_BUCKET_WIDTH == 128`). This is
   called with `bucketer.width()` (`dispatch/single_threaded.rs:179`,
   `dispatch/multi_threaded.rs:438`), which is the render-target's viewport width after snapping
   down to a multiple of `Tile::WIDTH == 4` - i.e. directly derived from the width the caller
   passed to `RenderContext::new`/`reset_and_resize`, with no upper-bound check anywhere in
   between. For a tile-aligned width in `[65409, 65532]` (the achievable top of the range, since
   `u16::MAX == 65535`), this yields exactly `512` buckets.
2. Every strip row's rasterization (`fine::rasterize_region`, `fine/mod.rs:466-501`)
   unconditionally calls `Fine::init_uncovered_range` after the front-to-back opaque-fill pass,
   which asks the (freshly-cleared-per-row) `DepthBuffer` for every still-unset ("not yet
   opaque-covered") bucket run via `DepthBuffer::for_each_unset_run` (`depth.rs:179-184`), over
   the row's *entire* pixel width. For any row with no opaque depth-tracked draw covering the
   full width (the common case - even a completely blank row, since the very last bucket,
   index 511, can *never* be written by any opaque draw: `split_opaque_span`'s own tile-alignment
   arithmetic can never advance a `BucketRange`'s `end` past 511 given a maximum achievable
   `u16`-width canvas), the *entire* buffer - all 512 buckets - comes back as a single contiguous
   unset run.
3. That run is converted from bucket indices back to a pixel `Span` via `bucket_span(0, 512)`
   (`depth.rs:286-289`, also duplicated via `BucketRange::span` at `depth.rs:62-65`), which
   computes `(512 - 0) as u16 * DEPTH_BUCKET_WIDTH` == `512u16 * 128u16` == `65536` - one past
   `u16::MAX`.
   - **Debug build** (`overflow-checks = true`, this workspace's default `dev` profile):
     panics with "attempt to multiply with overflow".
   - **Release build** (`overflow-checks = false`): silently wraps to `0`, producing a
     degenerate `Span { x: 0, width: 0 }`. `Span::intersect` then returns `None` for it
     (`0 < 0` is `false`), so `for_each_unset_run`'s callback (which normally clears/unpacks that
     row's pixels to the background) is **never invoked at all** for that row. Because `Fine`'s
     base blend buffer (`blend_buffers[0]`) is allocated once per `render()` call and reused
     across every strip row (not reallocated per row - see `Fine::new`, `fine/mod.rs:534-546`), a
     row that follows an opaque-filled row silently inherits ("bleeds") that earlier row's
     leftover pixel data instead of being cleared, as demonstrated by `repro/`.

## Introduced by

9c292860957b010ed8d81e6d4c7317d81ab0ec4d "vello_cpu: A full rewrite of the frontend (#1701)"
(2026-06-18). This is the commit that introduced `sparse_strips/vello_cpu/src/coarse/depth.rs`
(and the whole depth-buffer mechanism) in its current form; `git log --follow`/`git log -S` on the
file both terminate at this single commit, so the file (and this bug) has existed unchanged since
its introduction.

## The fix

Route both `BucketRange::span` and the free function `bucket_span` through the same saturating
computation: compute `start * DEPTH_BUCKET_WIDTH` and `end * DEPTH_BUCKET_WIDTH` in `usize`, clamp
each to `u16::MAX` before narrowing back to `u16`, and derive `width` as `end_px - x` (both
already `u16`, and `end_px >= x` always holds since `end >= start`). No real pixel coordinate can
ever legitimately exceed `u16::MAX`, and every caller of `bucket_span`/`BucketRange::span`
immediately intersects the result with a real, in-bounds request span - so a saturated (too-wide)
span behaves identically to the mathematically-exact one once intersected, while never
overflowing or degenerating into a bogus zero-width span.

Verified:
- `repro/` re-run against the patched tree: debug build no longer panics ("OK: ..."), release
  build no longer shows bled-through pixels ("OK: ..." - row 1 is fully transparent) - 1/1 clean
  run each, both immediately following the confirmed-failing pre-patch runs in the same session
  (see `output.txt`).
- Varied input within the same bug class (`CONFIRMATION.md` section 7): re-ran both the direct
  `bucket_span(0, 512)` unit-level check and the two-row bled-pixel scenario; both now produce the
  expected values (`Span { x: 0, width: 65535 }`, saturated rather than wrapped; row 1 fully
  transparent) instead of overflowing/corrupting.
- Project test suite (`cargo test -p vello_cpu`, this crate's whole lib + doc-test suite, the only
  crate this patch touches): **81 passed, 0 failed (lib) + 1 passed, 0 failed (doc-tests) before
  the patch; identical 81 passed, 0 failed (lib) + 1 passed, 0 failed (doc-tests) after the
  patch** - no existing test exercises this extreme a width, so the counts are unchanged either
  way; the patch introduces no regressions.

## What's not done / open questions

- The fix is narrowly scoped to the two overflowing multiplications; it does not add any
  validation to `RenderContext::new`/`reset_and_resize` itself (e.g. rejecting or clamping
  pathologically large widths) - a maintainer may separately want to decide whether extreme
  widths this close to `u16::MAX` should be rejected earlier instead of merely handled
  gracefully once reached.
- Did not check whether other u16-domain multiplications elsewhere in the coarse/fine rasterizer
  (outside `depth.rs`) have the same "count times a fixed granularity constant" shape and could
  overflow at similarly extreme widths/heights - this session only looked at the specific spot
  named in the candidate description. A systematic sweep for `<count>: u16 * <constant>: u16`
  patterns elsewhere in `sparse_strips/vello_cpu`/`vello_common` would be a reasonable follow-up
  (see `NewSuspectsNoticed` in the final report for one already-spotted example).
- The release-mode logic-corruption manifestation (stale-pixel bleed across strip rows) was only
  demonstrated for the specific "opaque row followed by blank row" scene shape in `repro/`; other
  scene shapes at this width (e.g. a blank canvas with zero draws at all, or many alternating
  opaque/blank rows) were reasoned about but not each individually built into a separate repro -
  the underlying mechanism (the whole-row unset-run degenerating to zero-width) is the same for
  all of them.
