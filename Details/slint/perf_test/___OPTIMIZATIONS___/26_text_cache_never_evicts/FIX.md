# 26 (fix) - bounding the shaped-text cache

Candidate 26 measured the problem: `TextLayoutCache` entries are only ever added. An item laid out
once at startup keeps ~4.7 KB of shaped glyphs until its component is destroyed, so a
non-virtualized list of 60 000 rows holds hundreds of megabytes while ~25 rows are on screen.

## The change

`ItemCache` gains two methods (`for_each`, `retain`) and `TextLayoutCache` gains a clock, an entry
estimate and a sweep:

* every entry is stamped with a monotonic counter when it is served;
* once the estimate passes `ENTRY_LIMIT` (1024 entries, ~4.8 MB), the sweep drops everything older
  than the newest `ENTRY_LIMIT / 2` stamps.

Halving rather than trimming to the limit is what keeps it amortized: a sweep is O(entries) and the
next one is 512 insertions away.

**Evicting is safe by construction.** A missing entry is reshaped on next use, which is the same
path an invalidated entry already takes - `CachedParagraphs::paragraphs` is already an `Option` the
code handles being `None`. The sweep runs *before* the entry is checked out, because the guard holds
a `RefMut` into the cache for as long as the caller keeps the paragraphs.

## Result

Measured with `tools/isolate.py off 26-text-cache-sweep`, so the two binaries differ by this fix and
nothing else. Medians of `RUNS=5`, two passes.

| scene | frame, no sweep | frame, sweep | RSS, no sweep | RSS, sweep |
|---|---|---|---|---|
| `scroll` 2 000 | 3.07 ms | 3.01 ms | 48.1 MB | **30.7 MB** (-36%) |
| `scroll` 5 000 | 4.70 ms | 4.71-5.61 ms | 92.7 MB | **44.5 MB** (-52%) |
| `scroll` 10 000 | 8.96-10.60 ms | 8.63-9.61 ms | 167.9 MB | **68.5 MB** (-59%) |
| `scroll` 20 000 | 23.33 ms | 25.46 ms (+9%) | 316.9 MB | **115.9 MB** (-63%) |
| `scroll` 60 000 | 71.74 ms | 77.50 ms (+8%) | 917.4 MB | **304.5 MB** (-67%) |
| `list` 60 000 (ListView) | 2.39 ms | 2.24 ms | 31.4 MB | 30.8 MB |
| `text` 50 000 lines | 4.08 ms | 4.00 ms | 29.0 MB | 28.6 MB |

**Up to 613 MB freed.** The 5 000 and 10 000 rows are noisy enough (spreads of 1-5.7 ms) that the
frame column there says only "no measurable change"; the 20 000 and 60 000 figures were confirmed
over three interleaved passes each and are real.

### Where the +9% comes from, and why it is the right price

With the sweep the `scroll` scene shapes **18 paragraphs per frame** instead of 0. That is not a
regression peculiar to this change - it is exactly what `ListView` has always done
(candidate 02: *"18 shapes/frame = exactly the rows entering the viewport"*). Rows scrolled into view
were shaped once at startup and evicted long ago, so they are reshaped on arrival.

Raising `ENTRY_LIMIT` does **not** buy this back: scrolling forward always meets rows that were
shaped at startup, whatever the limit, so any bound below the row count behaves the same. The choice
is between an unbounded cache and paying what virtualization already pays.

### Two things that do not change

`ListView` is unaffected - it already only lays out visible rows, so its entry count never reaches
the limit. The `text` scene is unaffected too: one huge document is a single entry, served every
frame, so it is never the oldest.

## Verification

* `cargo test -p i-slint-core`: 304 + 69 + 1 pass, including 5 new tests for the eviction cutoff
  (empty cache, exactly at the limit, one past it, unsorted stamps, and the 180 000 -> 512 first
  sweep after a large list is laid out).
* `cargo check -p i-slint-core --features ffi`: clean (the ABI assertions only compile under it).
* Rendering unchanged on `scroll` (layout and bare rows), `list` and `text`, checked with
  `tools/pixelcheck.sh`, which discounts the harness's own non-reproducible status label:

```
### --mode text --count 20000       difference outside the noise floor: 0 px
### --mode list --count 20000       difference outside the noise floor: 0 px
### --mode scroll --count 20000     difference outside the noise floor: 0 px
```

## Reproduce

```sh
tools/isolate.py off 26-text-cache-sweep   # build -> /tmp/pt_nosweep
tools/isolate.py restore                   # build -> /tmp/pt_sweep

for b in nosweep sweep; do
  BIN=/tmp/pt_$b RUNS=5 tools/matrix.sh \
    "scroll 20000|--mode scroll --count 20000" \
    "scroll 60000|--mode scroll --count 60000" \
    "list 60000|--mode list --count 60000"
done

RUNS=3 tools/pixelcheck.sh /tmp/pt_nosweep /tmp/pt_sweep -- --mode scroll --count 20000
```
