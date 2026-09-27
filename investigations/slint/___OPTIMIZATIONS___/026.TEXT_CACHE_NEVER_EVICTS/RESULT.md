# 26 - The shaped-text cache never evicts: 4.68 KB per `Text` item, forever

Ported from `perf_test/___OPTIMIZATIONS___/26_text_cache_never_evicts/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Candidate 003 fixed one `TextInput` holding a whole document. This is the other shape of the same
problem: **many small `Text` items**, each holding its own shaped layout, with nothing ever
releasing them.

## Measured

`--mode scroll`, layout rows (3 `Text`s each) against `--row-style bare` (none), resident memory
after 10 frames:

| rows | `Text` items | RSS, layout rows | RSS, bare rows | retained per `Text` |
| --- | --- | --- | --- | --- |
| 2 000 | 6 000 | 44.7 MB | 17.3 MB | **4.68 KB** |
| 5 000 | 15 000 | 89.7 MB | 21.1 MB | **4.68 KB** |
| 10 000 | 30 000 | 164.4 MB | 27.2 MB | **4.68 KB** |
| 20 000 | 60 000 | 313.4 MB | 39.2 MB | **4.68 KB** |

Exactly linear, to two decimal places, across a 10x range. 60 000 short `Text` items retain
**274 MB** of shaped glyphs while ~25 of them are on screen.

Rows whose children are positioned explicitly (`--row-style flat`) retain **0.63 KB** per `Text`
instead: nothing asks for their layout-info, so nothing is shaped (candidate 013). The 4.68 KB is
the `parley::Layout` for a ~30 character string.

## Cause

`TextLayoutCache` is an `ItemCache<CachedParagraphs>` - a `HashMap` per component of a `HashMap`
per item index. Searching `internal/core/item_rendering.rs` for `evict`, `LRU` or `capacity`
returns **nothing**. An entry is released only when:

- the component is destroyed (`component_destroyed`),
- the window scale factor changes (`clear_cache_if_scale_factor_changed`),
- something calls `clear_all`,
- or the entry is invalidated by a property change - which *re-shapes* it rather than freeing it.

Nothing releases an entry because it has not been drawn for a while. A `Text` item that was laid
out once at startup keeps its glyphs for the lifetime of the component.

## Why candidate 003 does not help here

That candidate's lazy shaping is gated on documents of more than 128 paragraphs
(`RETAINED_PARAGRAPHS`), because below that the glyphs are cheap to keep and keeping them avoids
ever re-shaping. A one-line `Text` is one paragraph, so it takes the eager path by design. The
gate is right for a single item and wrong in aggregate: 60 000 items each individually "cheap to
keep" is 274 MB.

## Fix direction (as scoped in this candidate)

The retention machinery candidate 003 built is per-*paragraph* inside one entry. What is missing is
the same idea one level up: a bound on the *cache*, evicting whole entries for items that have not
been drawn recently. The draw path already knows which items it visited, and
`CachedParagraphs::paragraphs` is already an `Option` that the code handles being `None` - an
evicted entry re-shapes on next use exactly like an invalidated one does today, so the correctness
surface is one that already exists.

The number to pick the bound with: 4.68 KB per entry, and ~25-50 entries are on screen in a list.

## What was actually built (per the original session's FIX.md, copied verbatim alongside this file)

The original session went on to implement this fix direction: `ItemCache` gained `for_each` and
`retain` methods, and `TextLayoutCache` gained a monotonic clock, an entry-count estimate and a
sweep that stamps each entry when served and, once the estimate passes `ENTRY_LIMIT` (1024 entries,
~4.8 MB), drops everything older than the newest `ENTRY_LIMIT / 2` stamps. `FIX.md` records this in
full, including the measured results:

| scene | frame, no sweep | frame, sweep | RSS, no sweep | RSS, sweep |
|---|---|---|---|---|
| `scroll` 2 000 | 3.07 ms | 3.01 ms | 48.1 MB | **30.7 MB** (-36%) |
| `scroll` 5 000 | 4.70 ms | 4.71-5.61 ms | 92.7 MB | **44.5 MB** (-52%) |
| `scroll` 10 000 | 8.96-10.60 ms | 8.63-9.61 ms | 167.9 MB | **68.5 MB** (-59%) |
| `scroll` 20 000 | 23.33 ms | 25.46 ms (+9%) | 316.9 MB | **115.9 MB** (-63%) |
| `scroll` 60 000 | 71.74 ms | 77.50 ms (+8%) | 917.4 MB | **304.5 MB** (-67%) |
| `list` 60 000 (ListView) | 2.39 ms | 2.24 ms | 31.4 MB | 30.8 MB |
| `text` 50 000 lines | 4.08 ms | 4.00 ms | 29.0 MB | 28.6 MB |

Up to 613 MB freed, with a measured +8-9% frame-time cost on the largest non-virtualized scenes
(paying for reshaping rows that scroll into view after their entry was evicted - the same shaping
work `ListView` has always done for rows entering the viewport). `ListView` and the single-document
`text` scene are unaffected. `FIX.md` also records verification: `cargo test -p i-slint-core`
(304 + 69 + 1 passing, including 5 new eviction-cutoff tests), a clean `cargo check -p i-slint-core
--features ffi`, and `tools/pixelcheck.sh` reporting 0 px difference outside the noise floor on
`text`, `list` and `scroll`.

## Reproduce

```sh
for c in 2000 5000 10000 20000; do
  tools/matrix.sh \
    "$c layout|--mode scroll --count $c" \
    "$c bare|--mode scroll --count $c --row-style bare"
done
```

## Patch re-derived and verified (2026-08-25)

The original session's patch was never saved (see "Does it change behavior/output?" below) - a
fresh implementation of the same fix direction was written independently against this workspace's
pinned base commit, following `FIX.md`'s description (`ItemCache` gains `for_each`/`retain`;
`TextLayoutCache` gains a monotonic clock, an entry-count estimate, and a sweep at `ENTRY_LIMIT =
1024` that drops everything older than the newest half), and is now `patch.diff` in this folder,
with a working `justfile`. Verified end to end via `just test`: `cargo test -p i-slint-core --lib`
307/307 both before and after, `cargo check --features ffi` clean, and a real
`--mode scroll --count 60000 --row-style layout --frames 10` run reproduces the shape of the
original finding - RSS 969.3MB (unbounded) -> 373.5MB (bounded), **-61.5%** (original: 917.4 ->
304.5MB, -67% - close but not identical, consistent with this being an independently-written
implementation of the same idea rather than the original bytes) - with later frames ~9.4% slower
patched (102.97ms -> 112.63ms median), matching the originally-described "+8-9%, paying for
reshaping rows that scroll into view after eviction" tradeoff. Pixel/hash comparison was not
re-run this pass (the RSS/frame-time numbers above are what was checked); the 5 "eviction-cutoff"
unit tests `FIX.md` describes were **not** added to `i-slint-core` this time - this skill's
current rules (`SKILL.md`, "This skill does not add new unit tests to the project") don't permit
that, so the cutoff math (`eviction_cutoff` in `patch.diff`) was instead verified with a temporary,
not-shipped test during development (empty/exactly-at-limit/one-past/unsorted/180000-row cases,
matching `FIX.md`'s own list) and left out of the final diff.

## Does it change behavior/output?

No patch/diff was preserved from the original session for this candidate, despite it being
recorded there as "win, applied". `FIX.md` (copied verbatim into this folder) describes the fix
that was made - including its measured pixel-diff and test-suite results - but the actual code
change itself was not saved anywhere findable, so it cannot be reproduced or verified here. Treat
the mechanism and numbers above as a description of prior work, not as something this folder can
independently confirm; no `justfile` is provided for this candidate because there is no patch to
apply.
