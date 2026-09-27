# `vello_cpu::fill_path` per-call cost for many small paths (glyphs)

**No patch attempted - investigation-only.** No low-risk fix was identified; see "What's not
done" below for the one plausible lever found and why it wasn't pursued.

The pipeline breakdown (see `../README.md`) found `draw_glyph` at 38.7% of `render()`'s time, of
which ~89% is a single call: `vello_cpu::RenderContext::fill_path()`, invoked once per glyph
*instance* (1232 times for `img.pdf`). Back-of-envelope: draw_glyph's ~114-140ms per 100
iterations / 1232 glyphs ~ **~1 microsecond per glyph fill call** (cache lookup + transform +
flatten + tile + strip + record, combined).

## What was tried

Code reading only, no patch built or measured - traced the call path:
`RenderContext::fill_path` (`vello_cpu/src/render.rs:273`) ->
`SingleThreadedDispatcher::fill_path` (`vello_cpu/src/dispatch/single_threaded.rs:277`, confirmed
single-threaded is the active path - hayro's `RenderSettings` uses `num_threads: 0`) ->
`StripGenerator::generate_filled_path` (`vello_common/src/strip_generator.rs:120`) ->
`flatten::fill(...)` (Bezier curve flattening into line segments) + tiling/strip generation ->
`record_fill` (records a paint command for the coarse rasterizer).

What was checked:
- `StripGenerator`, `SingleThreadedDispatcher`, and `CommandBucketer` all hold their working
  buffers (`line_buf`, `flatten_ctx`, `tiles`, `strip_storage`, `bucketer`, `recorder`,
  `viewport`) as struct fields, reused across every `fill_path` call - no per-call heap
  allocation at the structural level found by reading the code.
- `encode_current_paint()` (called once per `fill_path`) has a fast path for `PaintType::Solid`
  (the common case for text) that's a trivial conversion, no allocation.
- No obviously-avoidable redundant work spotted for a *tiny* input path (a glyph outline is a
  handful of curve segments) going through the same code as a large path - the pipeline stages
  (flatten -> tile -> strip -> record) all appear proportional to input size, not
  fixed-cost-dominated in an obviously wasteful way.

No `perf` available this session to get instruction-level attribution inside this call -
everything above is from reading the code, not profiling it, so this is a weaker,
"nothing obviously wrong found" result rather than a strong "confirmed nothing can be improved"
one.

## Result

No patch was built - see "What was tried" above. Nothing measured.

## Does it change behavior/output?

Not applicable - no code was changed.

## Why the result looks the way it does

The architecture already reuses all working buffers across calls and has no obvious per-call
allocation or redundant work. The one plausible remaining lever - batching multiple
non-overlapping glyphs from the same text run into a single combined path / single `fill_path`
call instead of one call per glyph - is architecturally promising (correct in principle: filling
the union of non-overlapping shapes with `NonZero` gives the same result as filling them
individually) but is:
1. A change in **hayro's own glyph-drawing code** (`hayro/src/glyph.rs`), not vello - vello
   already exposes the right primitive (`fill_path` takes any `BezPath`, which could be a
   multi-subpath combination).
2. Correctness-sensitive in edge cases not exhaustively enumerated this session (glyphs *can*
   overlap - synthetic bold/oblique transforms, unusual kerning, Type3 glyphs with strokes that
   extend beyond their nominal advance box - each would need verifying that batched fill-rule
   semantics still match per-glyph filling).
3. A bigger scope change than the other candidates in this audit, which all touched one
   function/one code path - batching would touch the glyph-drawing loop's control flow.

## What's not done / open questions

Getting real `perf` instruction-level data on `fill_path` first (this session's biggest
limitation throughout was the absence of it) would confirm where inside flatten/tile/strip/record
the microsecond actually goes, before attempting the batching rewrite - it's possible the real
cost is concentrated in one specific sub-step (e.g. tile-grid setup) that has a much smaller,
safer fix than full glyph-batching.
