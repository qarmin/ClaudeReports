# Fewer hasher writes in `LayoutJob` / `TextFormat` / `TextWrapping` / `Stroke` hashing

Every label layout (cache hit or miss) hashes its `LayoutJob` with ahash to look up the galley cache. A one-section job needed about 35 separate small writes (each `OrderedFloat`, `bool`, enum discriminant and `Color32` was its own write; the fallback ahash mixes on every write). Packed the small fields into fewer wider writes: job flags + `first_row_min_height` in one `u64`; `TextWrapping` max_width/break_anywhere/overflow char in one `u64`; `TextFormat` letter spacing + expand_bg, color + background, and `italics + valign` in `u64`/`u64`/`u8`; `Stroke` width + color in one `u64`. Float fields keep `OrderedFloat`'s equivalence classes (all NaN equal, `-0.0 == 0.0`) via a local `float_bits`, so Hash stays consistent with `PartialEq`. Independent of candidate 006 (colors are converted with `Color32::to_array`, not via `Color32::hash`); the two overlap on the color writes, the combined effect was not measured.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time (allocation counts unchanged: the hash allocates nothing).

## What was tried

Interleaved A/B of prebuilt criterion binaries (`tools/ab.sh`, pinned core, 6 rounds, load average ~1). Ratios patched/base:

| Benchmark | ratios per round |
|---|---|
| text_layout_cached | 0.8832, 0.8929, 0.8817, 0.8832, 0.8823, 0.8889 |
| demo_no_tessellate | 0.9525, 0.9629, 0.9633, 0.9698, 0.9635, 0.9684 |

## Result

Time: `text_layout_cached` (one cache-hit layout, 160 ns) about -11%; whole UI build `demo_no_tessellate` (156 us) about -3.5% (-3% to -4.8%), 6/6 rounds each. Allocation: no change.

## Does it change behavior/output?

- **Bit-exact rendering/layout?** Yes. Only `Hash` values of `LayoutJob`, `TextFormat`, `TextWrapping` and `Stroke` change (galley cache keys, any user hash map keyed on them). Hash/Eq consistency is preserved (same float normalization as `OrderedFloat`). Collisions: more fields are packed into a single mix, but each packed write still carries all its input bits (no information is dropped; `Option<char>` overflow char is encoded as `c + 1`).
- Breaking change: no API change.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed (1 ignored), `cargo test -p egui` 86 + 3 + 188 passed; same counts unpatched.
- Real change to the tested project's source: `patch.diff` (+39/-17).

## Why the result looks the way it does

The galley-cache key hash is on the path of every label every frame; with the fallback (non-AES) ahash each write is a folded multiply, so cutting about 20 writes of ~35 saves ~20 ns per label. The measured -11% of the 160 ns cached layout matches that; spread over ~100 labels in the demo it is the observed -3.5% of the frame.

## What's not done / open questions

- `FontId` (size + family) and `VariationCoords` hashing still use separate writes.
- Combined effect with candidate 006 (Color32 hash) not measured.
