# absolute_clip_rect_and_geometry memoization (focus/window sweep)

`ItemRc::absolute_clip_rect_and_geometry` (`internal/core/item_tree.rs:496-514`) is called by
`is_visible()`, which `internal/core/window.rs`'s `process_mouse_input` calls **unconditionally on
every dispatched mouse event** (including plain `MouseMoved`) to check whether the currently
focused item became invisible, and which `process_key_event` calls the same way on every key
press. It was defined recursively: for a clipping ancestor, it obtained that ancestor's own
window-space geometry by calling `ancestor.absolute_clip_rect_and_geometry()` again (discarding
the clip half of the result) - which itself repeats the same recursive definition for *its own*
ancestors. For `k` nested `clip: true` ancestors above the item being checked, this redundantly
re-walks overlapping sub-chains: `f(k) = f(k-1) + f(k-2) + ... + f(0)`, i.e. **O(2^k)**, not the
O(k) the code visually looks like.

## What was tried

Rewrote `absolute_clip_rect_and_geometry` as a single pass: collect the ancestor chain once
(walking up, O(k)), then fold the transform-to-window in root-to-self order (walking back down
the collected list), computing each ancestor's own window-space geometry exactly once instead of
recursively. This mirrors `local_to_window_transform`'s per-step logic (`children_transform` then
translate by `geometry().origin`) applied incrementally instead of restarting it from scratch for
every ancestor. Result: O(k) per call, no recursion, same return value.

Standalone bench (`bench/`, modeled on candidate 055's `focus_bench`): a `TextInput` focused
`depth` levels deep inside unrolled `ClipChain<depth>` `.slint` components (5/12/18/20/22 levels
of `Rectangle { clip: true; ... }`, Slint has no recursion so each depth is its own generated
component). One `Tab` press focuses the leaf, then `iters` consecutive `PointerMoved` events are
timed - each one re-triggers `is_visible()` on the focused item via `process_mouse_input`.

| depth | baseline p50 (unpinned) | patched p50 (unpinned) | baseline p50 (`taskset -c 3`) | patched p50 (`taskset -c 3`) |
|---|---|---|---|---|
| 5  | 5.74us / 5.84us (2 runs) | 1.82us / 1.82us (2 runs) | - | - |
| 12 | 584us / 597us (2 runs) | 3.5us / 3.5us (2 runs) | - | - |
| 18 | 38.9ms / 40.1ms (2 runs) | 5.14us / 5.18us (2 runs) | - | - |
| 20 | 153.9ms / 150.6ms (2 runs, unpinned); 299.4ms / 297.9ms (2 runs, pinned) | 5.6us / 5.7us (2 runs, unpinned); 5.68us / 5.64us (2 runs, pinned) | 299.4ms | 5.68us |
| 22 | 638ms / 623ms (2 runs) | 6.0us / 6.0us (2 runs) | - | - |

All timings are per single dispatched `PointerMoved` event (i.e. per-frame mouse-move cost for a
focused item this deeply clipped), min/p50 stable across repeats (spread under 5% at every depth,
both binaries) - see raw output in this candidate's session log, not re-pasted here for space.

## Result

Real, and the effect size only grows with nesting depth, consistent with the O(2^k) -> O(k)
change: **~3.1x at depth 5, ~170x at depth 12, ~7,700x at depth 18, ~27,000x (unpinned) / 52,700x
(pinned) at depth 20, ~104,000x at depth 22.** In absolute terms, a focused item nested 20 levels
deep inside `clip: true` containers cost ~150-300ms of main-thread time on **every single mouse
move**, before the patch; after, ~5.7us regardless of which run or whether pinned. Patched-side
timings across depth 5-22 stay in a tight 1.8-6.0us band (mildly super-linear, consistent with
O(k) plus a small per-level constant), confirming the exponential term is gone rather than just
scaled down. Not measured multi-core/unpinned-vs-pinned as a *ratio* beyond the depth-20 spot
check above: this is single-threaded work triggered by one dispatched event, so core count does
not change what is being measured, only ambient noise - both pinned and unpinned readings agree
to within run-to-run noise on both sides.

## Does it change behavior/output?

- **Bit-exact?** Yes. `absolute_clip_rect_and_geometry` returns the same `(clip, geometry)` pair
  by construction (same inputs consumed in the same combination, just without recomputing shared
  sub-chains); the bench's own correctness assertion (`leaf_has_focus` stays `true` after every
  dispatched event, at every depth, both binaries - a wrong `is_visible()` result would have
  dropped the focus via `take_focus_item`) held on every run.
- **Is this a breaking change?** No - private/internal function, no public API or file format
  touched.
- Project test suite: **307/307 passed, 0 failed** both before and after
  (`cargo test -p i-slint-core --lib`), including `item_tree::tests::test_visibility_with_clip_under_transform`
  which directly exercises this function under a clip+transform ancestor. **`cargo test -p slint`:
  19 result lines, same pass counts (0/1/1/1/1/2/24/2/1/4/3/1/1/1/18/1/1/1/18), 0 failed, 0
  `^error` matches, both before and after** (`--all-features` on `i-slint-core` fails to build for
  an unrelated, pre-existing reason - a `zeno`/no_std feature interaction documented in
  `projects/slint.md` - so plain `cargo check -p i-slint-core` was used to confirm the patched
  crate itself compiles cleanly, which it does).

## Why the result looks the way it does

The original code's only actual purpose in calling the full recursive function on each ancestor
was to obtain that ancestor's *own* geometry (`.1` of the pair) - the recursive call's own `clip`
half (`.0`) was discarded (`let (_, ancestor_geom) = ...`). Nothing about the algorithm actually
needed clip-of-clip composition; each clipping ancestor's window-space rectangle only depends on
the transform chain from that ancestor to the root, which is exactly the *tail* of the chain
already available while walking up from `self` - collecting ancestors once and folding the
transform top-down computes every ancestor's geometry in one shared pass instead of restarting
the walk from scratch, recursively, for each one.

## What's not done / open questions

- `is_visible_or_clipped_by_flickable` (`item_tree.rs:436-461`, used by `window.rs`'s Tab-focus
  traversal) still calls the now-fixed `absolute_clip_rect_and_geometry` once per ancestor in its
  own separate loop - each such call is now O(k) instead of O(2^k), so one call to
  `is_visible_or_clipped_by_flickable` is O(k^2) instead of O(2^k). A true O(k) fix would need to
  fold that function's own ancestor walk into the same single top-down pass (the "clip valid at
  ancestor A's own level" value it needs is exactly the running `clip` this patch already
  accumulates one level further up) - not attempted here to keep this candidate to the one
  function named in the backlog entry; left as a follow-up in `IDEAS.md`.
- Not measured: the compounding cost when `move_focus`'s Tab-navigation loop (which itself visits
  many candidate items per keypress) walks a tree where *many* items each have deep clipping
  ancestor chains - only a single focused item's cost was isolated here. Candidate 055's own
  cycle-detection fix touches the same traversal; a combined scene (many Tab-navigable items,
  each deeply clipped) is a plausible follow-up bench.
- Real hardware / GUI-visible confirmation not done - this is a headless, single-event
  microbenchmark. Given the effect only exists for unusually deep `clip: true` nesting (the
  `perf_test` harness's own `nest` mode goes no deeper than 16 levels and does not clip at every
  level), whether any shipped `.slint` UI reaches depth 15+ *nested clipping* in practice is
  unverified - this fixes a real algorithmic cliff, not a demonstrated real-world UI regression.
