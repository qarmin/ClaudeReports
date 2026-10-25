# `OutlineGlyphKey` hashed from four packed words instead of six fields

`GlyphAtlas::allocate_outline` (9% self in the `text_layout_uncached` profile, about 27% of that being ahash mixing) builds its cache key per glyph by hashing a 6-field tuple (face key, glyph id, two f32 bit patterns, the subpixel bin enum, location hash). Packed glyph id + bin and the two f32 bit patterns into `u64` words, so the tuple has 4 writes instead of 6. The key is only a hash (no stored fields to compare), so all input bits are still part of the hashed words: no information is dropped.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time (no allocation change).

## What was tried

Interleaved A/B of prebuilt criterion binaries (`tools/ab.sh`, pinned core, 6 rounds). Ratios patched/base for `text_layout_uncached`: 0.9816, 0.9861, 0.9811, 0.9790, 0.9799, 0.9946.

## Result

Time: about -1.4% to -2.1% on `text_layout_uncached`, 6/6 rounds faster. Cached paths are unaffected (the key is only computed while laying out glyphs).

## Does it change behavior/output?

- **Bit-exact?** Yes: only the internal cache-key hash values change. (As before, a 64-bit hash collision between two distinct glyph keys would return the wrong cached bitmap; the packed words carry the same information as the old fields.)
- Breaking change: no.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed (1 ignored), same as unpatched.
- Real change to the tested project's source: `patch.diff` (+3/-4).

## Why the result looks the way it does

Two fewer folded-multiply mixes per glyph (~2-3 ns each) out of ~200 ns of per-glyph layout work.

## What's not done / open questions

- Whole-structure alternatives (a direct `IntMap` keyed by `(glyph_id, bin)` per face/metrics) would avoid hashing the constant fields per glyph entirely; not tried.
