# Per-cluster lookups in `segment_into_runs`: ASCII-indexed face cache, and run-merge check before the `fonts.face()` lookup

For every grapheme cluster `segment_into_runs` did an `ahash::HashMap<char, FontFaceKey>` lookup (`Family::resolve_cluster`, 1.9% self in the `text_layout_uncached` profile) and then a second `IntMap` lookup (`fonts.face(font_key).is_none()`) just to check the face exists, even when the cluster simply continued the previous run (same face, which was already known to exist).

Changes: (1) `Family` gets a direct-indexed `[Option<FontFaceKey>; 128]` in front of `face_cache`, filled alongside it; (2) the "same face as the previous non-raster run -> extend it" check now runs before the face-existence check. Independent of candidate 001 (the diff applies to the base commit on its own; the combination with 001 was not measured).

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time only.

## What was tried

Interleaved A/B of prebuilt criterion binaries (`tools/ab.sh`, pinned core, 5 rounds). Ratios patched/base:

| Benchmark | ratios per round |
|---|---|
| text_layout_uncached | 0.9733, 0.9760, 0.9699, 0.9739, 0.9691 |
| demo_no_tessellate | 1.0042, 1.0030, 0.9906, 0.9998, 0.9951 |

## Result

Time: `text_layout_uncached` about -2.4% to -3.1%, 5/5 rounds. `demo_no_tessellate` (layout mostly served from the galley cache) within noise.

## Does it change behavior/output?

- **Bit-exact?** Yes - lookups return the same keys; the reordered branch only skips the face-existence check for a face that the previous run already proved to exist (the previous run is only extended when it is a non-raster run, which is only created after that check passed).
- Breaking change: no. Memory: +2 KiB per `Family` (128 x 16 bytes).
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed (1 ignored), `cargo test -p egui` 86 + 3 + 188 passed; unpatched counts identical.
- Real change to the tested project's source: `patch.diff` (+23/-7).

## Why the result looks the way it does

Two hash-map probes per character become one array index plus a compare against the previous run's key. Shaping (harfrust) and per-glyph atlas lookups, which dominate layout, are unchanged, hence only a few %.

## What's not done / open questions

- Combined effect with candidate 001 (same function) not measured separately.
- Chars outside ASCII still go through `face_cache`.
