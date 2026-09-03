# 101 - `track_focus_item_visibility` skips `ChangeTracker::init` when there are no visibility clips

`WindowInner::track_focus_item_visibility` (`internal/core/window.rs`, called from
`publish_focus_item` on every focus change) always called `self.focus_item_visibility_tracker
.init(...)`, even when `item.visibility_clips()` returns an empty `Vec` - the common case for a
focused item with no `Clip`/`Flickable` ancestor. When the `Vec` is empty, the tracker's own eval
closure is `[].iter().all(...)` == always `true`, so the tracker's notify callback could never
fire; `init` (heap-allocate a `BindingHolder`, run the eval closure once, store the result) was
pure overhead in that case. The fix: when `visibility_clips()` is empty, call
`self.focus_item_visibility_tracker.clear()` instead of `.init(...)`.

## What was tried

Reused candidate `055.MOVE_FOCUS_CYCLE_DETECTION`'s bench (`focus_bench`) unchanged: a headless
`Window` with `count` plain `Rectangle`s and nothing focusable (flat scene, no `Clip` ancestor
anywhere, so `visibility_clips()` is always empty), so a single Tab press with nothing focused
walks the entire tree and fails, calling `track_focus_item_visibility` once per visited item
(current real source calls it unconditionally before checking `FocusAccepted`, same as candidate
077 found - 077's own fix is not applied to real source, per the "always revert" rule). `rdebug`
profile, 15 iterations per point (first discarded as warmup), 2 repeats per configuration to
check the noise floor before comparing.

Two environment quirks specific to this container session (documented in
`projects/slint.md` section 9 for future sessions) affected how this was measured:
`bench/target/` is not where the binary actually lands while building from inside this checkout
(the repo's own `.cargo/config.toml` redirects it to `<repo_root>/target`), and `taskset -c 3`
was ~2.8-3x slower than `taskset -c 0` for identical work in this session - all pinned numbers
below use core 0 after confirming it agrees with the unpinned numbers.

Because the effect of this patch alone is masked by candidate 055's still-unfixed O(n^2)
`visited.contains` scan in `move_focus` (dominates total time at any count worth measuring), two
scenarios were measured, the same split candidate 077 used:

**A) This patch alone, against today's real, unpatched baseline:**

| Variant | count | single-core min (pinned core 0, 2 runs) | multi-core min (unpinned, 2 runs) |
|---|---|---|---|
| baseline | 2 000 | 721us / 721us | 747us / 744us |
| baseline | 8 000 | 7.72ms / 7.72ms (also see below) | 7.92ms / 7.84ms |
| baseline | 32 000 | 106.6ms / 106.6ms | 106.6ms / 107.1ms |
| baseline | 100 000 | 1.025s / 1.032s | 1.038s / 1.044s |
| patched (this candidate only) | 2 000 | 692us / 700us | (not separately re-run; scenario B below covers count independence) |
| patched (this candidate only) | 8 000 | 7.54ms / 7.64ms | |
| patched (this candidate only) | 32 000 | 104.6ms / 106.4ms | |
| patched (this candidate only) | 100 000 | 1.045s / 1.032s | |

(An earlier baseline pass pinned to core 3 by mistake showed 299-321ms at 32 000 and 3.0-3.2s at
100 000 - the core-3 slowdown described above, not a property of the code. Discarded once
identified; not used in the comparison.)

**B) On top of candidate 055's (unlanded) O(n) fix, isolating this candidate's own cost the way
the original finding describes it:**

| Variant | count | single-core min (pinned core 0, 2 runs) | multi-core min (unpinned, 2 runs) |
|---|---|---|---|
| 055-only baseline | 8 000 | 1.273ms / 1.287ms | 1.327ms / 1.310ms |
| 055-only baseline | 32 000 | 5.173ms / 5.180ms | 5.198ms / 5.382ms |
| 055-only baseline | 100 000 | 16.206ms / 16.230ms | 16.811ms / 16.587ms |
| 055 + this patch | 8 000 | 1.120ms / 1.120ms | 1.138ms / 1.136ms |
| 055 + this patch | 32 000 | 4.518ms / 4.514ms | 4.622ms / 4.625ms |
| 055 + this patch | 100 000 | 14.088ms / 14.857ms | 15.834ms / 15.721ms |

Each cell's two numbers are independent repeat runs, agreeing to within a few percent (except
100 000's mean/p95, which was noisier run-to-run - the `min` values quoted are stable).

## Result

- **Scenario A (against today's real, unpatched baseline):** a small, roughly consistent
  reduction at the low end (~4% at 2 000: 721us -> ~696us) shrinking to noise-floor-sized at
  larger counts (32 000/100 000: differences of a few tenths of a percent, inside the ~1-3%
  pinned-repeat noise floor observed here). Same shrinking-with-scale pattern candidate 077 found
  for its own change: candidate 055's O(n^2) scan dominates total time at scale, so a genuinely
  O(n) saving (avoiding one `ChangeTracker::init`'s allocation+eval per rejected item) is a
  shrinking fraction of an O(n^2) total as `count` grows.
- **Scenario B (on top of 055's fix, isolating this candidate's own effect):** a real,
  scale-independent **~12-13% reduction**: 8 000 -> ~1.12ms from ~1.28ms (single-core, -12.5%;
  multi-core -13.6%), 32 000 -> ~4.52ms from ~5.18ms (-12.7% single-core, -12.5% multi-core),
  100 000 -> ~14.1-14.9ms from ~16.2-16.3ms (single-core, roughly -10 to -13% depending on which
  repeat is compared; multi-core ~15.7-15.8ms from ~16.6-16.8ms, -5 to -8%). Smaller than
  candidate 077's own ~25-29% isolated win (which removes the entire `track_focus_item_visibility`
  call for a rejected candidate) because this patch only removes the `ChangeTracker::init`
  allocation+eval - the `item.visibility_clips()` ancestor walk itself (needed either way, to
  learn the `Vec` is empty) still runs every time.

## Does it change behavior/output?

- **Bit-exact?** Yes for everything user-observable: which item ends up focused (or that focus
  stays unset) is unchanged.
  - `focus_bench <count> <iters> --check`: a `FocusScope` placed after `count` plain rectangles
    still gains focus after exactly one Tab press. Passes before and after the patch, and in all
    four build variants measured above (baseline, patch-only, 055-only, 055+patch).
  - The one internal state difference: for an item with no `Clip`/`Flickable` ancestor,
    `self.focus_item_visibility_tracker` is left cleared (`inner` pointer null) instead of
    holding an initialized-but-permanently-`true` tracker whose notify callback could never fire.
    No observable difference - the old tracker's eval closure (`[].iter().all(...)`) is always
    `true` regardless of any property changing, so `run_change_handlers` would never have called
    its notify function either way.
- **Is this a breaking change?** No. Internal, private function (`fn
  track_focus_item_visibility`, not `pub`); no public API, file format, or documented behavior
  involved.
- Project test suite, patched vs. baseline (real, unpatched `internal/core/window.rs`), same
  counts both times:
  - `cargo test -p i-slint-core --lib`: 307/307 before, 307/307 after.
  - `cargo test -p i-slint-core --lib focus`: 14/14 before, 14/14 after (`item_focus::tests::*`,
    `item_tree::tests::test_tree_traversal_*_focus`).
  - `cd tests && cargo test -p test-driver-interpreter focus`: 37/37 before, 37/37 after -
    includes `test_interpreter_focus_7058_scrolled_clip`, which exercises the non-empty-clips
    path (untouched by this patch) alongside the flat-scene cases the empty-clips path now
    short-circuits.

## Why the result looks the way it does

`ChangeTracker::init` (`internal/core/properties/change_tracker.rs`) does real work even for a
trivial closure: it calls `self.clear()` (drop any previous tracker), builds a
`ChangeTrackerInner` holding the data tuple and both closures, heap-allocates it via
`Box::into_raw`, then immediately runs `eval_fn` once (through `current_binding_storage::set`,
which sets up the property-dependency-tracking machinery) and stores the result. For the flat
scene this bench uses, `visibility_clips()` returns an empty `Vec` every time, so `eval_fn`'s
`[].iter().all(...)` reads no properties and always returns `true` - all of that setup exists
only to store a value that can never change. Skipping straight to `clear()` (which just checks
whether the previous tracker's pointer is null and, if not, drops it - the same work `init` would
have done as its own first step) removes the allocation and the eval call entirely, at the cost
of one `Vec::is_empty()` check that was already implicit in the `Vec` this function already had
to build to make that decision either way.

The gap between Scenario A (near-zero at scale) and Scenario B (~12-13% at every count) is the
same shape candidate 077 documented for its own related finding: this cost is real and
count-independent, but it is dominated by candidate 055's still-unfixed O(n^2) scan in every
scene large enough to matter, so it's invisible against today's real baseline and only shows up
once that scan is fixed.

## What's not done / open questions

- Candidate 055's own fix (O(n^2) -> O(n) cycle detection) is **not** applied to real source as
  part of this candidate - per this workspace's "one fix per candidate" rule, `patch.diff` in
  this folder contains only this candidate's change, diffed against the real, unpatched base
  commit. Scenario B above is reported only as supplementary evidence isolating this candidate's
  own cost, matching candidate 077's precedent.
- Candidate 077's own fix (`track_focus_item_visibility` only called for the accepted item, not
  every rejected candidate) is also not applied here, for the same reason - stacking either
  077 or 055 on top of this patch was not attempted as a combined candidate.
- Not measured on a scene with any `Clip`/`Flickable` ancestor (the non-empty-clips path is
  completely untouched by this patch, so no different behavior is expected there - the
  interpreter test suite's `test_interpreter_focus_7058_scrolled_clip` exercises that path and
  passed unchanged).
- Not measured on a real GUI backend (winit/femtovg) with actual keyboard input, only via the
  headless software-renderer platform's `dispatch_event`, matching candidates 055/077's
  precedent - the code path inside `track_focus_item_visibility` does not depend on which
  renderer/platform is in use.
