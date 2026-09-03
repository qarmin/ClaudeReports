# 104 - Checking `list`/`scroll` mode's `ScrollBar` for candidate 099's real-clock jitter, under candidate 102's fix

This is an **investigation candidate, not a performance candidate**: no product source was
changed (`patch.diff` is empty). It answers the `IDEAS.md` entry left open by candidates 099/102:
099 root-caused a real-wall-clock-driven `animate width, height { duration: 150ms }` transition on
the Fluent `std-widgets` `ScrollBar` thumb as the source of run-to-run pixel jitter in `--bench
--mode text --type-text`, once a scene's per-frame cost drops below 150ms; 102 made `BenchPlatform`'s
clock deterministic and verified the fix only on that `text` scenario. `list` (`ListView`, which
`internal/compiler/widgets/common/listview.slint` implements as `ListView inherits ScrollView`) and
`scroll` (a plain `for` directly inside `ScrollView`) both drive the exact same `ScrollBar` component
and were never checked.

## What was tried

Reproduced the same "content size changes every frame" trigger condition 099 identified for
`--type-text`, using `list`/`scroll` mode's own equivalent: `--mutate <n> --mutate-mode push`, which
appends rows every frame (growing the `ScrollView`'s content height, and therefore the vertical
`ScrollBar` thumb's animated `height`, every frame) instead of `--type-text`'s per-frame document
growth. `--count 500 --mutate 20 --mutate-mode push --scroll-step 0 --frames 60`: per-frame cost is
~85-115us (`--bench`'s own reported spread), four orders of magnitude below the thumb's 150ms
transition duration - comfortably in the regime 099 showed makes the transition sampled mid-flight
almost every frame, i.e. a scenario that would have shown the pre-102 real-clock jitter if it were
present here the same way it was in `text` mode. All comparisons below are 15-20 repeated runs of
the identical binary/flags/`--snapshot`, compared pixel-by-pixel against run 1 (no diffing tool
needed at this scale - a small Python script, not saved, summed differing bytes across all repeats).

| Mode | Clock (build variant) | Core affinity | Runs differing / total | Where |
|---|---|---|---|---|
| `list` | virtual (102's fix, current permanent state) | unpinned | 0/19 | - |
| `list` | real (102's fix reverted) | unpinned | 0/14 | - |
| `scroll` | virtual (102's fix) | unpinned | 17/19 (session 1), 2/14 (session 2) | rows 118-121, cols 1266-1267 (vertical `ScrollBar` thumb, top of track) |
| `scroll` | real clock (102's fix reverted) | unpinned | 1/14 | same rows/cols |
| `scroll` | virtual (102's fix) | pinned (`taskset -c 3`, 10 runs; `-c 5`, 15 runs) | 0/10, 0/15 | - |
| `scroll` | virtual (102's fix) | unpinned, `--mutate-mode rotate` (row count constant, no content growth) | 0/15 | - |

## Result

**`list` mode's `ScrollBar` is bit-exact** at this scenario, both with and without candidate 102's
fix, unpinned - no pre-102-style jitter shows up here at all, at counts/frame combinations fast
enough to have shown it in `text` mode.

**`scroll` mode's `ScrollBar` is *not* reliably bit-exact**, but the cause is not candidate 099/102's
mechanism: reverting 102's fix (restoring the real system clock) does **not** make the jitter
meaningfully worse or more frequent (1/14 differing runs, vs. 17/19 and 2/14 in two sessions with
the fix applied - same order of magnitude, same exact rows/columns, not a clean "fixed vs. broken"
split). If 099/102's mechanism were the cause here, the virtual-clock build should have been
reliably 0/N the way `text` mode was (candidate 102's own table: 0/20 with the fix vs. 16/20
without, on the `text` scenario). It is also not attributable to the `on_add_records` status label
(`perf_test/src/logic.rs`, embeds real elapsed fill time) - candidate 097's `app.set_status(Default::default())`
blanking already covers every mode unconditionally (`bench.rs:533`, outside the `match options.mode`
block), confirmed by finding no differing pixels anywhere near the status label's row range in any
of the comparisons above.

Two things do reliably make `scroll` mode's residual disappear:
1. **Pinning the process to a single core** (`taskset -c N`, tried on two different cores):
   0 differing pixels across 25 runs total, vs. non-zero when unpinned in every unpinned sample
   taken.
2. **Holding the row count constant per frame** (`--mutate-mode rotate`, `remove(0)` + `push`, same
   net content size every frame): 0/15 differing, vs. non-zero with `--mutate-mode push` (net growth)
   at the same `--mutate 20`, same frame count, same core-affinity conditions (both unpinned).

Point 2 confirms the *trigger* is the same as 099's family (a per-frame-changing target for the
thumb's animated `width`/`height`), but point 1's clean pinned/unpinned split, combined with the
"real clock vs. fixed clock" comparison **not** cleanly separating the two build variants,
indicates the remaining mechanism for `scroll` mode is a **second, unidentified,
core-scheduling-correlated source** - not eliminated by candidate 102's clock fix, and not the same
real-wall-clock-sampling mechanism 099 diagnosed and 102 fixed. This is the same shape of finding
candidate 102 itself already reported for `--mode rects --animate` (also "a second, unidentified
non-determinism source... survives candidate 102's deterministic-clock fix" - already in
`IDEAS.md`) - `scroll` mode's growing-content `ScrollBar` case looks like a second, independent
sighting of a similarly-shaped problem, not necessarily the identical bug, but not chased down to a
common root cause in this candidate.

## Does it change behavior/output?

- **Bit-exact?** N/A to the product - no tracked Slint source was changed (`patch.diff` empty).
  This candidate is purely a correctness/determinism check of `perf_test`'s own `--bench` harness
  (already-permanent code from candidate 102), not a change to it.
- **Is this a breaking change?** No - no product or harness source changed by this candidate.
- Project test suite (`i-slint-core`, unaffected since no source changed by this candidate):
  `cargo test -p i-slint-core --all-features --lib`: **319 passed, 0 failed**, run once against the
  pristine tree (no tracked-source diff to test "before" vs. "after" against, matching candidates
  099/102's own convention for a patch-less investigation).

## Why the result looks the way it does

`list` mode's `ListView` virtualizes: only the handful of rows actually inside the viewport are
laid out and drawn, so a mutation appended far outside the visible window (or even a total
content-height estimate change) has a smaller and differently-timed effect on what gets recomputed
per frame than `scroll` mode's plain unvirtualized `for`, which lays out and draws all `--count`
rows every frame regardless of what's visible. Whatever the second, unidentified mechanism
underlying the `scroll`/`rects+animate` cases turns out to be, it plausibly needs the larger,
non-virtualized per-frame workload to manifest at a visible probability in a short (15-20 run)
sample - consistent with `list` mode showing 0/33 differing runs total across both clock variants
tested here, while `scroll` mode showed a nonzero (if inconsistent, 1/14 to 17/19) rate in every
unpinned sample taken.

No other real-clock read exists on the pixel-producing path that could explain `scroll`'s residual
directly: grepping `internal/core/`, `internal/renderers/`, and `perf_test/src/` for
`Instant::now()`/`SystemTime::now()` finds only benchmark-timing/logging call sites (already
verified not to feed back into any rendered property - e.g. `perf_test/src/logic.rs`'s
`on_add_records` timing text, which the existing `app.set_status(Default::default())` call already
blanks for every mode) and the two used by the (now-deterministic) `BenchPlatform`/`AnimationDriver`
clock path itself. This rules out "another, uncovered real-clock read" as the explanation and points
instead at something core-scheduling-correlated but not clock-value-correlated (e.g. an
allocator/thread-timing-dependent floating summation order, or a race with some other genuinely
concurrent piece of the runtime) - not identified further here, per the same "second unidentified
source" caveat candidate 102 already recorded for a different scene.

## What's not done / open questions

- **The `scroll`-mode second mechanism itself was not identified**, only isolated to: requires
  per-frame content-size growth (not just any per-frame mutation), requires the process to be
  unpinned (any single-core pinning tested removes it), and is not attributable to the real vs.
  virtual clock choice, the status label, or `list` mode's own geometry. Whether this is the same
  underlying bug as `--mode rects --animate`'s already-flagged "second unidentified source" (candidate
  102) or a coincidentally similarly-shaped, unrelated one was not determined - a plausible next
  step would be checking whether `--mode rects --animate`'s residual also disappears under
  `taskset`, which would be strong evidence for a shared root cause.
- **Not swept**: `--count`/`--mutate`/`--frames` values other than the one combination used here,
  so the exact probability of hitting the residual (varied 1/14 to 17/19 across the handful of
  samples taken) is not characterized as a function of scene size - only shown to be nonzero and
  reproducible at this one combination.
- **`list` mode was only checked at one scenario** (`--count 500 --mutate 20 --mutate-mode push
  --frames 60`, matching `scroll`'s own test for a fair comparison) - a larger, fully unvirtualized
  effective workload (e.g. a much bigger `--count`, or forcing more of the list's rows to be
  simultaneously visible) was not tried, so "list is unaffected" is confirmed at this scale, not
  proven impossible at every scale.
- Added to `IDEAS.md`: chase the `scroll`-mode / `--mode rects --animate` second unidentified
  source(s) down together, starting with the `taskset` cross-check above.
