# adaptive_chunk_should_stop_exploration_overflow

`State::should_stop_exploration` (`src/runner/runner_variants/adaptive_chunk/state.rs:104-122`)
computes `total * EXPLORATION_CAP_PCT` (line 108) and `total * EXPLORATION_TARGET_PCT` (line 120)
as plain, non-saturating `usize` multiplications, inconsistent with the neighboring
`explored.saturating_mul(100)` two lines above. `total` is `self.initial_len`, the caller-supplied
iterator's exact size (from `ExactSizeIterator`/matching `size_hint` bounds), fixed once at the
start of a parallel operation. Candidate description flagged this as low-confidence/"likely
impractical to actually allocate", from `IDEAS.md`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Manual boundary-value construction through the crate's real public API: `(0usize..total).par().sum()` (a real `std::ops::Range`, the default `AdaptiveChunkRunner`), with `total = usize::MAX / 10 + 2` (just above the 64-bit overflow boundary for `total * 10`), `cargo run` (dev profile, debug-assertions on) | Panics immediately (well under 1 second) with `attempt to multiply with overflow` at `state.rs:108:58`, process exit code 101. A `Range<usize>` is a genuine `ExactSizeIterator` reporting a huge `size_hint` while consuming O(1) memory - no actual allocation of ~1.8e18 items is needed, contrary to the candidate description's stated impracticality concern. |
| 2. Repeat runs, identical build/input | 4/4 runs panic identically (`state.rs:108:58`, same message), deterministic - not layout/seed-dependent. Confirmed both with the original `total = 2_000_000_000_000_000_000` value and separately at the exact boundary `usize::MAX / 10 + 2` (see `repro/`). |
| 3. Release build (no debug-assertions), same input | No panic - `cargo run --release` runs for as long as observed (killed after 8s, still running) since the multiplication silently wraps and the (would-be astronomically long) actual summation proceeds instead of crashing. Confirms this is a debug-assertions-only manifestation, per `CONFIRMATION.md` section 2 - not less real for that, since a debug-assertions build is a normal, supported way to build/test this crate (`cargo test`/`cargo run` without `--release`). |

No escalation past a debug-assertions build + boundary-value construction was needed - approach 1
gave a clean, deterministic, on-demand reproduction on the first try (per `HEURISTICS.md`'s
"arithmetic on untrusted sizes" guidance), so the soft cap-of-3 in `SKILL.md`'s candidate loop was
not exercised.

## Status

**Confirmed.** Reproduces 4/4 runs, debug-assertions build (`cargo run`'s default dev profile,
equally reachable via `cargo test`), deterministic (pure integer-arithmetic overflow, not
layout/seed-dependent) - both at the originally-tried `total = 2e18` and independently at the exact
boundary `usize::MAX / 10 + 2`.

**Fully reproducible**: **Yes**. The repro drives the crate's real public entry point
(`Range<usize>::par().sum()`, the actual `AdaptiveChunkRunner` default runner) with a real
`std::ops::Range` - a genuine external input a real caller could construct (e.g. summing a
range-based computation), not a hand-built internal `State`. This is a single-participant finding
(one caller handing one iterator to one public API call, no peer/wire interaction to fake), so
`CONFIRMATION.md` section 11's two-participant bar does not apply.

## Class

crash

## Root cause

`should_stop_exploration` runs on every call to `complete_chunk` while in `Explore` mode (i.e.
after every processed chunk, starting with the very first one) and unconditionally computes
`total * EXPLORATION_CAP_PCT` (`EXPLORATION_CAP_PCT = 10`) and, further down,
`total * EXPLORATION_TARGET_PCT` (`= 2`) - both as plain `usize * usize` multiplication with no
overflow protection, unlike the `explored.saturating_mul(100)` two lines above it in the same
function. `total` comes from `self.initial_len`, computed once in `AdaptiveChunkRunner::new_state`
from the input iterator's `size_hint` when its lower and upper bounds match (i.e. any
`ExactSizeIterator`-shaped input, including a plain `Range<usize>`). For `total > usize::MAX / 10`
(64-bit: ~1.8e18; reachable purely via a lazy `Range` with no memory allocation, since a range's
size_hint doesn't require materializing elements), `total * 10` overflows. Under a
debug-assertions build this panics with `attempt to multiply with overflow` on the very first
`should_stop_exploration()` call (i.e. after the first, typically size-1, exploration chunk) -
well before any meaningful fraction of the input has actually been processed. Under a release
build, the multiplication silently wraps, producing a nonsensical (much smaller than intended)
threshold for the cap/target checks - a masked logic bug rather than a crash, but still a
correctness defect in the exploration-stopping heuristic for any such input.

The 64-bit threshold (~1.8e18 elements) is admittedly astronomical for real workloads, matching
the candidate description's own "low-confidence" framing - but this crate has genuine `wasm32`
target support (`Cargo.toml`'s `[target.'cfg(target_arch = "wasm32")'.dependencies]`, per this
branch's own name), where `usize` is 32 bits: the same overflow boundary there is only
`u32::MAX / 10 ≈ 429_496_729` items, which is a plausible size for a real range-based parallel
computation (e.g. `(0..600_000_000usize).par()...` on `wasm32`), not an impractical one - this
was reasoned from reading `Cargo.toml`/the wasm pool implementation, not independently
re-reproduced under an actual `wasm32` build in this session (no `wasm32` toolchain execution
environment - a JS engine driving the real `WasmWebPool`/Worker-based pool - was set up; see "What's
not done" below).

## Introduced by

1e87c645 "early exit" (2026-07-28) introduced the line-108 `total * EXPLORATION_CAP_PCT`
(the multiplication the observed panic actually hits first). The sibling line-120
`total * EXPLORATION_TARGET_PCT` was introduced by 70309ff6, also "early exit" (2026-07-28), a
few minutes earlier the same day. Both identified via `git blame` on the affected lines
(`src/runner/runner_variants/adaptive_chunk/state.rs:104-122`, formerly
`src/runner/runner_variants/run_b/state.rs` before a later rename/move).

## The fix

Changed both multiplications from plain `total * EXPLORATION_CAP_PCT` /
`total * EXPLORATION_TARGET_PCT` to `total.saturating_mul(EXPLORATION_CAP_PCT)` /
`total.saturating_mul(EXPLORATION_TARGET_PCT)`, matching the `saturating_mul` style already used
two lines above for `explored.saturating_mul(100)`. Saturating to `usize::MAX` on overflow means
the cap/target comparison (`explored_x100 >= total.saturating_mul(PCT)`) simply never fires via
this path for such an enormous `total` (correct: the real fraction-of-total threshold genuinely is
larger than any reachable `explored_x100` for a `total` this large) - exploration then falls
through to the other, already-overflow-safe stopping conditions (convergence-based stopping,
tiny-work early exit, wall-clock `EXPLORATION_MIN_MS`), rather than panicking or wrapping to a
bogus small threshold.

Verified against the primary repro (`total = usize::MAX / 10 + 2`, the exact boundary value) both
directly in the working tree and via `just test` from a fresh clone (`repro/` + `patch.diff`, see
`output.txt`): before the fix, `just reproduce-before` panics 101 within under a second, 4/4 direct
runs plus the `just test` run itself (5/5 total); after `just apply`, `just reproduce-after` shows
no panic within a 5-second timeout (3/3 direct runs plus the `just test` run, 4/4 total) - it
instead keeps running normally (as expected: correctly proceeding rather than crashing, since
actually summing ~1.8e18 items is intentionally not waited out to completion).

- Project test suite (`cargo test --lib`): 99 passed / 0 failed before the fix -> 99 passed / 0
  failed after (identical - this change is purely additive safety on an arithmetic edge case none
  of the existing tests exercise).

## What's not done / open questions

- The `wasm32` relevance noted under "Root cause" (32-bit `usize` lowering the overflow threshold
  to ~430M, a practically reachable range size) was reasoned from reading `Cargo.toml` and the
  wasm pool implementation, not independently reproduced by actually building/running against a
  real `wasm32-unknown-unknown` target in a JS engine this session (no C linker for a 32-bit native
  target was available either, which would have been a cheaper stand-in - see `README.md`'s
  tooling notes). A future session with a working wasm32 execution harness could confirm the same
  panic fires there at the lower threshold.
- No other code path in the crate was searched for the same "plain multiplication next to a
  sibling `saturating_mul`" pattern; `should_stop_exploration` is the only place using
  `EXPLORATION_CAP_PCT`/`EXPLORATION_TARGET_PCT`, so there is no sibling location for this specific
  constant pair, but the general pattern (inconsistent overflow-checking within the same function)
  was not swept for elsewhere in this session.
