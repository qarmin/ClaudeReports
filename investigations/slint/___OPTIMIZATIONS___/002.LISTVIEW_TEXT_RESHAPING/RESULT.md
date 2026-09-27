# 02 - Does ListView re-shape visible text on every frame?

Ported from `perf_test/___OPTIMIZATIONS___/02_listview_text_reshaping/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

The cache behaves correctly; the hypothesis was wrong.

> **Measured before candidates 008-010 were applied.** On the current tree the absolute numbers are
> better (the `scroll` baseline is now ~133 ms/frame with 300 742 allocations and 1 424 MB instead
> of 148 ms / 701 063 / 1 542 MB), but the comparison between the variants - which is what this
> candidate is about - is unchanged.

## Why it looked suspicious

`ListView` over 100 000 records allocates ~2 131 times per frame while scrolling, with only
~25 rows visible. The `--alloc-profile` output for such a frame is dominated by stacks that
end in parley shaping:

```
~4 allocations (3.8%)
    ItemCache<CachedParagraphs>::get_or_update_cache_entry_ref  [item_rendering.rs:169]
    cached_paragraphs  [textlayout/sharedparley/cache.rs:182]
    ...
    <ComplexText as Item>::render  [items/text.rs:143]
```

That reads like "the visible text is shaped again every frame".

## What was tried

`TextLayoutCache::cache_miss_count()` per frame (the counter the cache itself keeps):

```
../target/rdebug/perf_test --bench --mode list --count 100000 --frames 5

frame          time     allocations   text shapes   line breaks
    0        2.77ms            3581             0            79
    1        2.25ms            2132            18            36
    2        2.22ms            2132            18            36
    3        2.24ms            2130            18            36
    4        2.26ms            2130            18            36
```

## Result

18 shapes per frame, and the scroll step is 200 px over 34 px rows = ~6 rows entering the
viewport, 3 `Text` items each = **18**. So exactly the newly appeared rows are shaped and
nothing else; the visible rows that were already there are served from the cache. Frame 0
shapes nothing because the shaping happened during the layout pass triggered by `show()`
(before the counter reset) - that is candidate 003.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session. This
candidate is a measurement that confirmed existing `ListView` caching behavior is already correct.

## Why the result looks the way it does

Virtualization already works as intended: `ListView` only creates and shapes rows as they enter
the viewport, and the shaped paragraphs stay in the per-item cache for rows that remain visible.
The allocation profile looked suspicious only because it was read without the shape counter to
confirm which allocations were new shaping work versus cache hits.

## What's not done / open questions

`line breaks` is 36 = 2 per newly shaped text: the lines of a new row's text get broken
twice per frame, presumably once for `layout_info` (measuring, unconstrained width) and once
for drawing (assigned width), so the retained line breaking cannot be reused. At 18 new
texts per frame that is noise (2.25 ms/frame total), but on a wide table with many columns
scrolling fast it is 2x the line-breaking work. Not pursued here. `RetainedLineBreaking` in
`internal/core/textlayout/sharedparley/layout.rs` is where it would be looked at.
