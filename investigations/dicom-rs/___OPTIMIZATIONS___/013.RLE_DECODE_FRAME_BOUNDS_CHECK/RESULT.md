# RLE Lossless decode_frame: same bounds-check removal as candidate 011, applied to the single-frame path

Direct follow-on to candidate 011, which fixed this exact anti-pattern in
`RleLosslessAdapter::decode` (the whole-object decode path) and flagged `decode_frame` (the
single-frame variant, sharing the same interleave loop shape almost verbatim) as an untested but
likely-equivalent win. Confirmed here.

## What was tried

Applied the identical rewrite from candidate 011 to `decode_frame`'s interleave loop: slice the
destination range once (`&mut dst[base_offset + start..base_offset + end]`) and write through
`.iter_mut().step_by(stride).zip(decoded_segment.iter())` instead of indexing both buffers by
computed offsets inside the loop.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s new `decode-frame-raw` mode
(`PixelDecoder::decode_pixel_data_frame(0)` alone) against `WG04/RLE/CT1_RLE` (same fixture as
candidate 011).

| Variant | multi-core p50 |
|---|---|
| baseline (indexed loop) | 0.336/0.335 ms (2 rounds) |
| patched (iterator-based) | 0.237/0.244 ms (2 rounds) |

## Result

**-29.4%/-27.3% across two independent rounds, 9/9 and 8/9 paired wins, both consistent.** A
larger relative win than candidate 011's `decode` fix (-9.0%/-10.8%) on the same underlying change
- plausible explanation (not independently profiled this candidate): `decode_frame` only decodes
one frame, so the interleave loop is a much larger fraction of its total per-call cost than in
`decode` (which pays other per-frame overhead - fragment lookup, RLE header parsing - proportionally
diluted across all frames when decoding a whole multi-frame object at once, though this fixture is
single-frame so that specific explanation doesn't fully apply here either; not chased further).

## Does it change behavior/output?

- **Bit-exact?** Yes. FNV-1a checksum of the decoded frame data is identical between baseline and
  patched builds.
- **Is this a breaking change?** No public API/type changes - same as candidate 011.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. The 2 RLE-specific integration
  tests (`--features rle`) also pass unchanged.

## Why the result looks the way it does

Identical mechanism to candidate 011 - see that candidate's RESULT.md for the full explanation.

## What's not done / open questions

- The larger relative win vs. candidate 011 was noticed but not root-caused (see Result) - would
  need per-phase timing inside `decode`/`decode_frame` to attribute precisely.
- Same open items as candidate 011 apply here too (LUT construction math, not re-listed).
