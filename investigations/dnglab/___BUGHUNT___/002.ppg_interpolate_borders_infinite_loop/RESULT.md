# PPG demosaic: infinite loop in `interpolate_borders` for 4-5px-wide images

Noticed while boundary-testing candidate 001's fix (`saturating_sub` guard on `.take(h - 6)`/
`.take(w - 6)` in the same file) - varying `width` across the boundary (per `CONFIRMATION.md`
section 7: "vary the input within the same bug class before calling a fix complete") turned up a
second, independent, unrelated-mechanism bug in the very first pass of the same demosaic pipeline,
`interpolate_borders`, which runs *before* the functions candidate 001's patch touches.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Boundary-value sweep of `(width, height)` pairs while re-verifying candidate 001's fix | debug build, hand-built DNGs via `DngWriter`, decoded via `raw_to_srgb` | `(width=4, height=8)` and `(width=5, height=8)` hang indefinitely instead of panicking or returning |
| Code reading of `interpolate_borders`'s "jump to right border" logic (`ppg.rs:77-83`) | - | Found the jump can move `col` *backward*, re-triggering itself forever, for `width` in `{4, 5}` |
| Standalone repro with a background thread + 10s watchdog, hand-built spec-valid 4x8 uncompressed RGGB DNG, decoded via the real public `raw_to_srgb` (same fn `dnglab analyze --srgb` uses) | debug build | 3/3 runs: watchdog fires, "HANG DETECTED", process would otherwise spin forever (killed by the watchdog's own `process::exit`) |
| Same repro against the patched tree (this candidate's `patch.diff` only, not candidate 001's) | debug build | 3/3 runs (only the first is kept as `output.txt`, remaining two visually confirmed identical during this session): completes in ~200ms - no hang. It then hits the separate, already-documented candidate 001 bug and panics, because `width=4` is *also* candidate 001's trigger range and that patch was intentionally not applied for this isolated verification - see "The fix" |
| Revert-and-recheck (isolate the specific fixing hunk, per `SKILL.md` pitfall #6) | debug build | Reverting this patch alone (candidate 001's patch not present either) makes the hang come back immediately, 1/1 confirmatory run - rules out a confounded fix |
| `cargo test -p rawler --lib`, this patch applied in isolation (candidate 001's patch NOT applied) | debug build, 66 tests | 66 passed / 0 failed |

## Status

**Confirmed.** Reproduces 3/3 runs, deterministic (not layout/seed-dependent - the loop condition
is pure integer arithmetic on `width`), debug build. The hang is not gated by
`overflow-checks`/build profile at all (it's a plain `while` loop that never reaches its exit
condition, not an arithmetic panic) - it reproduces identically in a `--release` build too (not
separately re-run this session, but the loop logic has no debug-only instrumentation in it, so
there is no reason to expect a build-dependent difference; noted here rather than asserted as
independently observed, per `CONFIRMATION.md` section 2's spirit).

**Fully reproducible**: **Yes**. `repro/` builds a genuinely tiny-but-valid DNG file on disk with
rawler's own public `DngWriter`, then decodes it through the real, public, unmodified
`rawler::analyze::raw_to_srgb` entry point (the exact function `dnglab analyze --srgb <file>`
calls) on a background thread with a bounded watchdog, so the repro itself terminates predictably
either way. Single-participant case (a file handed to a parser's real entry point) - no second
participant to fake, per `CONFIRMATION.md` section 11.

## Class

`crash` (in the DoS/hang sense - an unbounded CPU-spinning infinite loop, not a panic or memory
unsafety).

## Root cause

`interpolate_borders` (`ppg.rs:71-108`) processes the outer 3px border of the image explicitly,
one column at a time, and is meant to jump straight from column 3 to column `w - 3` in the middle
rows (skipping the fully-interior columns, which the other three passes handle):

```rust
let mut col = 0;
while col < w {
  if col == 3 && row >= 3 && row < h - 3 {
    col = w - 3 // go to right border
  }
  ...
  col += 1;
}
```

This assumes `w - 3 >= col` (3) at the moment of the jump, i.e. `w >= 6`, so that the jump always
moves `col` *forward*. For `w` in `{4, 5}`, `w - 3` is `1` or `2` - **less** than the current `col`
(3). The jump moves `col` backward to 1 or 2; the loop then increments back up through 3 again,
re-triggers the same jump, and repeats forever for every row where `row >= 3 && row < h - 3` (i.e.
whenever `height >= 7`, so that range is non-empty). This is a pure infinite loop, not an
arithmetic overflow - it reproduces identically regardless of debug-assertions/overflow-checks
being on or off, unlike candidate 001's bug in the same file.

Note the overlap with candidate 001's trigger condition: both bugs require an image narrower than
6px, because `interpolate_borders` runs first in the pipeline (`demosaic()` at `ppg.rs:52`, before
`interpolate_green` etc. at `ppg.rs:53-55`). Before this fix, a `width` in `{4, 5}` never reaches
candidate 001's code at all (it hangs here first); after this fix alone, execution reaches
candidate 001's code and panics there instead, unless candidate 001's patch is also applied - see
`output.txt`'s "AFTER" section for exactly that sequence.

## The fix

Added `&& w > 6` to the jump condition (`ppg.rs:81`), so the jump only fires when it can move `col`
forward (`w - 3 > 3` reduces to `w > 6`; for `w == 6` the jump would be a no-op regardless, since
`w - 3 == col == 3`, matching the pre-existing behavior exactly). For `w` in `{4, 5}`, the jump no
longer fires; `col` increments through every column of the image normally and the loop terminates
after `w` iterations per row, same as the already-correct `w <= 3` case.

Verified: `repro/` (4x8) no longer hangs (isolated re-verification: this patch alone, candidate
001's patch not applied - 3/3 runs, ~200ms each, see "What was tried"). Re-verified the fix wasn't
confounded with something else changing at the same time by reverting just this hunk and
confirming the hang returns (see "What was tried").

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`, this patch applied in isolation). No new failures; no existing test exercises PPG
  demosaic at this width, so this fix is additive coverage, not a regression risk to existing
  behavior.

## What's not done / open questions

- Not independently re-verified in a `--release` build this session (see Status) - low risk given
  the mechanism has no debug-only gating, but not directly observed.
- This candidate's fix and candidate 001's fix are independent (different functions, different bug
  mechanisms) but share the same "image narrower than 6px" input space - a maintainer applying
  only one of the two patches will still see the other bug on the same class of input. Both should
  land together for `width < 6` images to be fully handled by PPG demosaic.
