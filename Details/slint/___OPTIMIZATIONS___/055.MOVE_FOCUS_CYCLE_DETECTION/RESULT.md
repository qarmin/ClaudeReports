# 055 - O(N^2) cycle detection in `Window::move_focus`

`Window::move_focus` (`internal/core/window.rs:1514-1541`) walks the focus chain one item at a
time via a caller-supplied `forward` closure (`next_focus_item`/`previous_focus_item`), pushing
every visited item into a `Vec<ItemRc>` and calling `visited.contains(&current_item)` after each
step to detect that the walk has cycled back on itself. `next_focus_item`/`previous_focus_item`
are exact inverses of each other (verified in `internal/core/item_focus.rs` and its own
`test_focus_chain_*`/`test_tree_traversal_*` unit tests), so `forward` is a bijection over every
item in the tree: the walk from `start_item` can only ever cycle back to `start_item` itself, not
to some other previously-visited item first. The `Vec::contains` scan is therefore an unnecessary
O(n) check on every one of the up to n steps (O(n^2) total) where a single `current_item ==
start_item` comparison (O(1) per step, O(n) total) is sufficient.

## What was tried

perf_test's own scenes could not exercise the worst case: `perf_test/ui/main.slint`'s chrome
always wraps every mode in a row of `std-widgets` `Button`s, which accept keyboard focus within
~8 traversal steps regardless of scene size - the huge item arrays every scene is built around are
never even reached. A minimal standalone bench (`bench/`, `focus_bench`) was written instead: a
`Window` with `count` plain `Rectangle`s and nothing else, so a Tab press with nothing focused has
to walk the entire tree and fail before giving up - the "100k items, no focusable items" case from
the original finding.

Measured with `taskset -c N`, single Tab-press-to-timeout, 15 iterations (first discarded as
warmup), `rdebug` profile (release + full debug info, no debug assertions):

| count | baseline min (pinned) | patched min (pinned) | baseline min (unpinned) | patched min (unpinned) | speedup (pinned) |
|---|---|---|---|---|---|
| 2 000 | 878.7 us / 883.9 us (2 runs) | 383.7 us / 384.3 us (2 runs) | - | - | 2.3x |
| 8 000 | 9.367 ms / 9.373 ms (2 runs) | 1.549 ms / 1.549 ms (2 runs) | 9.377 ms | 1.543 ms | 6.0x |
| 32 000 | 102.7 ms / 103.7 ms (2 runs) | 5.091 ms / 5.121 ms (2 runs) | 102.1 ms | 5.271 ms | 20.0x |
| 100 000 | 1013.9 ms / 1014.4 ms (2 runs) | 16.16 ms / 16.08 ms (2 runs) | 1007.8 ms | 16.15 ms | 62.8x |

Each baseline and patched cell was measured twice (both single-core pinned via `taskset -c 3`)
to establish the noise floor before comparing across variants - both repeats agree to within
~1% at every count, so the noise floor is small relative to the measured deltas. Unpinned
(multi-core, realistic wall-clock) numbers match the pinned ones closely, as expected: this is a
single-threaded synchronous call (`Window::dispatch_event` -> `move_focus`), so core count does
not change its cost.

Scaling shape confirms the complexity-class change directly:
- Baseline (`git stash`/reverted patch): doubling `count` roughly **triples** the time
  (1000->2000->4000->...->32000 min times: 312.7us, 880.0us, 2.780ms, 9.328ms, 27.15ms,
  103.2ms - ratios per doubling: 2.8x, 3.2x, 3.4x, 2.9x, 3.8x), consistent with the claimed O(n^2).
- Patched: doubling `count` gives almost exactly **2x** (185.5us, 368.8us, 747.7us, 1.489ms,
  2.987ms, 5.929ms - ratios per doubling: 2.0x, 2.0x, 2.0x, 2.0x, 2.0x), consistent with O(n).

## Result

Real, large, and growing with N as predicted by the complexity-class change: from ~2.3x at
count=2000 to ~63x at count=100000. At 100k items with nothing focusable, a single Tab press
drops from ~1.01 seconds to ~16 ms - the "multi-second Tab-press stall" the original finding
described is confirmed in kind (order of magnitude away from multi-second at 100k on this
machine, but the O(n^2) scaling means a larger tree, or a slower machine, reaches it directly).

## Does it change behavior/output?

- **Bit-exact?** Yes for every case that matters: which item ends up focused (or that focus stays
  unset) is unchanged. Verified two ways:
  - A dedicated correctness check in the bench (`focus_bench <count> <iters> --check`): a
    `FocusScope` placed after `count` plain rectangles must still gain focus after exactly one Tab
    press. Passes both before and after the patch, at count=2000.
  - The project's own interpreter focus test suite (`tests/cases/focus/*.slint`, 37 tests via
    `cargo test -p test-driver-interpreter focus` from `tests/`) and `i-slint-core`'s own
    `item_focus`/`item_tree` unit tests (14 tests, `cargo test -p i-slint-core --lib focus`) all
    pass unchanged with the patch applied - 0 failures before, 0 failures after, same 37+14 tests.
- **Is this a breaking change?** No. Internal, private function (`fn move_focus`, not `pub`); no
  public API, file format, or documented behavior involved.
- Project test suite: 37/37 focus interpreter tests pass before and after; 14/14 `i-slint-core`
  focus/item-tree unit tests pass before and after. No new failures.

## Why the result looks the way it does

`next_focus_item`/`previous_focus_item` are built as exact inverses specifically so that Tab and
Shift+Tab retrace each other's steps - which is exactly the invariant that makes the full-history
scan redundant: a bijection's forward-iteration from any starting point traces out precisely the
cycle containing that point, with no "tail" that could make it revisit an *other* previously-seen
item before returning to the start (unlike a general, non-injective function, where a Floyd/Brent-
style cycle detector genuinely would be needed). The `Vec` + `contains` was doing strictly more
work than the problem requires.

## What's not done / open questions

- The neighboring finding from the same IDEAS.md row group - `publish_focus_item`
  (`window.rs:1461-1512`) paying for a full visibility-tracker setup on every rejected candidate,
  not just the accepted one - was not touched here; it's a separate, independent change (moving
  `track_focus_item_visibility` to after `FocusAccepted` is confirmed) and per this workspace's
  "one fix per candidate" rule belongs in its own candidate.
- Not measured on a real GUI backend (winit/femtovg) with actual keyboard input, only via the
  headless software-renderer platform's `dispatch_event` - the code path inside
  `Window::move_focus` itself does not depend on which renderer/platform is in use, so this should
  not matter, but wasn't independently confirmed with a real event loop.
- `perf_test`'s own scenes were not modified to exercise this case (adding then reverting a
  `--focus-tab` flag during investigation showed the app's toolbar buttons always intercept focus
  early); if a permanent `perf_test` mode for "no focusable items" is wanted later, it would need
  its own chrome-free scene, not a flag on an existing one.
