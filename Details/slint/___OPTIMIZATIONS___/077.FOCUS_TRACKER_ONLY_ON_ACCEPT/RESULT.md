# 077 - `publish_focus_item` builds a visibility tracker for every rejected focus candidate

`Window::publish_focus_item` (`internal/core/window.rs:1461-1512`) unconditionally called
`track_focus_item_visibility(item)` - which allocates a `Vec` and walks the item's ancestor chain
via `visibility_clips()` (`internal/core/item_tree.rs:421`), then initializes a `ChangeTracker` -
*before* checking whether `item.focus_event(FocusIn)` returned `FocusAccepted`. During
`Window::move_focus`'s search loop (`window.rs:1514-1538`), every non-focusable item visited pays
this setup cost only to have it immediately discarded/overwritten by the next candidate. The fix
moves the `track_focus_item_visibility` call to run only inside the `result ==
FocusEventResult::FocusAccepted` branch, so the cost is paid once (for the item that actually
gets focus, or zero times if the whole walk fails) instead of once per rejected candidate.

## What was tried

Reused candidate `055.MOVE_FOCUS_CYCLE_DETECTION`'s bench (`focus_bench`) unchanged: a headless
`Window` with `count` plain `Rectangle`s and nothing focusable, so a single Tab press with
nothing focused walks the entire tree and fails, calling `publish_focus_item` (and, before this
patch, `track_focus_item_visibility`) once per item. `taskset -c 3`, single Tab-press-to-timeout,
15 iterations (first discarded as warmup), `rdebug` profile (release + full debug info, no debug
assertions). Two independent scenarios were measured because this candidate's effect is masked by
`055`'s still-unfixed O(n^2) cycle-detection scan in the current real source (055 was measured but
reverted, per this workspace's "always revert" rule - it is not part of today's baseline):

**A) This patch alone, against today's real baseline (055's O(n^2) scan still present, dominates
at scale):**

| Variant | count | single-core min (pinned, 2 runs) | multi-core min (unpinned, 2 runs) |
|---|---|---|---|
| baseline | 2 000 | 880.7us / 880.9us | 908.9us / 987.2us |
| baseline | 8 000 | 9.391ms / 9.383ms | 7.506ms / 7.523ms |
| baseline | 32 000 | 103.16ms / 103.42ms | 104.28ms / 106.05ms |
| baseline | 100 000 | 1007.1ms / 1006.2ms | 998.5ms / 999.3ms |
| patched (this candidate only) | 2 000 | 783.3us / 778.7us | 634.8us / 654.1us |
| patched (this candidate only) | 8 000 | 8.953ms / 8.947ms | 7.213ms / 7.366ms |
| patched (this candidate only) | 32 000 | 101.41ms / 101.76ms | 101.90ms / 101.85ms |
| patched (this candidate only) | 100 000 | 997.6ms / 999.1ms | 1005.4ms / 984.8ms (noisy, see below) |

**B) On top of candidate 055's (unlanded) O(n) fix, isolating the tracker-setup cost the way the
original finding describes it (the case this fix was actually motivated by):**

| Variant | count | single-core min (pinned, 2 runs) | multi-core min (unpinned) |
|---|---|---|---|
| 055-only baseline | 8 000 | 1.327ms / 1.275ms | - |
| 055-only baseline | 32 000 | 5.082ms / 5.151ms | - |
| 055-only baseline | 100 000 | 15.923ms / 16.043ms | - |
| 055 + this patch | 8 000 | 935.8us / 920.5us | - |
| 055 + this patch | 32 000 | 3.752ms / 3.753ms | - |
| 055 + this patch | 100 000 | 11.793ms / 11.786ms | 11.46ms, 16.7ms, 14.1ms (noisy, unpinned - background load on this shared machine, see below) |

Each pinned cell was measured twice to check the noise floor before comparing: all pairs agree to
within ~1-3%. Unpinned numbers were noisier during this session - a first unpinned baseline pass
was measured while `uptime` showed load average 6.39 (another process, likely a concurrent
candidate session on this shared machine, building in the background) and was visibly slower than
a same-config pinned run; it was re-measured once load dropped to ~1.8-2.8 and matched the pinned
numbers within a few percent (see "Why the result looks the way it does"). The 100k unpinned
number for scenario B fluctuated run-to-run (11.5/16.7/14.1ms) even after load appeared to drop,
consistent with this being a shared, noisy machine during this session - the pinned numbers are
the reliable ones here.

## Result

- **Scenario A (against today's real, unpatched baseline):** small but measurable and consistent
  win, shrinking as `count` grows: ~11% at 2 000 (880us -> 780us pinned), ~4.7% at 8 000, ~1.7% at
  32 000, ~0.1-0.9% at 100 000. This shrinking-with-scale pattern is expected: candidate 055's
  O(n^2) `visited.contains` scan is still present in real source and dominates total time at
  scale, so removing a genuinely O(n) cost (one tracker setup avoided per rejected item) is a
  shrinking fraction of an O(n^2) total as n grows.
- **Scenario B (on top of 055's fix, i.e. the scenario the original finding actually describes -
  "with that scan now linear, this becomes the new dominant per-step cost"):** a real, scale-
  independent ~25-29% reduction: 8 000 -> ~940us from ~1.30ms (-28%), 32 000 -> ~3.75ms from
  ~5.12ms (-27%), 100 000 -> ~11.8ms from ~15.98ms (-26%). This is the number that supports the
  original finding's claim, but it requires 055's change to be present to observe - 055 is not
  applied to real source today (per this workspace's "always revert" rule), so it is not what a
  reader gets by applying *only* this candidate's patch to the current tree.

## Does it change behavior/output?

- **Bit-exact?** Yes for everything that's user-observable: which item ends up focused (or that
  focus stays unset) is unchanged, verified two ways:
  - `focus_bench <count> <iters> --check`: a `FocusScope` placed after `count` plain rectangles
    still gains focus after exactly one Tab press. Passes before and after the patch, and in
    every one of the four build variants measured above (baseline, patch-only, 055-only,
    055+patch).
  - `cargo test -p i-slint-core --lib focus` (14 tests) and `cd tests && cargo test -p
    test-driver-interpreter focus` (37 tests): 0 failures before, 0 failures after, same 14+37
    tests, same as candidate 055's precedent.
  - One internal side effect changes, not externally observable through any tested path: when
    `move_focus`'s walk rejects every candidate and returns `None` (nothing focusable found), the
    window's `focus_item_visibility_tracker` (`window.rs`'s field, separate from `focus_item`
    itself) is left in whatever state it was in before the walk started (cleared, since
    `focus_next_item`/`focus_previous_item` both call `take_focus_item` - which clears the
    tracker - immediately before calling `move_focus`) instead of dangling on whichever rejected
    candidate happened to be visited last. `self.focus_item` itself (the field callers actually
    read) is unaffected by this patch either way - it was already being reassigned to each
    rejected candidate in both the old and new code, independent of this change.
- **Is this a breaking change?** No. Internal, private function (`fn publish_focus_item`, not
  `pub`); no public API, file format, or documented behavior involved.
- Project test suite: 14/14 `i-slint-core` focus/item-tree unit tests pass before and after;
  37/37 focus interpreter tests pass before and after. No new failures.

## Why the result looks the way it does

`track_focus_item_visibility` does two things per call: `item.visibility_clips()` walks the
item's ancestor chain allocating a `Vec` of any `Clip` ancestors found, and
`ChangeTracker::init` sets up a dependency-tracked closure. In this bench's flat scene (all
`Rectangle`s are direct children of the root, no nested `Clip`/`Flickable`), the ancestor walk is
shallow and the `Vec` stays empty, so the per-call cost is real but modest - which is exactly why
it is invisible against the O(n^2) `visited.contains` scan (still present in today's real source)
at any count worth measuring, but becomes a clear, count-independent ~25-29% cut once that O(n^2)
scan is fixed (scenario B). A deeper/nested scene (more `Clip` ancestors per item) would likely
widen the gap between scenario A and B further, but wasn't measured here (055's bench scene is
intentionally flat).

The unpinned noise is a genuine confound observed during this session, not a property of the
patch: this machine had `uptime` load average up to 6.39 at times (a concurrent process, likely
another parallel candidate/session sharing the machine), which measurably slowed an unpinned run
without affecting a `taskset`-pinned one measured moments apart - re-measuring at lower load
(1.8-2.8) brought unpinned numbers back in line with pinned ones. This is why the pinned numbers,
not the unpinned ones, are treated as authoritative in the "Result" section above.

## What's not done / open questions

- Candidate 055's own fix (O(n^2) -> O(n) cycle detection) is **not** applied to real source as
  part of this candidate - per this workspace's "one fix per candidate" rule, scenario B above is
  reported only as supplementary evidence that the two findings compound as the original backlog
  entry predicted, not as this candidate's own patch. `patch.diff` in this folder contains only
  this candidate's change (the `track_focus_item_visibility` move), diffed against the real,
  unpatched base commit.
- Not measured on a real GUI backend (winit/femtovg) with actual keyboard input, only via the
  headless software-renderer platform's `dispatch_event`, matching candidate 055's precedent -
  the code path inside `publish_focus_item` does not depend on which renderer/platform is in use.
- Not measured on a scene with nested `Clip`/`Flickable` ancestors, where `visibility_clips()`'s
  ancestor walk and its resulting non-empty `Vec` would cost more per call than in this bench's
  flat scene - the win measured here is likely a lower bound for such scenes.
