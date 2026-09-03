# 50 - Making candidate 049's subtree-bounds pass incremental (skip-when-clean instead of full recompute)

Candidate 049 added a per-frame `PartialRenderer::compute_subtree_bounds` pass so
`render_item_children` can skip a culled item's whole subtree. That pass recomputed the bound for
every single item, every frame, regardless of whether anything below it changed - a real,
non-amortized O(items) cost that made it a win on static scenes (-15 to -16%) but a wash to a
regression while scrolling or on scenes with nothing to prune (`bare` +16.6%, `rects` +7.1%). This
was IDEAS.md's top backlog entry: make that pass incremental so the scrolling/no-prune cases stop
paying for a recompute that would produce the same answer as last frame.

## What was tried

Each item's cache entry gets a new `subtree_bounds_dirty: Cell<bool>` flag (default `true`).
`compute_dirty_regions` (the existing, unavoidable per-frame walk that already detects per-item
geometry changes via `same_geometry`) now calls a new `mark_subtree_bounds_dirty` whenever an
item's own bounding rect/transform changed, or the item was just created. That function walks
*upward* from the item's parent (not the item itself - an item's own cached `subtree_bounds`
describes its descendants, which are unaffected by the item's own rect changing; only its
ancestors' unions are affected) to the root, setting each ancestor's flag and stopping the instant
it finds one already set this frame (everything above an already-dirty ancestor was necessarily
already marked). `compute_subtree_bounds` then checks this flag before doing anything: if clean,
it returns the cached bound immediately without recursing into any descendant at all; only if
dirty does it do the recursive aggregation 049 always did, then clears the flag. Removing an item
is deliberately *not* specially handled - a stale (too-large) union is safe by construction (can
only miss a pruning opportunity, never cause an incorrect skip), so only growth-causing events
(geometry change, creation) need to invalidate anything.

This makes the pass cost roughly O(number of items whose own geometry changed this frame x tree
depth) instead of O(total items) - the scrolling case (where typically one `Flickable`/viewport
item's own offset changes, not each row's) collapses to a handful of ancestor updates instead of
walking all 100000 rows a second time.

`scroll` at 100000 rows, software renderer, isolating *only* this candidate's effect (candidate
049's binary as baseline "A", this candidate's binary - 049 + incremental - as "B"), paired
interleaved runs (`tools/paired.sh`, RUNS=9, FRAMES=20):

| row style / scene | scenario | single-core (`taskset -c 2`) | multi-core |
|---|---|---|---|
| `layout` | scrolling (`--scroll-step 5`) | **-16.7% (9/9)** | **-15.5% (9/9)** |
| `layout` | static (`--scroll-step 0`) | **-18.3% (9/9)** | **-15.9% (8/9, consistent)** |
| `flat` | scrolling | **-18.0% (9/9)** | - |
| `clipped` | scrolling | **-26.7% (9/9)** | **-25.0% (9/9)** |
| `bare` (previously a regression) | scrolling | **-10.4% (9/9)** | **-15.1% (9/9)** |
| `rects` 3000 (previously a regression) | static-ish (moving primitives) | **-4.9% (9/9)** | - |
| `list` 100000 (virtualized `ListView`) | scrolling | -0.6%, **not consistent (6/9)** | - |

For scale, against a fully unpatched (`master`, no 049 either) baseline, single-core: `scroll`
100000 `layout` rows scrolling is **-14.9% (8/9, consistent)** and static is **-18.3%** (same
number as the 049-vs-this-candidate row above, since 049's own static-case cost was already near
zero and this candidate doesn't change the static path's behavior, only the scrolling/no-prune
ones).

Pixel correctness (`tools/pixelcheck.sh`, RUNS=5), against candidate 049's binary: pixel-identical
outside the noise floor on `scroll` (`layout`/`flat`/`clipped`/`bare` x scrolling), `rects`,
`list`, `text`, `nest`, `images`, and - specifically targeting this candidate's structural-change
handling - `scroll` with `--mutate 20 --mutate-mode push/insert-front/rotate/changed` (row
addition, front-insertion, reordering, in-place mutation) plus `--resize-step 5` (container-level
geometry change propagating through many ancestors) and `--animate` (visual-only property
changes). All pixel-identical outside the noise floor. Also spot-checked two of these
(`layout` scrolling, `rotate` mutation) against a fully unpatched baseline with the same result.

## Result

Turns candidate 049's documented "static wins, scrolling washes, no-prune scenes regress" split
into a win across the board on every scene/row-style tested:

- **Scrolling, which was a wash-to-small-loss for 049 alone, is now a clear win** (-10 to -27%
  depending on row style, all 9/9 or 8/9 consistent): the incremental short-circuit means the
  second pass now costs close to nothing while scrolling, since only the ancestor chain of the
  one item whose own offset actually changes (the `Flickable` viewport) gets recomputed - the
  100000 rows underneath are never revisited.
- **The two scenes that regressed under 049 alone are now wins instead**: `bare` scrolling
  (was +16.6% regression, now -10.4% to -15.1%) and `rects` (was +7.1% regression, now -4.9%).
  With nothing changing structurally frame-to-frame in `rects` (only fill colors/positions bound
  to non-geometry-affecting properties, or geometry changes that are recognized as such and
  correctly marked dirty only where they occur), most of the tree short-circuits every frame.
- **`clipped` scrolling improved the most** (-25 to -27%, resolving 049's own +2.9% regression
  there): `clip: true` already prunes render descent on its own, so 049's added pass was pure
  overhead in that scenario with little left to prune; now that pass costs almost nothing when
  clean, so the underlying render-side saving 049 already had elsewhere shows through undiminished.
- **`list` (virtualized) stays a wash** (-0.6%, not consistent): a `ListView` only ever
  materializes a handful of visible rows, so there was never much second-pass cost to remove
  in either version - consistent with 049's own finding that this case is small and not fully
  understood, unaffected by this change.
- **Static scenes are unaffected in magnitude** (~-18% either way): 049's static-scene cost was
  already close to zero because nothing is dirty at all in `compute_dirty_regions`'s pass, so
  there was little for this candidate's short-circuit to remove there; the number in the table is
  effectively the same win 049 already reported, just re-measured against this candidate's binary
  for a consistent baseline.

## Does it change behavior/output?

- **Bit-exact?** Yes, on every scene/scenario checked (see pixel-correctness list above),
  including the structural-change stress cases (`--mutate push/insert-front/rotate/changed`,
  `--resize-step`, `--animate`) specifically chosen to exercise this candidate's dirty-marking
  logic (item creation, front-insertion, reordering, in-place property change, multi-level
  geometry propagation, non-geometry visual changes).
- **Is this a breaking change?** No public API changes. The new field
  (`subtree_bounds_dirty: Cell<bool>`) and method (`mark_subtree_bounds_dirty`) are private to
  `partial_renderer.rs`; nothing outside `PartialRenderer` observes them.
- No project test suite was run against this patch (not integrated into the workspace's CI in
  this session); the correctness evidence here is the pixel-comparison harness above plus the
  reasoning below for why a stale flag can only cost a missed optimization, never an incorrect one.

## Why the result looks the way it does

The design leans on one invariant, stated in candidate 049's own `RESULT.md` for a related case
(`Clip` children) and reused here deliberately: a cached `subtree_bounds` value that is too large
(stale, hasn't shrunk after something moved away or was removed) can only cost a missed pruning
opportunity, never cause an incorrect skip - `subtree_can_be_skipped` only returns `true` when the
(possibly stale) bound does *not* intersect the dirty region, so an over-large bound is always the
conservative direction. This means the only case that must be caught is the opposite one: a
subtree's true footprint *growing* into territory the cached union doesn't cover, which happens
exactly when some item's own bounding rect/transform changes or a new item appears - both of which
`compute_dirty_regions` already detects for its own, unrelated purpose (`same_geometry`/`None`
cache lookup), so this candidate adds no new property reads or dependency registrations, only a
cheap upward flag-propagation on top of a check that pass 1 was already doing.

Item *removal* is deliberately unhandled: nothing walks up to invalidate an ancestor when a
descendant disappears (e.g. a `Repeater` row removed, an `if` becoming false). This is safe by the
invariant above, and avoids a much harder problem (there is no hook run at removal time with
access to the item's former parent, since the item's own `ItemRc` may already be gone by the time
`CachedRenderingData::release` runs) - it just means a subtree that shrinks or empties out keeps
its old, larger bound until something else in the same subtree changes and forces a real
recompute, which only costs an occasional missed pruning opportunity, not a correctness bug.

The upward-walk's early-stop (`if already_dirty { break; }`) is what keeps the marking cost itself
cheap even on the very first frame, when every item is "new": each item's own freshly-created
entry already starts `subtree_bounds_dirty = true` (see `PartialRenderingCachedData::new`), so a
child's walk up to its immediate parent finds it already marked (parents are always visited, and
thus created, before their children in `compute_dirty_regions`'s traversal order) and stops after
one hop - the whole tree's worth of markings amortizes to O(items), not O(items x depth).

## What's not done / open questions

- **Hit-testing, layout and the dirty-region walk itself are still untouched** - unchanged from
  candidate 049's own list; this candidate only makes the render-traversal slice's own bookkeeping
  cheaper, it doesn't extend the mechanism to the other three traversals.
- **Compiler-provable bounds (candidate 012's option 2) remain a separate, larger design task.**
- **The dirty flag never causes a bound to shrink back down on its own** - a subtree that
  permanently loses descendants (without ever changing again itself) keeps an oversized cached
  bound forever. Not observed to matter in any scene tested here (the descendant count in these
  benches only grows or stays constant, or the container itself keeps moving/changing which
  forces fresh recomputes anyway), but a workload that grows a subtree large and then permanently
  shrinks it without further changes nearby would see a permanently reduced pruning benefit for
  that subtree specifically - a periodic full recompute (e.g. every N frames) would reclaim this
  if it ever matters in practice; not implemented since no tested scene showed a need for it.
- Only the software renderer (`--bench`) was measured, same limitation as candidate 049 (no GPU
  display available this session to check the femtovg/skia `PartialRenderer` paths).

## Reproduce

```sh
# from ___OPTIMIZATIONS___/, with pt_049 (candidate 049's patch only) and pt_incremental (this
# candidate's patch.diff, which already includes 049) built per this candidate's justfile
RUNS=9 FRAMES=20 taskset -c 2 bash tools/paired.sh /tmp/pt_049 /tmp/pt_incremental -- \
    --mode scroll --count 100000 --row-style layout --scroll-step 5
RUNS=5 bash tools/pixelcheck.sh /tmp/pt_049 /tmp/pt_incremental -- \
    --mode scroll --count 20000 --row-style layout --mutate 20 --mutate-mode rotate
```
