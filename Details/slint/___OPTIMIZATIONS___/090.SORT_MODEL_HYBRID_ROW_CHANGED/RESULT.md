# 90 - Hybrid `SortModelInner::row_changed`: adaptive reverse-index vs. linear scan

Candidate 089 added a `reverse_mapping: RefCell<Vec<usize>>` to `SortModelInner`
(`internal/core/model/adapters.rs`) to turn `row_changed`'s O(n) `mapping.iter().position(...)`
scan into an O(1) lookup, but found it's a real win only when the changed row's reorder distance
in `mapping` is small, and a real regression when the distance is large (the fixup that keeps
`reverse_mapping` correct is a cache-hostile scatter write that loses to the old scan's cheap
sequential one). This candidate (IDEAS.md's top backlog entry, sourced from 089) makes the choice
size-adaptive: keep `reverse_mapping` valid and use it (O(1) lookup + a bounded fixup) as long as
each `row_changed`'s fixup span stays under half of `n`; the moment a span exceeds that, mark
`reverse_mapping` invalid and fall back to the plain O(n) scan for `removed_index` on every
subsequent call, until a bulk operation (`row_added`/`row_removed`/`build_mapping_vec`, which
already rebuild it fully as part of their own O(n)-or-worse work) re-validates it.

## What was tried

Added a `reverse_mapping_valid: Cell<bool>` alongside 089's `reverse_mapping` field. In
`row_changed`: look up `removed_index` via `reverse_mapping[row]` in O(1) only while valid,
otherwise scan (089's original fallback, restored). After computing `insertion_index`, compute
`span = insertion_index.abs_diff(removed_index)`; if `span * 2 <= mapping.len()`, do the bounded
fixup over `[lo, hi]` (089's mechanism, unchanged) and keep `reverse_mapping` valid; otherwise skip
the fixup entirely and set `reverse_mapping_valid = false`, so no scatter-write pass is ever paid
for a large-span change - the next call (whether or not it turns out to be large again) simply
scans instead. `row_added`/`row_removed`/`build_mapping_vec` rebuild `reverse_mapping` fully and
set `reverse_mapping_valid = true`, exactly as in 089.

Reused 089's standalone bench (`bench/`) unmodified: `VecModel::set_row_data` on a
`SortModel::new_ascending`-wrapped model, 200 000 scrambled `u64` rows, 5000 updates per run, same
two deterministic edit-distance modes (`max_delta=0` "jump anywhere" vs. `max_delta=D` "nudge by at
most D"). Built against the real working-tree `internal/core` directly (not a clone) for this
session's measurements, via a scratch `tmp/bench_sort_hybrid/` copy with its `Cargo.toml`
`path = "../clone/internal/core"` pointed at the real checkout instead - the candidate folder's own
`bench/Cargo.toml` keeps the clone-relative path for standalone reproduction via `justfile`.

This session's host was measurably noisier than 089's (`uptime` load average 7-10 throughout on 24
cores, vs. 089's 4-7), which inflated pinned-run absolute numbers well above 089's original
baseline (e.g. this session's `max_delta=50` baseline: ~590-1105ms p50 vs. 089's ~322-338ms) and
made a couple of individual pinned data points disagree between repeats (noted below) - unpinned
runs were consistently much cleaner and are the more trustworthy signal this session.

Pinned (`taskset -c 3`), `n=200000`, `updates=5000`, p50 in ms, two runs at the key points:

| max_delta | baseline p50 (run1/run2) | patched p50 (run1/run2) |
|---|---|---|
| 0 (jump anywhere) | 685.2 / 664.7 | 659.5 / 554.7 |
| 5 | 748.4 | 190.7 |
| 50 (nudge) | 1105.6 / 590.1 | 190.4 / 161.6 |
| 500 | 558.3 | 126.1 |
| 2 000 | 525.4 | 117.8 |
| 10 000 | 520.5 | 214.2 |
| 50 000 | 599.3 | 345.2 |
| 100 000 | 323.9 / 550.0 | 273.6 / 482.4 |
| 150 000 | 637.7 / 542.5 | 764.5 / 538.5 |

Unpinned, `n=200000`, `updates=5000`, two runs at max_delta 0/50, one run at 100000/150000:

| max_delta | baseline p50 (run1/run2) | patched p50 (run1/run2) |
|---|---|---|
| 0 (jump anywhere) | 248.6 / 248.4 | 242.7 / 247.0 |
| 50 (nudge) | 271.0 / 271.7 | 80.6 / 74.3 |
| 100 000 | 276.1 | 249.9 |
| 150 000 | 274.9 | 271.5 |

## Result

- **Small/medium reorder distance (`max_delta` up to ~10 000, i.e. up to ~5% of `n`):** the same
  strong win 089 found for this regime is preserved, ~2.4x-5.8x pinned, ~3.4x-3.7x unpinned
  (`max_delta=50`: 271.0/271.7ms -> 80.6/74.3ms unpinned).
- **089's worst case (`max_delta=0`, "jump anywhere"):** the ~1.6-1.8x regression 089 measured here
  is gone. Unpinned, patched is a hair faster than baseline both runs (242.7 vs 248.6, 247.0 vs
  248.4 - inside noise, i.e. a wash, not a loss). Pinned shows patched faster in both runs too
  (659.5 vs 685.2, 554.7 vs 664.7), though the pinned absolute numbers for both baseline and
  patched are inflated and noisy this session (see host-noise note above) - the unpinned numbers
  are the more trustworthy read here, and both say "no longer a regression."
- **The 50-100% of `n` region (`max_delta` 50 000-150 000) where 089 found its regression zone:**
  no longer a clear loss anywhere in this sweep. `max_delta=50000`: still a clear win (599.3ms ->
  345.2ms pinned). `max_delta=100000`: a small, consistent win, both pinned repeats and the
  unpinned run (e.g. unpinned 276.1ms -> 249.9ms, ~10%). `max_delta=150000`: the two pinned repeats
  disagree with each other (run1: patched 20% slower, 637.7 -> 764.5; run2: essentially tied,
  542.5 vs 538.5) while the single unpinned run is a wash (274.9 vs 271.5) - given the noted host
  noise, this is read as "no longer a clear regression," not confirmed to be a clean win either.

Net: the adaptive threshold removes 089's clear worst-case regression (`max_delta=0`, and
apparently the broader 50-100% zone) at every point actually measured, while keeping the small/
medium-distance win essentially intact, at the cost of the added state (`reverse_mapping_valid`)
and its own branch per call.

## Does it change behavior/output?

- **Bit-exact?** Yes. Every `RunResult.checksum` (hash of the model's final sorted contents)
  matches exactly between baseline and patched builds, for every `max_delta` value tested - the
  notification sequence is unchanged; only internal bookkeeping cost and its adaptive on/off
  switching changed.
- **Is this a breaking change?** No - internal-only change to `SortModelInner`, no public API
  affected.
- Project test suite: `cargo test -p i-slint-core --lib` - **307 passed, 0 failed** before
  (baseline/master) and **307 passed, 0 failed** after (patched, this candidate's hybrid), run
  fresh in this session both times. Also ran `cargo test -p i-slint-core --lib
  model::adapters::sort_tests` explicitly against the patched build: all 5 tests
  (`test_sorted_model_insert`, `test_sorted_model_remove`, `test_sorted_model_changed`,
  `test_sorted_model_source_model`, `test_sorted_broken_model`) passed.

## Why the result looks the way it does

The mechanism is exactly the one 089's own "what's not done" section proposed: the expensive part
of 089's fix was never the O(1) lookup itself, it was the fixup loop that keeps `reverse_mapping`
correct for *every* position between old and new spot, which becomes an increasingly large,
cache-hostile scatter write as the span grows. By computing the span first (both `removed_index`,
from the still-valid reverse index, and `insertion_index`, from the pre-existing `partition_point`
call, are already cheap) and simply declining to pay for the fixup once the span crosses half of
`n`, the large-span case degrades gracefully to "acts like the original O(n)-scan code" instead of
"pays for an O(span) scatter write nobody benefits from." The `n/2` threshold was chosen directly
from 089's own observed crossover range (50-75% of `n`) as a conservative middle point, not
independently tuned in this session - see "what's not done" below.

## What's not done / open questions

- The `span * 2 <= mapping.len()` threshold was not swept - 089's crossover was measured to be
  somewhere in 50-75% of `n` for its bench parameters, and `n/2` was picked as a conservative
  single guess inside that range, not confirmed to be optimal. A threshold sweep (e.g. `n/4`,
  `n/3`, `2n/3`) was not attempted.
- Once `reverse_mapping` is invalidated by one large-span change, it stays invalid (falling back
  to the O(n) scan) until the next bulk `row_added`/`row_removed`/full rebuild - there is no path
  in `row_changed` itself to cheaply re-validate it after a run of small-span changes following a
  single large one. Whether real usage patterns hit this "stuck in scan mode" case often enough to
  matter is unknown; a possible follow-up is an explicit periodic or count-triggered full rebuild.
- This session's host was noticeably noisier than 089's own session throughout (`uptime` load
  7-10 vs. 089's 4-7), which is visible directly in this candidate's own pinned numbers (e.g. the
  two `max_delta=150000` pinned repeats disagreeing with each other by ~20%) - the crossover-region
  numbers here should be treated as "no longer a clear loss" rather than a tightly-bounded measured
  win, unlike the small/medium-distance case where the effect size is large enough to be visible
  through the noise regardless.
- As with 089: no measurement of what reorder-distance distribution real Slint applications using
  `SortModel` + frequent `set_row_data` actually see.
