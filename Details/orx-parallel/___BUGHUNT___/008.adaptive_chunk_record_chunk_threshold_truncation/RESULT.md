# adaptive_chunk_record_chunk_threshold_truncation

`State::record_chunk` (`src/runner/runner_variants/adaptive_chunk/state.rs:90-99`) computes
`threshold = prev_avg / CONVERGENCE_STABILITY_DIVISOR` (integer `u64` division, divisor 50) to
decide whether the EWMA per-item-time average has "converged" (stopped changing much). For any
`prev_avg < 50` (i.e. per-item work below ~50ns), this truncates to `0`, and since `change` is an
unsigned `u64`, `change < 0` can never be true - so `converged_samples` can never be incremented,
no matter how stable the measured average actually is. Source:
`___FINDINGS___/findings.toml` (overflow_underflow, easy), via `IDEAS.md`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Manual boundary-value construction: a `#[cfg(test)]` module added directly inside `state.rs` (the affected fields/methods are `pub(super)`, unreachable from any public API or an external `tests/` integration test) that builds real `ChunkState`s with a controlled elapsed time (via `std::thread::sleep` over a 10,000,000-item chunk, so measurement noise is negligible relative to the sleep duration) and calls the real `State::record_chunk`/`State::new` repeatedly with identical (zero-change) per-item costs of 20ns/item, `cargo test --lib`, debug-assertions test profile | `avg_ns_per_item` converged to 20 (as expected), but `converged_samples` stayed at `0` after 10 identical calls (`CONVERGENCE_THRESHOLD` is 5) - confirming the truncation. A control test at 200ns/item (same identical-sample setup, above the 50ns boundary) reached `converged_samples=9` within the same 10 calls, showing the convergence-tracking logic itself works correctly outside the affected range. |
| 2. Repeat run, identical build/inputs | Same result both times (2/2): sub-50ns/item case stuck at `converged_samples=0`, above-50ns/item control reaches the threshold. Deterministic - this is a pure integer-division truncation, not layout/seed-dependent, so no further repeat-run count was needed beyond the standard bar. |
| 3. Boundary-value sweep (ad hoc, not part of the committed repro): 1ns/item and 49ns/item (both `< 50`) also reproduce the stuck-at-0 behavior before the fix and converge correctly after it; a genuinely-fluctuating low-average workload (alternating 10ns/30ns per item, ~100% swings) correctly does NOT spuriously "converge" after the fix, showing `.max(1)` doesn't just paper over the one boundary value that was tested first. See `output.txt`. | Confirms the fix holds across the whole affected range (`prev_avg` in `1..49`), not just the one value (20ns) used in the primary repro. | 

## Status

**Confirmed.** Reproduces 2/2 runs, debug-assertions test build (`cargo test`'s default test
profile), deterministic (not layout/seed-dependent - the mechanism is a pure integer-division
truncation on values derived from a real, if noisy, wall-clock sleep, and held across every repeat
run and every boundary value tried: 1, 20, 49ns/item).

**Fully reproducible**: **No**, with a stated reason. `State`'s relevant fields
(`avg_ns_per_item`, `converged_samples`) and methods (`record_chunk`, `should_stop_exploration`)
are `pub(super)` - there is no public API, diagnostics hook, or externally observable side effect
that exposes `converged_samples` or lets a caller distinguish "stopped exploring via
convergence" from "stopped exploring via some other condition". Worse: reading
`should_stop_exploration`'s full logic (see "What's not done" below) shows that for this exact
value range (`prev_avg < 50`, which is below `TINY_WORK_THRESHOLD_NS = 500`), the independent
"tiny work" early-exit (`avg_ns > 0 && avg_ns < TINY_WORK_THRESHOLD_NS && explored >=
TINY_WORK_MIN_SAMPLES`) already stops exploration at the same point in practice (both checks only
run once `explored >= min_samples`, which is always `>= 128 > TINY_WORK_MIN_SAMPLES = 64`) - so
even a full end-to-end run through the crate's public API (`Runner::adaptive()`,
`.chunk_size`/timing behavior) would show **no observable difference** in exploration length or
chosen chunk size between buggy and fixed code for this input class, because a second, unrelated
stopping condition already covers the same domain. Building a "through the public API, observe
externally" repro was therefore not just impractical (no observability path exists) but would not
even demonstrate a symptom, so a direct internal-state `#[cfg(test)]` call (as the candidate
description anticipated) is what was built instead.

## Class

logic

## Root cause

`record_chunk`'s convergence check (`state.rs:90-99`) computes
`threshold = prev_avg / CONVERGENCE_STABILITY_DIVISOR` with `CONVERGENCE_STABILITY_DIVISOR = 50`
(`constants.rs:42`, documented as "the EWMA average must change by less than 1/50 (2%) between
updates to count as stable"). For any `prev_avg` in `1..49`, integer division truncates this to
`0`. `change` (`updated_avg.abs_diff(prev_avg)`) is a `u64`, so `change < 0` is always `false` -
`converged_samples` is reset to `0` (never incremented) on every call while `prev_avg` stays below
50, regardless of how stable the actual measurements are (verified: even *identical*, zero-change
samples never increment it in this range). Convergence-based early stopping
(`should_stop_exploration`'s `converged >= CONVERGENCE_THRESHOLD` check) can therefore never fire
for average per-item costs below 50ns.

As documented under "Fully reproducible" above, this specific consequence is masked in the actual
runner by `TINY_WORK_THRESHOLD_NS = 500` already forcing an equivalent early exit for the same
input class, so the practical effect on `AdaptiveChunkRunner`'s behavior is believed to be nil for
today's constants - but the convergence-tracking mechanism itself is genuinely broken for
`prev_avg < 50` independent of that overlap, and would matter for any future change that narrows
or removes the tiny-work early exit, or lowers `TINY_WORK_THRESHOLD_NS` below 50, without revisiting
this line.

## Introduced by

562ef363 "revised state" (2026-07-27)

## The fix

Changed `state.rs:93` from:
```rust
let threshold = prev_avg / CONVERGENCE_STABILITY_DIVISOR;
```
to:
```rust
let threshold = (prev_avg / CONVERGENCE_STABILITY_DIVISOR).max(1);
```
This guarantees a minimum granularity of 1ns for the stability comparison, so `change == 0`
(a perfectly stable/identical average) always counts as stable regardless of how small `prev_avg`
is, while a genuine fluctuation still correctly fails to converge (verified: an alternating
10ns/30ns-per-item workload, ~100% swings on a low average, does not spuriously converge after the
fix - see "What was tried" #3 and `output.txt`).

Verified against the primary repro (20ns/item, and separately 1ns/item and 49ns/item boundary
values) both directly in the working tree and via `just test` from a fresh clone
(`repro/add_repro_test.diff` then `patch.diff`, see `output.txt`).

- Project test suite: `cargo test --lib`: 100 passed / 1 failed (this candidate's own added repro
  test) before the fix -> 101 passed / 0 failed after. `cargo test` (doctests): 260 passed / 0
  failed / 16 ignored, unaffected by this one-line change (ran only after the fix; the pristine
  lib-test run above already establishes the 99 pre-existing lib tests are unaffected either way).

## What's not done / open questions

- As noted above, this specific bug's downstream effect on `should_stop_exploration`'s actual
  return value is believed to be fully masked by the independent `TINY_WORK_THRESHOLD_NS` (500)
  early-exit for every value in the affected range (`prev_avg` in `1..49`) - both checks are gated
  by the same `explored >= min_samples` (>= 128) precondition, and `min_samples` always exceeds
  `TINY_WORK_MIN_SAMPLES` (64) structurally (`min_samples = max(128, 8 * num_threads)`). This was
  reasoned from reading `should_stop_exploration`'s full logic, not independently verified with a
  second, separate repro exercising that function end-to-end; a future session could build one
  (drive `AdaptiveChunkRunner` through its public API with a real sub-50ns-per-item closure and
  compare total explored-item counts before/after this fix) to confirm the masking claim, or to
  find a boundary condition where it doesn't hold (e.g. if `TINY_WORK_MIN_SAMPLES`/`min_samples`'s
  relationship changes in a future refactor).
- The neighboring, unrelated `overflow_underflow` lead from the same `findings.toml` source
  (`should_stop_exploration`'s `total * EXPLORATION_CAP_PCT` / `total * EXPLORATION_TARGET_PCT`
  non-saturating multiplication, noted in `IDEAS.md`) was not investigated as part of this
  candidate.
- No other code path in the crate performs this same "divide-by-constant-then-compare-unsigned"
  pattern was searched for; this is the only `record_chunk`/convergence-tracking implementation in
  the crate (the `adaptive_chunk` runner variant is the only one with this kind of exploration
  state), so there's no sibling location to check for the same bug class.
