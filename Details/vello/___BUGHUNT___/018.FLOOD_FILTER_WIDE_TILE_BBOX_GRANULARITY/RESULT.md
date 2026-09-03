# Flood filter wide-tile-bbox granularity (issue #1423)

Issue #1423 claims the flood filter (`<feFlood>`) is implemented by computing the bounding box of
all drawn content inside the filter layer "in wide tile units" and flooding that whole area, so a
tiny 3x3 rect ends up flooding a full 256-pixel-wide wide tile - confirmed as a real bug by the
project's own collaborator in the issue thread ("Yes, that's incorrect... I haven't spent time
properly addressing it yet."). Investigated by driving the real, public `vello_cpu::RenderContext`
API end-to-end (`push_filter_layer(Flood)` -> `fill_rect` a 3x3 rect -> `pop_layer` -> render to a
real `Pixmap`) and checking whether pixels far from the 3x3 rect were also flooded.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction against the real public API on current HEAD (75168314): a 512x64 `RenderContext`, a flood filter layer containing a 3x3 rect at (250,30)-(253,33), sampling far-left (x=2) and far-right (x=509) pixels on the same row | release build, `cargo run --release` via the standalone `repro/` | Did **not** reproduce: only pixels in x=248..=255, y=28..=35 (an 8x8 area - the 3x3 rect expanded/snapped to `Tile::WIDTH`/`HEIGHT` = 4px boundaries) were flooded; far-left and far-right pixels stayed fully transparent. 3/3 runs, identical, deterministic. |
| Code reading: traced the pixmap-sizing path for a filter layer (`FilterLayerPlacement::new` in `sparse_strips/vello_common/src/filter/mod.rs`, called from `record.rs`) | Static | `pixmap_bbox` is computed as the tight bbox of the filtered content, expanded by the filter's own `filter_padding` (zero for `Flood`, since `FilterPrimitive::filter_expansion()` falls through to `Rect::ZERO` for `Flood`), then `.snap_to_tile_coordinates()` - which rounds to `Tile::WIDTH`/`Tile::HEIGHT` = 4px, **not** any 256px "wide tile" unit, on current HEAD. |
| Historical check: same manual boundary-value scan run against a scratch build pointed at commit 3df80378 (the commit immediately preceding the one identified below as the fix) | release build, separate scratch `cargo run --release` (not part of the packaged `repro/`, which targets current HEAD only - see `output.txt`) | **Did** reproduce exactly as the issue describes: the flood covered the *entire* 256-pixel-wide row (x=0..=255), confirmed against `WideTile::WIDTH = 256` in that commit's `sparse_strips/vello_common/src/coarse.rs`. 3/3 runs, identical, deterministic. |
| Bisection between the reproducing commit and current HEAD (`git log --oneline 3df80378..9c292860 -- sparse_strips/`, then re-running the same scan built against 9c292860 itself) | Static + one more scratch build | Exactly one commit separates the two (`9c292860`, "vello_cpu: A full rewrite of the frontend (#1701)"), and the scan against that commit already shows the fixed (tile-granularity) behavior - see "Introduced by" below. |

## Status

**Not reproducible** (on current HEAD, commit 75168314). Deterministic, 3/3 runs.

This bug **was real** as recently as commit 3df80378 (parent of `9c292860`), matching issue #1423's
description exactly (flooding the entire 256px-wide "wide tile" for a 3x3 rect), and reproduced
there 3/3 runs. It stopped reproducing once `9c292860` ("vello_cpu: A full rewrite of the frontend
(#1701)", merged well after the issue was filed) replaced the old `WideTilesBbox`-based filter-layer
pixmap sizing with the current `FilterLayerPlacement`/`pixmap_bbox` mechanism, which snaps to
`Tile::WIDTH`/`HEIGHT` (4px) instead of `WideTile::WIDTH` (256px). This looks like an incidental
side effect of that rewrite (a general frontend restructuring, not a fix targeted at this issue) -
the GitHub issue is still open, most likely simply because nobody has revisited/closed it since the
rewrite landed.

**Fully reproducible**: N/A (no bug present to reproduce on current HEAD - see Status). The
*investigation itself* is fully reproducible on demand: `just test` was verified end-to-end from a
completely fresh clone (see `output.txt`), and the supplementary historical scans against 3df80378
and 9c292860 were each repeated 3/3 times with identical results.

## Class

N/A (not Confirmed on current HEAD).

## Root cause

Not a bug in the current codebase. The filter-layer pixmap-sizing logic that issue #1423 describes
(computing the filtered content's bounding box, then quantizing it up to some tile-like granularity
before allocating/flooding the pixmap) is real and still exists in the current code
(`FilterLayerPlacement::new` in `sparse_strips/vello_common/src/filter/mod.rs`), but the granularity
it quantizes to is now `Tile::WIDTH`/`Tile::HEIGHT` (4x4 pixels - `sparse_strips/vello_common/src/tile.rs`),
not the old `WideTile::WIDTH` (256 pixels - `sparse_strips/vello_common/src/coarse.rs` as it existed
at commit 3df80378, before the June 2026 frontend rewrite removed the `WideTile`/`WideTilesBbox`-
based layer storage entirely). A 4x4-pixel quantization for a filter with zero padding (like
`Flood`) is a reasonable, tight-ish filter region, not the "whole width of a wide tile" the issue
complains about.

## Introduced by

N/A - not applicable to "introduced by" in the usual sense, since there is no bug present on current
HEAD to have been introduced. For context: the bug **was** present through commit 3df80378 ("Triple
script timeout on WASM (#1710)") and was fixed by the very next commit touching `sparse_strips/`,
9c292860 "vello_cpu: A full rewrite of the frontend (#1701)" - identified via `git log --oneline
3df80378..9c292860 -- sparse_strips/` (returns exactly that one commit) and confirmed directly by
re-running the same scan against each of those two commits (see `output.txt`).

## The fix

No fix needed/attempted - nothing to fix on the current tree (Status is Not reproducible). No
`patch.diff` is included.

- Project test suite: N/A, no patch was applied. Baseline sanity run on current HEAD (no changes):
  `cargo test -p vello_common --lib --release` -> **159 passed; 0 failed; 1 ignored**; `cargo test
  -p vello_cpu --lib --release` -> **81 passed; 0 failed**.

## What's not done / open questions

- This candidate checked only the `vello_cpu` (CPU/single-threaded) flood-filter path via the
  `RenderContext` public API. `vello_hybrid`'s filter implementation
  (`sparse_strips/vello_hybrid/src/filter.rs`) was not checked for whether it ever had, or still has,
  an analogous coarse-bbox issue - `vello_hybrid`'s core logic was separately rewritten by a later
  commit (`c3343862`, "vello_hybrid: A complete rewrite of the core logic (#1759)"), so its history
  doesn't necessarily track `vello_cpu`'s exactly.
- The GitHub issue (#1423) itself is still open upstream; this candidate's finding (that it's fixed
  as an incidental side effect of an unrelated large rewrite, not via any deliberate fix commit
  referencing the issue) may be worth relaying back so the issue can be closed, but this workspace's
  own convention is not to interact with GitHub issues directly from a bug-hunt session.
- Did not check whether other filter primitives with nonzero `filter_expansion`/`source_expansion`
  (`GaussianBlur`, `DropShadow`, `Offset`) have any analogous granularity looseness beyond the
  now-4px tile snap - out of scope for this candidate, which was specifically about the `Flood`
  case from issue #1423.
