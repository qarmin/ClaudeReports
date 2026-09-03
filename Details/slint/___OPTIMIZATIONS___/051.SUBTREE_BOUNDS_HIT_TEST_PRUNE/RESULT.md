# 51 - Subtree bounds for hit-testing: pruning `send_mouse_event_to_item`'s blind descent

IDEAS.md's top backlog entry split candidates 049/050's render-only subtree-bounds cache into
three still-open consumers (hit-test, layout, dirty-region walk), each needing its own design.
This candidate implements the hit-test one: candidate 015 found that `send_mouse_event_to_item`
(`internal/core/input.rs`) descends into **every** child of an item whose own geometry doesn't
contain the cursor and which doesn't clip its children (`InputEventFilterResult::ForwardAndIgnore`
forwards blindly, because a child may legitimately sit outside its parent's bounds) - at 100 000
rows, one mouse move cost +38 ms, ~98 ns/item, linear in total item count regardless of what's
visible.

## What was tried

Exposed candidates 049/050's already-computed, already-fresh-every-frame per-item
`subtree_bounds` (the union of a descendant's bounding rects, in the item's own children
coordinate frame) through a new `RendererSealed::cached_subtree_bounds(&ItemRc) -> Option<LogicalRect>`
method (default `None`, conservative), implemented for `SoftwareRenderer` by delegating to a new
`PartialRenderingState::cached_subtree_bounds` read-only accessor. `send_mouse_event_to_item`
consults it only in the exact branch candidate 015 identified (cursor outside the item's own
geometry, item doesn't clip its children): `event_for_children`'s position is already translated
into the item's own children coordinate frame at that point in the function - the same frame
`cached_subtree_bounds` is expressed in - so no extra transform is needed. If the cached bound
doesn't contain that position, `forward_to_children` is set to `false` instead of unconditionally
`true`, skipping the recursive visit into that item's children entirely. `None` (no cache, e.g. a
non-`PartialRenderer`-based backend, or an item created since the last render) falls back to the
original always-descend behavior.

This piggybacks entirely on data 049/050 already maintain for rendering; no new cache, dirty
flag, or invalidation logic was added for this candidate. The full patch (`patch.diff`) therefore
includes 049+050's mechanism plus this candidate's ~25-line consumer, since it must apply cleanly
against plain `master`.

`--mode scroll --count 100000 --pointer-move` (one extra `PointerMoved` dispatched per frame, the
scenario candidate 015 used), `perf_test`'s software (`--bench`) renderer, `rdebug` profile, paired
interleaved runs (`tools/paired.sh`, RUNS=9, FRAMES=20), baseline = plain `master`
(`cdb6c43f7`, no 049/050/051):

| row style | single-core (`taskset -c 2`) | multi-core | notes |
|---|---|---|---|
| `layout` (5 items/row) | **-30.6% (9/9)** | **-29.7% (9/9)** | candidate 015's headline scenario |
| `touch` (`layout` + `TouchArea`, hover feedback) | **-32.1% (9/9)** | - | exercises the exact code path a real clickable row uses |
| `clipped` (`clip: true`) | **-24.5% (9/9)** | - | `clips_children()` already opts out of the old blind-forward for the row itself, so this is smaller than `layout`'s but still real - the row's own *children* still benefited |
| `bare` (1 item/row, no children) | -0.8%, **not consistent (5/9)** | - | expected: a leaf row has no children to skip descending into, so the new check is a no-op read that costs about as much as it (rarely) saves |

## Result

A real, consistent win at every row style with children to prune (-24.5% to -32.1%, single- and
multi-core, 9/9 paired), matching the magnitude of candidate 049's render-side prize on the same
data. `bare` (the floor, no descendants) is a wash, as expected - there's nothing for this
optimization to skip there, only a cheap `Option` check to pay for.

## Does it change behavior/output?

- **Bit-exact?** Yes, on every scenario checked.
- `tools/pixelcheck.sh` (RUNS=5, `--bench`, 6 frames): pixel-identical outside the noise floor on
  `scroll --row-style layout/bare/clipped/touch` with `--pointer-move`; `scroll --row-style touch
  --pointer-move` combined with `--mutate 20 --mutate-mode push/insert-front/rotate`, `--resize-step 5`,
  and `--pointer-jitter`; `list` (virtualized), `rects`, `text`, `nest` with `--pointer-move`; and a
  no-pointer-move sanity run. `--row-style touch` is the meaningful check here: its `TouchArea`
  reads `has-hover` to paint a highlight, so any case where the pruning wrongly skips an item that
  should have received the event would show up as a missing/wrong hover highlight - it did not.
- **Is this a breaking change?** No - `RendererSealed` is a sealed trait with a conservative
  default (`None`), so every renderer backend other than the software renderer is unaffected byte-
  for-byte. `cargo test -p i-slint-core --lib` (307 tests) passes unchanged.
- Only wired up for the software renderer (what `perf_test --bench` and this measurement use).
  Skia and femtovg also build on `PartialRenderer`/`PartialRenderingState`-equivalent machinery in
  places, but wiring the same accessor into their `RendererSealed` impls was not attempted here -
  see "What's not done" below.

## Why the result looks the way it does

`compute_dirty_regions` (a mandatory, already-existing per-frame pass) refreshes every item's own
bounding rect regardless of screen visibility - Slint doesn't clip children by default, so it has
to. Candidates 049/050 built a per-item subtree-bounds union on top of that data purely for
render-side pruning. That union turns out to answer the input traversal's question too: "does the
cursor, translated into this item's own frame, fall anywhere a descendant might be" - with zero
new per-frame computation, because 049/050 already keep it fresh incrementally. The saving is
exactly the same shape as 049's render-side one: one `Option<LogicalRect>` cache read replaces an
unconditional recursive visit into every child, at every row that isn't directly under the cursor.

## What's not done / open questions

- Only implemented for the software renderer's `RendererSealed` impl. Skia's own `PartialRenderer`
  usage was not touched - would need the same `cached_subtree_bounds` delegation added to its
  `impl RendererSealed for SkiaRenderer`. FemtoVG has migrated off `PartialRenderer` onto a
  different (`anyrender`-based) abstraction entirely and has no equivalent cache to delegate to at
  all - giving it this optimization would need its own subtree-bounds mechanism, not a delegation.
- The narrow staleness window candidate 015/049/050 already accept for rendering applies here too,
  slightly differently: `cached_subtree_bounds` reflects the *last completed render*, not any
  property writes made earlier in the same batch of processing before this input event. In
  practice this means hit-testing what's currently on screen (arguably the more correct behavior
  for a mouse event - the user clicked what they see), but a pathological case exists: an item
  created and given a mouse handler in the same frame, before the next render, whose parent's
  cached `subtree_bounds` doesn't yet include it, could have a click incorrectly pass through if
  the click lands outside the old (pre-creation) union. Not exercised by any scene here (`perf_test`
  has no such "create then immediately click, same frame, no intervening render" scenario) and
  self-heals on the very next render; flagged here rather than silently accepted.
- The remaining two backlog items from the same source entry (the dirty-region walk's own
  bookkeeping, and layout's `Text::layout_info`) are still open - see IDEAS.md, they need designs
  of their own per the original backlog note.
- No attempt was made to also prune the case where the cursor position is `None` (some `MouseEvent`
  variants have no position) - the check requires a concrete point to test against `subtree_bounds`
  and is skipped entirely for those, unchanged from before.

## Reproduce

```sh
tools/paired.sh <baseline_bin> <patched_bin> -- --mode scroll --count 100000 --row-style layout --pointer-move
tools/pixelcheck.sh <baseline_bin> <patched_bin> -- --mode scroll --count 20000 --row-style touch --pointer-move
```
