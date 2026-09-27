# `is_visible_or_clipped_by_flickable` recomputes the same clip 2-3 times per call

`ItemRc::is_visible_or_clipped_by_flickable` (`internal/core/item_tree.rs:436-464`, used by
`WindowInner::move_focus`'s Tab-navigation traversal for every candidate item it visits) was:

```rust
pub(crate) fn is_visible_or_clipped_by_flickable(&self) -> bool {
    if self.is_visible() {                                  // calls #1: self.absolute_clip_rect_and_geometry()
        return true;
    }
    let geometry = self.absolute_clip_rect_and_geometry().1.to_box2d();   // call #2, same self, discards .0
    let mut parent = self.parent_item(...);
    while let Some(ancestor) = parent {
        if ancestor.borrow().as_ref().clips_children() {
            let (clip, ancestor_geo) = ancestor.absolute_clip_rect_and_geometry();  // call #3
            ...
        }
        parent = ancestor.parent_item(...);
    }
    false
}
```

The backlog entry ("candidate 100 follow-up") assumed this loop visits *every* clipping ancestor
(making it O(k) per ancestor times k ancestors = O(k^2) once candidate 100 made
`absolute_clip_rect_and_geometry` itself O(k)). Investigating it turned up a different, more
direct problem: **the loop always stops at the very first `clips_children()` ancestor it meets**
(the early `return` inside the `if` fires unconditionally once that ancestor is found  -  see "Why
the result looks the way it does" below for why it's mathematically guaranteed to fire), so this
was never O(k^2) - it does exactly one, two, or three calls to `absolute_clip_rect_and_geometry`
per invocation, not up to k. Two of those calls are provably redundant:

- Call #1 (`self.is_visible()`) and call #2 (`self.absolute_clip_rect_and_geometry().1`) are the
  *same call on the same item*, run back-to-back, with call #2's `.0` half thrown away.
- Call #3 (on the first clipping ancestor found) recomputes a value that is, by construction,
  bit-identical to `clip` from call #1: that ancestor's "own geometry intersected with every
  clipping ancestor above it" is exactly the same set of rectangles `self`'s own accumulated clip
  already intersects (nothing between `self` and the first clipping ancestor contributes any
  further clipping, by definition of "first"). So `item_in_clip` at that point can never disagree
  with the `visible` result already computed in call #1 - is_empty()-driven visibility is exactly
  order-independent for min/max-based rectangle intersection, including all `unwrap_or_default()`
  empty-set edge cases (see "Why" below for the full argument), so call #3's answer is always
  already known before it runs.

## What was tried

Rewrote the function to compute `absolute_clip_rect_and_geometry()` exactly once, reuse both
halves of its result, and reduce the ancestor walk to a purely structural search (no geometry
math) for the first `clips_children()` ancestor, only to check if it's a `Flickable`:

```rust
pub(crate) fn is_visible_or_clipped_by_flickable(&self) -> bool {
    let (clip, geometry) = self.absolute_clip_rect_and_geometry();
    // ... same visibility check as is_visible(), inlined ...
    if visible { return true; }
    let mut parent = self.parent_item(...);
    while let Some(ancestor) = parent {
        if ancestor.borrow().as_ref().clips_children() {
            return ancestor.downcast::<crate::items::Flickable>().is_some()
                && ancestor.is_visible_or_clipped_by_flickable();
        }
        parent = ancestor.parent_item(...);
    }
    false
}
```

Same return value in every case (see Correctness below), 1 `absolute_clip_rect_and_geometry` call
per invocation instead of 2-3, no other behavior change.

### Bench

New standalone bench, `bench/` (Cargo crate `focus_sweep_bench`, modeled on candidates 055/100's
own bench crates - reused their headless `MinimalSoftwareWindow` platform pattern). `count`
copies of a `HiddenChain` component (`TextInput` nested 15 levels deep inside `clip: true`
`Rectangle`s, with `visible: false` on the leaf itself - compiled by
`internal/compiler/passes/visible.rs` into a synthetic visibility-`Clip` ancestor directly above
it) sit before one always-visible, always-focusable `tail` `TextInput`. A single Tab press with
nothing focused makes `move_focus` call `is_visible_or_clipped_by_flickable()` on every one of the
`count` hidden leaves before landing on `tail` - exactly the "many Tab-navigable items each under
a deep clipping-ancestor chain" scenario the backlog entry asked for. Correctness assertion
embedded: `tail_has_focus` must be `true` after every odd-numbered Tab press (each even-numbered
press wraps focus back to nothing via `move_focus`'s cycle detection, then the next press starts
fresh again - both binaries must agree on this pattern).

Three scenarios were measured, matching the depth of the real effect at different points in the
patch stack (same structure as candidate 101's writeup, for the same reason - a real,
scale-independent per-call win here can be masked by other unlanded O(2^k)/O(n^2) costs on the
same call path):

**Scenario A - against today's real, unpatched tree** (`absolute_clip_rect_and_geometry` is still
O(2^k); `count` kept small since baseline cost is exponential in depth, not just this candidate's
own multiplier):

| count | pristine (2 runs, `taskset -c 0`) | this patch alone (2 runs, `taskset -c 0`) |
|---|---|---|
| 10 | 791.96ms / 797.50ms | 499.59ms / 500.80ms |

**Scenario B - on top of candidate 100 alone** (its unlanded O(2^k)->O(k) fix applied first;
candidate 055's O(n^2) `visited.contains` scan in `move_focus` is *not* applied, so it dominates
total cost at any count worth measuring and masks this candidate's effect):

| count | 100-only (pinned core 0) | 100 + this patch (pinned core 0) |
|---|---|---|
| 500 | 45.17ms / 44.36ms (min, 2 runs unpinned) | 43.96ms / 43.47ms (min, 2 runs unpinned) |
| 2 000 | 556.29ms (min, pinned) | 557.42ms (min, pinned) |

(500/2000 differences are inside noise - consistent with candidate 101's own finding that
candidate 055's still-unfixed O(n^2) scan swamps any O(n) or per-item saving at any count large
enough to matter.)

**Scenario C - on top of candidates 100 and 055 together** (isolates this candidate's own effect,
same technique candidate 101 used):

| count | baseline (100+055) min, pinned core 0 (2 runs) | patched (100+055+this) min, pinned core 0 (2 runs) | baseline min, unpinned | patched min, unpinned |
|---|---|---|---|---|
| 8 000 | 237.34ms / 233.69ms | 224.84ms / 219.89ms | 227.35ms | 216.95ms |
| 32 000 | 941.93ms / 940.60ms | 889.73ms / 888.10ms | 893.61ms | 858.70ms |

All `min` values were stable to within ~1% across repeats at a given count/pin combination
(`p50`/`mean` were noisier at times due to background load on this shared machine - `uptime`
showed load 4-9 on 24 cores throughout, `min` is the reported figure per MEASUREMENT.md's
noise-floor discipline). `taskset -c 3` was avoided per `projects/slint.md` section 9's own note
for this container (core 3 measured ~2.8-3x slower than core 0 for identical work); core 0 was
used for all pinned numbers here.

## Result

- **Scenario A** (today's real baseline, `count=10`, depth 15, O(2^15)-per-call regime): **~37%
  reduction** (795ms -> 500ms, both min-of-2-runs). This mostly reflects removing one of the two
  redundant *full* O(2^15) calls (the third call, on a one-level-shallower ancestor, is smaller in
  this regime but not negligible) - not the whole 2-3x a naive call-count argument would suggest,
  since the calls aren't equal-cost pre-patch.
- **Scenario B** (on top of candidate 100 alone): no measurable difference at 500 or 2 000 - the
  O(n) linear cost this patch removes per item is dwarfed by candidate 055's still-unfixed O(n^2)
  scan once `count` is large enough to see anything at all above noise.
- **Scenario C** (on top of both candidates 100 and 055, isolating this candidate's own O(k)
  constant-factor reduction): a small, real, reproducible **~5-6% reduction** at both 8 000
  (237ms/234ms -> 225ms/220ms) and 32 000 (942ms/941ms -> 890ms/888ms), consistent between pinned
  and unpinned runs. Not multi-core-scalable in the parallelism sense (this is single-threaded
  work triggered by one dispatched key event, so "unpinned" here means "not noise from core
  migration", not "runs on more than one core at once") - both pinned and unpinned readings agree
  to within run-to-run noise, matching candidate 100's own note about this code path.

## Does it change behavior/output?

- **Bit-exact?** Yes. The rewritten function returns the same value in every case: the
  "not-visible, check ancestors" branch is proven above to always evaluate to the same answer the
  original per-ancestor geometry check would have produced, and the structural walk still finds
  the same first clipping ancestor and does the same `Flickable` check on it. The bench's own
  correctness assertion (`tail_has_focus` toggling true/false in the expected pattern across
  Tab presses, at `count` up to 32 000, on all six built binaries - pristine, this-patch-alone,
  100-only, 100+this, 100+055-only, 100+055+this) held on every run.
- **Is this a breaking change?** No - private/internal function, no public API or file format
  touched.
- Project test suite: **307/307 passed, 0 failed**, both before and after
  (`cargo test -p i-slint-core --lib`), measured separately for: pristine tree, this patch alone,
  this patch on top of candidate 100, and this patch on top of candidates 100+055 - all four
  configurations passed the identical 307/307. (`cargo test -p i-slint-core --all-features` still
  fails to build for the unrelated, pre-existing `zeno`/no_std reason documented in
  `projects/slint.md` and in candidate 100's own `RESULT.md` - not specific to this patch.)

## Why the result looks the way it does

Rectangle intersection via repeated `min`/`max` is exactly associative and commutative in
IEEE-754 (no rounding is introduced by comparison-and-select, unlike a running sum), so folding an
intersection over a set of ancestors gives a bit-identical result regardless of the order the
ancestors are folded in. The remaining subtlety is `absolute_clip_rect_and_geometry`'s
`.unwrap_or_default()` on an empty intersection, which substitutes a degenerate zero-area rect at
the origin rather than propagating "no rectangle at all" - but a zero-area rect is `is_empty()`
regardless of position, and intersecting a zero-area rect with anything else can only ever produce
another zero-area (hence still-empty) rect or `None` (which again defaults to zero-area). So
"empty" is sticky under further intersection regardless of fold order, and "non-empty" can only
ever be the true, order-independent full intersection (since every partial fold is a superset of
the full one, and a superset of a non-empty set is only forced empty by the buggy substitution if
the intersection has already gone empty earlier, which is excluded by definition when the true
answer is non-empty). This is what makes the original loop's per-ancestor recomputation entirely
redundant: the first clipping ancestor's own "geometry intersected with everything above it" is
provably the same value as `self`'s already-computed `clip`, in every case, not just typical ones.

## What's not done / open questions

- Not measured with a `Flickable` ancestor actually present (the recursive case,
  `ancestor.is_visible_or_clipped_by_flickable()`). That recursion calls this same function again
  on a *different* item and is unaffected by this patch's own correctness argument (which is only
  about the non-recursive "return false" exit) - it was exercised indirectly by the existing
  `cargo test -p i-slint-core --lib` suite (nothing there specifically stresses nested
  Flickables inside deep clip chains together) but not by this candidate's own bench.
- The backlog's original premise (O(k^2) from repeatedly visiting many ancestors) does not hold
  for a single linear clipping-ancestor chain, per the mathematical argument above - it's possible
  a *branching* structural situation (multiple distinct clipping subtrees feeding into the same
  Flickable recursion chain) could still visit more than one ancestor's geometry per top-level
  call, but that would require actually reaching the `Flickable` recursion branch repeatedly
  across *different* Flickable ancestors stacked on top of each other, which is a distinct,
  narrower scenario from "k nested clip:true ancestors" and was not investigated separately here.
- Real hardware / GUI-visible confirmation not done - headless, single-event microbenchmark, same
  caveat as candidates 100/101 note about whether shipped `.slint` UIs commonly reach 15+ levels
  of nested clipping with many Tab-navigable hidden siblings.
