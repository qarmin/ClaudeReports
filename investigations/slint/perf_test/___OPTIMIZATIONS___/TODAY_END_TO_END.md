# End-to-end effect of this session, measured against "before today"

`before` is this tree with **`internal/core/textlayout`, `properties.rs`, `change_tracker.rs` and
`model/adapters.rs` reverted** - i.e. everything applied in this session (candidates 03, 14, 19, 23,
29) removed, and everything from earlier sessions (06, 08, 09, 10, 11) left in place. Both binaries
built in the same invocation; medians of 9 runs (7 for the heavy `scroll` scenes), spreads given.

| scene | before | after | | RSS before | RSS after |
| --- | --- | --- | --- | --- | --- |
| `text` 100 000 | 3.68 ms (±0.07) | 4.04 ms (±0.16) | +10% | 543.8 MB | **38.9 MB (-93%)** |
| `text` 100 000 `--word-wrap` | 3.67 ms (±0.06) | 3.75 ms (±0.16) | - | 544.5 MB | 545.7 MB |
| `list` 100 000 | 2.22 ms (±0.12) | 2.35 ms (±0.10) | - | 39.8 MB | 37.0 MB |
| `rects` 3 000 | 2.64 ms (±0.23) | 2.55 ms (±0.25) | - | 22.9 MB | 20.6 MB |
| `rects` 3 000 `--gradient` | 3.50 ms (±0.32) | 3.34 ms (±0.43) | - | 23.9 MB | 20.5 MB |
| `scroll` 100 000 | 163.25 ms (±2.94) | **116.92 ms (±5.45)** | **-28%** | 1592.1 MB | 1506.1 MB |
| `scroll` 100 000 `flat` | 118.84 ms (±10.90) | 103.80 ms (±5.12) | -13% | 322.0 MB | 314.5 MB |
| `scroll` 100 000 `bare` | 30.79 ms (±4.03) | **20.03 ms (±1.47)** | **-35%** | 138.1 MB | 135.5 MB |

Plus, not visible in a steady-state table:

- **`text` resize**: 76 ms -> 5.6 ms per frame (candidate 03's width-independence).
- **`text` selection**: +1.09 ms -> +0.43 ms per frame at 400 000 lines (candidate 19).
- **A 5 000-character line no longer aborts the process** (candidate 23).
- **Sorted append**: 200 rows into 200 000 went +6.82 ms -> +1.54 ms (candidate 29).

## A correction worth recording

A first pass at this table used 5 runs and reported **`list` +21%**, which read as a real
regression from the session's work. Bisecting it (each change reverted separately) showed each
change alone was neutral and only the combination looked slow - an "interaction" that would have
been an interesting finding if it were true.

At 9 runs the effect is 2.22 -> 2.35 ms against a baseline spread of ±0.12, and the 100 000-row
baseline had measured ±0.64 on an earlier pass. **It is noise.** The bisect was the right instinct
and the wrong conclusion; what settled it was simply running the comparison more times.

The `text` +10% (3.68 -> 4.04, spreads ±0.07 and ±0.16) *does* survive - it is the 13 paragraphs
re-shaped per scrolling frame that candidate 03 trades for 505 MB, and it is the one cost in this
table that is real and intended.

## Reproduce

```sh
# build the baseline (see tools/gpu_before_after.sh for the mtime trap this must avoid)
git checkout -- internal/core/textlayout internal/core/properties.rs \
    internal/core/properties/change_tracker.rs internal/core/model/adapters.rs
# ... build, save the binary, restore, rebuild, then:
RUNS=9 tools/matrix.sh "text|--mode text --count 100000" "scroll|--mode scroll --count 100000"
```
