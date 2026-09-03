# 89 - `SortModel::row_changed`'s O(n) linear scan to find a changed row's sorted position

`SortModelInner::row_changed` (`internal/core/model/adapters.rs:687` at the time this was picked,
IDEAS.md's "models sweep" entry) locates a changed row's current position in `mapping` via
`mapping.iter().position(|r| *r == row).unwrap()` - an O(n) linear scan - on every single
`set_row_data` call on a sorted model, because `mapping` is sorted by *value*, not original row
index, so no binary search is possible without a reverse index.

## What was tried

Added a `reverse_mapping: RefCell<Vec<usize>>` field (`reverse_mapping[row]` = row's position
within `mapping`), maintained alongside `mapping`:
- `build_mapping_vec` and the bulk paths (`row_added`, `row_removed`) rebuild it fully from
  `mapping` after their own O(n)-or-worse work (a `rebuild_reverse_mapping` helper, O(n), same
  order as the sort/splice they already do).
- `row_changed` (the targeted hot path) looks up `reverse_mapping[row]` in O(1) instead of
  scanning, then after `mapping.remove`/`mapping.insert` fixes up only the `reverse_mapping`
  entries between the old and new position (bounded by the same span `Vec::remove`/`insert`
  already memmove, not a full rebuild).

No existing project bench reaches `SortModel::row_changed` in a way that isolates it from other
per-frame costs, so a standalone bench (`bench/`) drives it directly through the public API:
`VecModel::set_row_data(wrapped_row, value)` (on the model `SortModel::new_ascending` wraps) ->
`ModelNotify::row_changed` -> `SortModelInner::row_changed`. 200 000 scrambled `u64` rows, a fixed
deterministic sequence of 5000 updates (same seed used for baseline and patched so the resulting
model contents/checksum are directly comparable), only the update loop timed (mapping pre-built,
untimed, before the timer starts). Two edit-distance modes, since **how far a changed row moves in
`mapping` turned out to be decisive**:
- **`max_delta=0`** ("jump anywhere"): each update's new value is uniformly random across the
  whole value range (`0..4n`) - the worst case for how far a row travels in `mapping`.
- **`max_delta=D`** ("nudge"): each update perturbs the row's *current* value by at most `+/-D`,
  clamped to the valid range - the more realistic case for interactively editing a sorted list
  (bumping a score/counter by a little), keeping the average reorder distance small relative to
  `D`.

Single-threaded, pure CPU-bound `Rc`/`RefCell` code with no thread pool involved, so "multi-core"
here means "unpinned", not "uses multiple cores" - included per protocol.

Raw pinned runs (`taskset -c 3`, `uptime` load 4-7 on 24 cores throughout), `n=200000`,
`updates=5000`, p50 in ms, two repeats at the key `max_delta` values:

| max_delta | baseline p50 (run 1 / run 2) | patched p50 (run 1 / run 2) |
|---|---|---|
| 0 (jump anywhere) | 278.8 / 278.5 | 490.9 / 462.5 |
| 5 | 324.3 | 79.9 |
| 50 (nudge) | 338.5 / 322.4 | 81.9 / 88.2 |
| 500 | 323.2 | 86.9 |
| 2 000 | 336.0 | 90.5 |
| 10 000 | 330.9 | 103.7 |
| 50 000 | 330.7 | 186.5 |
| 100 000 | 313.6 | 268.1 |
| 150 000 | 315.6 | 333.6 |

Unpinned (2 runs each), `n=200000`, `updates=5000`:

| max_delta | baseline p50 (run 1 / run 2) | patched p50 (run 1 / run 2) |
|---|---|---|
| 0 (jump anywhere) | 273.5 / 278.2 | 471.8 / 443.8 |
| 50 (nudge) | 297.8 / ... | 82.0 / 82.0 |

## Result

This is a workload-dependent crossover, not a flat win or loss:

- **Small/medium reorder distance (`max_delta` up to ~50 000, i.e. up to ~25% of `n`, covering the
  "nudge a value" interactive-edit case the original finding described):** a real, reproducible
  win, from ~1.8x (`max_delta=50000`: 330.7ms -> 186.5ms) up to ~4x (`max_delta<=2000`: e.g.
  338.5ms -> 81.9ms), consistent across pinned/unpinned and repeat runs.
- **Large reorder distance (`max_delta` approaching the full `0..4n` value range, i.e. a row's new
  sort position is essentially uniformly random relative to its old one):** a real, reproducible
  **regression**, ~1.6-1.8x slower (278.8ms -> 490.9ms at `max_delta=0`), consistent across
  pinned/unpinned and repeat runs. The crossover for this bench's parameters is around
  `max_delta` ~100 000-150 000 (50-75% of `n`).

## Does it change behavior/output?

- **Bit-exact?** Yes. Every `RunResult.checksum` (a hash of the model's final sorted contents)
  matches exactly between baseline and patched builds, for every `max_delta` value tested and both
  seeds used - the notification sequence (`row_changed`/`row_removed`+`row_added` pairs and their
  indices) is unchanged; only the internal bookkeeping cost changed.
- **Is this a breaking change?** No - internal-only change to `SortModelInner`, no public API
  affected.
- Project test suite: `cargo test -p i-slint-core --lib` - **307 passed, 0 failed** before
  (baseline/master) and **307 passed, 0 failed** after (patched), including the 5
  `model::adapters::sort_tests::*` tests (`test_sorted_model_insert`, `test_sorted_model_remove`,
  `test_sorted_model_changed`, `test_sorted_model_source_model`, `test_sorted_broken_model`) run
  explicitly both before and after.

## Why the result looks the way it does

The original finding's premise - "an O(1) reverse-index lookup is strictly better than an O(n)
scan" - only accounts for the *lookup*, not what happens after. The old code's O(n)
`mapping.iter().position(...)` is a **sequential, branch-predictable scan** over a `Vec<usize>` -
exactly the access pattern CPUs auto-vectorize/prefetch well. The new code's O(1) lookup is real,
but `row_changed` still has to fix up `reverse_mapping` for every position between the row's old
and new spot so future lookups stay correct - and that fixup is `reverse_mapping[mapping[position]]
= position`, a **scatter write** indexed by `mapping[position]` (an essentially-random row id once
the model is sorted). When the reorder distance is large, this trades a cheap sequential scan for
an equally-large but cache-hostile scatter-write pass, which loses. When the reorder distance is
small (the common case this candidate's own justification described - editing one field of a
sorted row rarely moves it far in sort order), the fixup touches only a small span, and removing
the O(n) scan entirely wins outright. Both the "always wins" framing in IDEAS.md and this result
are compatible with the same underlying mechanism: the win/loss is about the fixup's *scatter
span*, not just the lookup's asymptotic class.

## What's not done / open questions

- The crossover point (~50-75% of `n`) was located by a bounded parameter sweep on one `n`
  (200 000); it was not characterized as a function of `n` itself (e.g. does the crossover scale
  with `n`, or stay a fixed fraction) - would need more sweep points across `n` to answer.
- A hybrid that only maintains `reverse_mapping`'s fixup lazily (e.g. falling back to a scan when
  the fixup span would exceed some threshold) was not attempted - it would need its own
  correctness/perf validation and adds real complexity for a workload-dependent win.
- No attempt was made to determine what reorder-distance distribution real Slint applications
  using `SortModel` + frequent `set_row_data` actually see; the "nudge a value" vs. "jump anywhere"
  framing here is a hypothesis about likely usage (leaderboards, live-updating counters), not a
  measurement of real app behavior.
