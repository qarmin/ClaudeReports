# Reserve `Paragraph::glyphs` capacity once per text segment

The `big allocations` probe of the new allocation-counting harness (`harness/alloc_count`, README) showed one uncached layout of the 900-char lorem ipsum doing a doubling realloc chain 2304 -> 73728 bytes: `Paragraph::glyphs` (72-byte `Glyph`) growing by `push`, copying the whole vec 5 times. Now `layout_section` reserves `segment.chars().count()` glyphs before shaping each non-empty segment (one glyph per char is an invariant of the layout, see `layout_shaped_run`).

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metrics: allocation count / allocated bytes per iteration (counting global allocator, system malloc) and wall-clock time.

## What was tried

| Scene (harness `alloc_count`) | allocs/iter base | allocs/iter patched | bytes/iter base | bytes/iter patched |
|---|---|---|---|---|
| text_layout_uncached (900 chars) | 128.0 | 113.0 | 468,912 | 344,352 (-26.6%) |
| demo frame (run_ui, run_ui+tessellate, only_tessellate), cached text | unchanged | unchanged | unchanged | unchanged |
| text_layout_cached, short_label_cached | 2.0 / 2.0 | 2.0 / 2.0 | 1472 / 147 | same |

Peak live bytes delta per iteration (harness): 240,168 both (peak is dominated by the final galley, not by the transient growth chain).

Time, interleaved A/B (`tools/ab.sh`, pinned core, 5 rounds), patched/base: text_layout_uncached 0.9873, 0.9867, 0.9775, 0.9868, 0.9895; text_layout_cached_many_lines_modified 0.9991, 0.9942, 1.0062, 0.9964, 0.9708 (noise).

## Result

Allocation: -15 allocations and -124.6 KB allocated per uncached 900-char layout (-26.6% bytes). Time: about -1.3% on `text_layout_uncached` (5/5 rounds), no measurable change elsewhere. Peak live memory unchanged.

## Does it change behavior/output?

- **Bit-exact?** Yes - only Vec capacity changes.
- Breaking change: no. Capacity can be larger than needed by at most the difference between estimated and produced glyphs (continuation glyphs keep glyphs == chars).
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed (1 ignored) before and after.
- Real change to the tested project's source: `patch.diff` (+3/-0).

## Why the result looks the way it does

Realloc growth costs ~2x the final size in copies, which at 72 B/glyph is visible in bytes but small compared with shaping and atlas work in time. The `chars().count()` pass adds a few hundred ns for long text, partly cancelling the saved copies.

## What's not done / open questions

- A second doubling chain (2304 -> 36864 bytes, 36-byte elements) in the same layout was not located (probably inside harfrust buffers).
- Per-row `paragraph.glyphs[a..b].to_vec()` copies (lines ~784/828) could move glyphs out instead of copying.
