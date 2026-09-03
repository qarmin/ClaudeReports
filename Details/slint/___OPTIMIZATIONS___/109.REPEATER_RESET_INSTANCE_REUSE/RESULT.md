# Repeater full reset reuses instances instead of destroying/recreating them

`RepeaterTracker::reset` (`internal/core/model/repeater.rs:613-616`, the `ModelChangeListener`
callback fired by any `.notify.reset()`, e.g. `VecModel::set_vec`/`clear`, or a `SortModel`/
`FilterModel` re-sort/re-filter, both of which forward through their own `reset()` to
`self.notify.reset()`) cleared `RepeaterInner::instances` entirely. The next `ensure_updated`/
`ensure_updated_listview` pass then treats every row as brand new: `RustRepeaterOps::splice`
inserts fresh `(Dirty, None)` slots, and `ensure_updated` sees `c.1.is_none()` for all of them,
so it calls `init()` (the create closure) and `RepeatedItemTree::init()` for all N rows, even
when N is unchanged and most of the underlying `ItemTreeRc` subtrees could have been reused with
just a new `update()` call - the same reuse-by-position `ensure_updated` already does for
`row_changed`/`row_added`/`row_removed`, just not applied to `reset`. From the 2026-08-23 models
discovery sweep, at the 20-60k row scale already used by candidates 043/044.

## What was tried

Changed `reset()` to mark every existing instance `Dirty` in place (keeping its `Option<ItemTreeRc<C>>`)
instead of calling `instances.clear()`. The next `update_all_instances`/`ensure_updated` pass then
calls `update(row, data)` on each already-created instance (reusing the `ItemTreeRc`, its subtree,
and its item-tree registration) rather than dropping it and calling `init` on a freshly allocated
one. Row count changes are still handled the same way as before (`update_all_instances`'s
`splice` grows/shrinks the `Vec` around whatever's left).

Measured with `perf_test`'s `scroll` scene (non-virtualized `for`, so every row is instantiated)
plus a new `--mutate-mode reset` (added to `perf_test` for this candidate - see README.md): each
frame calls `VecModel::set_vec` with a same-length, entirely-different-content `Vec`, the shape a
`SortModel`/`FilterModel` re-sort/re-filter or a `VecModel::set_vec`/`clear` call takes (one
`notify.reset()` instead of per-row notifications). Two `--row-style`s to separate the two costs
`update()` pays regardless of this patch (text shaping) from the cost this patch actually removes
(subtree alloc/dealloc + reinitialization): `bare` (`Rectangle` only, no text) isolates the
structural cost; the default `layout` row style (`Rectangle` + `HorizontalLayout` + 3 `Text`)
keeps the text-shaping cost that dominates a real row.

`tools/paired.sh`, `RUNS=9 FRAMES=15` (unpinned = multi-core; `taskset -c 2` = single-core):

| Variant | rows | row style | single-core p50 | multi-core p50 | notes |
|---|---|---|---|---|---|
| baseline | 20 000 | `bare` | 18.26ms | 17.99ms | |
| patched | 20 000 | `bare` | 12.81ms (**-29.8%**, 9/9) | 12.67ms (**-29.6%**, 9/9) | |
| baseline | 60 000 | `bare` | 50.16ms | 50.50ms | |
| patched | 60 000 | `bare` | 37.18ms (**-25.9%**, 9/9) | 37.33ms (**-26.1%**, 9/9) | |
| baseline | 20 000 | `layout` (default) | 720.33ms (single-core, `RUNS=7 FRAMES=10`) | 717.04ms (`RUNS=9 FRAMES=15`) | shaping-dominated |
| patched | 20 000 | `layout` (default) | 701.70ms (**-2.6%**, 7/7) | 715.49ms (-0.2%, 2/9 - NOT consistent) | shaping-dominated |

Allocations/frame at 20 000 rows, `bare`: baseline 412 354 -> patched 232 107 (**-43.7%**).
At 20 000 rows, `layout`: baseline 2 711 443 -> patched 1 951 085 (**-28.0%**) - real and
reproducible, but a small fraction of frame time at this row style (see "Why" below).

## Result

A real, consistent wall-clock win when the per-row structural cost (item-tree alloc/dealloc,
property/binding graph setup) is what dominates a frame: -26 to -30%, 9/9 paired wins at both
20 000 and 60 000 rows, `--row-style bare`. When per-row text shaping dominates instead (the
default row style, 3 `Text` items per row, all of whose content changes on every reset), the same
allocation saving (-28% allocations/frame) is real but the wall-clock effect shrinks to a small,
only-just-detectable win under low-noise single-core pinning (-2.6%, 7/7 consistent) and
disappears into noise unpinned (-0.2%, 2/9 - not consistent per `tools/paired.sh`'s bar).

## Does it change behavior/output?

- **Bit-exact?** Yes. `tools/pixelcheck.sh` (`RUNS=5`, 20 000 rows, both row styles, `--mutate 1
  --mutate-mode reset`): 0 px differing outside a 0 px same-binary noise floor for both.
- Every row's `update(row, data)` is still called with the post-reset data on every reset, for
  every instance, exactly as before - the only thing skipped is destroying and reallocating the
  `ItemTreeRc` and its subtree, and re-running `RepeatedItemTree::init()` (documented as "called
  once after the ItemTree has been instantiated", which no longer happens for previously-existing
  slots on a reset, since they were never re-instantiated). A row whose `init()` has an
  observable side effect beyond what `update()` also does (rare - the only two callers in this
  codebase are the property-tracking scaffolding in `update()`'s own path) would not see that
  side effect fire a second time. Not exercised by generated Slint code today, which calls `init`
  only to set up bindings that `update` doesn't already cover.
- Any transient, non-model-driven state living directly on a repeated instance's own item tree
  (e.g. keyboard focus if it happened to be on a row, a `TextInput`'s selection) previously would
  have been unconditionally destroyed by a reset (the instance is dropped and recreated) and is
  now instead retained across the reset, since the underlying `ItemTreeRc` survives. This is a
  user-visible behavior difference under a specific, narrow condition (something has focus/
  selection on a row *and* the model is fully reset that frame) - not covered by `perf_test`'s
  scenes (no focus/selection state is set up in the `scroll`/`list` scenes) or by
  `cargo test -p i-slint-core`'s existing suite, so it is asserted from reading the code
  (`ensure_updated` calling `update()` on the existing instance rather than `init()` on a new
  one), not measured directly.
- **Is this a breaking change?** No public API or generated-code signature changes. The behavior
  difference above is a semantic corner case (see above), not a compile-time or ABI break.
- Project test suite: `cargo test -p i-slint-core --all-features` - baseline 319 passed / 0
  failed (unit) + 71 passed / 0 failed / 11 ignored (doctests) + 1 passed / 0 failed (compile-fail
  doctest); patched: same counts, all passing (319 / 71+11 ignored / 1). No new failures.

## Why the result looks the way it does

Frame time in `perf_test`'s `scroll` scene with the default row style is dominated by text
shaping (~60 000 `parley` shapes/frame at 20 000 rows x 3 `Text` per row - see the workspace's
`README.md`/candidate `003`), which `update()` triggers regardless of whether the instance was
just recreated or reused; the patch only removes the *structural* cost of destroying and
reallocating the item tree and re-registering its property/binding graph, which is a small slice
of a frame this expensive. Strip the text out (`--row-style bare`) and that structural cost is
close to the entire frame, which is where the full -26 to -30% shows up. This mirrors candidate
`058`'s finding on the same file: a real, reproducible allocation reduction that is invisible in
wall-clock time until a scene exists where the removed cost isn't dwarfed by something else -
except here the removed cost scales with row count (unlike `058`'s fixed one-allocation-per-
repeater cost), so it's large enough to show up even amid shaping, just not enough to clear this
machine's 8-9/9 "consistent" bar unpinned.

## What's not done / open questions

- Not measured on a `list` (virtualized `ListView`) scene: virtualization already limits the
  visible instance count to roughly one screenful (~25-30 rows regardless of model size), so a
  reset there recreates only that many instances either way - the fix should still save
  allocations proportionally but the absolute win is expected to be far smaller in wall-clock
  terms; not built since the `scroll` scene already isolates the mechanism cleanly at the row
  counts this workspace uses elsewhere (043/044).
- Not measured via an actual `SortModel`/`FilterModel` re-sort/re-filter call (only `VecModel::
  set_vec`, which reaches the identical `RepeaterTracker::reset` through the same `notify.reset()`
  call both adapters also make - confirmed by reading `internal/core/model/adapters.rs:381-383`/
  `785-787`, not separately benchmarked, since the code path from `notify.reset()` onward is
  identical regardless of which `Model` impl triggered it).
- The focus/selection-retention behavior difference noted above is asserted from reading the
  code, not exercised by a repro - worth a small `.slint` test the next time a focus-related
  candidate is being investigated (deliberately out of scope for `perf-optimize`, which does not
  add new unit tests to the project).
