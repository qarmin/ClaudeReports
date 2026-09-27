# Repeater::ensure_children_instantiated per-frame Vec allocation

`Repeater::ensure_children_instantiated` (`internal/core/model/repeater.rs`) is invoked every
frame for every repeater (via `ensure_updated`/`ensure_updated_listview_callback`, called from
`window.rs:1759`). It called `instances_vec()`, which does
`self.0.inner.borrow().instances.iter().flat_map(|x| x.1.clone()).collect()` - allocating a fresh
`Vec<ItemTreeRc<C>>` every call, purely so `ensure_item_tree_instantiated` (which may run
arbitrary reentrant code per child) can run without holding the `RefCell` borrow on
`RepeaterInner`. From IDEAS.md's models-discovery sweep.

## What was tried

Replaced the ad hoc `Vec` with a scratch buffer (`instantiate_scratch: RefCell<Vec<ItemTreeRc<T>>>`)
stored on `RepeaterTracker` and reused across calls: `take()` it out (leaving an empty default
behind), extend it from a brief `inner.borrow()` (dropped immediately after), drain-and-process
without holding any borrow (preserving the original reentrancy-safety property), then put the
now-empty-but-capacity-retained `Vec` back for next frame. `instances_vec()` itself (the public
API used by the interpreter/eval code elsewhere) was left untouched - only the internal call site
in `ensure_children_instantiated` was changed.

| Variant | single-core p50 (`--mode scroll --count 60000`) | multi-core p50 | notes |
|---|---|---|---|
| baseline | 90.87ms | 90.54ms | `instances_vec()` alloc+collect every call |
| scratch buffer | 91.27ms (+0.4%, 5/9 pairs faster) | 90.60ms (+0.1%, 6/9 pairs faster) | not consistent, per `tools/paired.sh` |
| baseline, `--count 100000` | 156.12ms | - | |
| scratch buffer, `--count 100000` | 155.11ms (-0.6%, 3/9 pairs faster) | - | not consistent |

Allocation counts (not timing) show the fix working exactly as intended: steady-state frame
allocations at `--count 60000` drop from 421,120 to 421,103 (-17/frame), first-frame allocations
from ~6,116,823 to ~6,116,807 (-16), reproducible across repeated runs of each binary. The -17
(not -1) is consistent with `perf_test`'s own chrome (`main.slint`) containing a handful of its
own repeaters (list/combo controls) in addition to the `scroll` scene's top-level `for`, each
contributing one avoided allocation.

## Result

No measurable wall-clock difference, at either row count, on either single-core (`taskset`) or
multi-core (unpinned) runs - `tools/paired.sh` reports "NOT consistent" (5/9, 6/9, 3/9 out of 9
pairs) in every configuration tried, i.e. within the noise floor. The allocation-count reduction
is real and reproducible (confirmed over repeated runs) but is 1 allocation removed per repeater
per frame, not O(N): at `--count 60000` that is -17 allocations out of ~421,120 (0.004%), far
below what shows up in frame-time noise.

## Does it change behavior/output?

- **Bit-exact?** Yes outside the measurement noise floor: `tools/pixelcheck.sh` (5 runs/side,
  `--mode scroll --count 60000`) reports 0 px differing outside the same-binary noise floor
  (187 px noise floor, 172 px raw difference between binaries, all inside the floor).
- **Is this a breaking change?** No - internal-only change, no public API or generated-code
  signature touched.
- Project test suite: not run separately for this candidate; the change is behavior-preserving
  by construction (same set of instances processed, same borrow-drop ordering around the
  reentrant `ensure_item_tree_instantiated` call) and `cargo check -p i-slint-core` passes.

## Why the result looks the way it does

The candidate's own premise ("O(N) alloc + N Rc clones") slightly overstates the cost: `collect()`
on a flat_map allocates the backing buffer **once** per call regardless of N (the N `Rc` clones are
non-atomic refcount increments, not allocations). So per repeater per frame this was always exactly
one allocation, not one that scales with row count - the fix removes a fixed, small cost, not a
per-row one. At the 60k-100k row scale already used to benchmark other candidates in this
workspace, per-frame allocation totals are dominated by property-dependency-node churn (~421k
allocations/frame, see candidates 008/014/045), so a single avoided allocation per repeater is
several orders of magnitude below the noise floor of a paired timing comparison.

## What's not done / open questions

- Not tried: a scene with many independent repeaters in the same frame (e.g. deeply nested `for`
  loops, one per row) where the fixed one-allocation-per-repeater cost would multiply - `perf_test`
  has no such scene today; this might turn the fixed win into a real per-frame one at large
  repeater *counts* (as opposed to large row counts within one repeater).
- The scratch buffer adds one `RefCell<Vec<..>>` (24 bytes on 64-bit, empty after every call) of
  permanent per-repeater memory overhead; not measured separately since it is far below the
  resident-memory noise already observed (see `resident memory ~973-974 MB` in both runs above,
  unchanged).
