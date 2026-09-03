# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

Fix lines marked `*` (candidates 033/038/048/049/053/058): these vendor the whole `pic-scale-safe`
crate into `vendor/pic-scale-safe/` so `[patch.crates-io]` can point at it; `git apply --numstat` on
their `patch.diff` counts that entire copied-in crate as new-file additions (git has never seen
`vendor/` before), not just the actual gate/multiversioning change. The real behavioral diff in
each is much smaller (a ~10-30 line row/size gate for 033/038; a
`#[target_feature(enable = "avx2")]`/`"avx512f,avx512bw,avx512vl"` variant function plus dispatch
for 048/049/053/058) - summed literally per `SUMMARY_SHORT_template.md`'s instruction here for
consistency with every other row, but don't read these six as "9000-line changes."

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 001 | Strong win (flip_vertical_in) / Small win (rotate180_in_place) | affine.rs flip_vertical_in/rotate180_in_place row-slice fast path | +43/-1 | flip_vertical_in ~7.5-9x faster; rotate180_in_place ~11-27% faster |
| 004 | Strong win | affine.rs flip_horizontal_in/flip_horizontal_in_place row-slice fast path | +51/-1 | flip_horizontal_in ~2.3-2.7x faster; flip_horizontal_in_place ~1.5-1.6x faster |
| 005 | Strong win | imageops::resize vertical_sample row-slice fast path | +62/-0 | ~23-27% faster wall-clock (Triangle/Lanczos3) |
| 006 | Strong win | imageops::thumbnail thumbnail_sample_block fast path | +54/-3 | ~2.4-2.5x faster (4032x3024 -> 200x150) |
| 007 | Strong win | affine.rs rotate90_in/rotate270_in blocked-transpose fast path | +80/-0 | rotate90_in ~11% faster; rotate270_in ~37-42% faster |
| 008 | Small win | bmp decoder 8-bit palette fixed-array lookup | +7/-4 | ~3-5% faster BMP 8-bit decode |
| 011 | Strong win | bmp decoder 4-bit/2-bit/1-bit palette fixed-array lookup | +14/-6 | ~8-10% faster BMP 4-bit decode, ~7.5-7.8% faster 1-bit |
| 013 | Strong win | imageops::resize Rgb\<u8\> fast path via pic-scale-safe delegation | +82/-2 | ~3.9-4.2x faster (RgbImage resize) |
| 015 | Small win | resize RgbaImage fused-alloc premultiply | +213/-5 | ~16-24% faster (non-constant-alpha, small output sizes) |
| 018 | Strong win | removes candidate 017's RGBA fast-path size threshold | +219/-5 | ~25-56% faster vs. 017's threshold build (CatmullRom/Gaussian/Lanczos3) |
| 020 | Strong win | bmp 2-bit decode bench corpus + fixed-array fix | +15/-6 | ~8% faster BMP 2-bit decode |
| 023 | Strong win | pic-scale-safe rayon capped thread count (small resizes) | +103/-7 | Triangle ~35-45% faster, Lanczos3 ~2-5% faster |
| 026 | Strong win | pic-scale-safe rayon capped thread count extended to CatmullRom/Gaussian (small resizes) | +108/-7 | CatmullRom ~21-24% faster, Gaussian ~27% faster |
| 027 | Strong win | pic-scale-safe rayon thread-count sweep for CatmullRom/Gaussian - N=6 beats untuned N=4 | +108/-7 | CatmullRom ~28-33% faster, Gaussian ~24-33% faster (N=6 vs. no gate) |
| 029 | Strong win | pic-scale-safe rayon per-filter capped thread count (N=4 for Nearest/Bilinear/Lanczos3, N=6 for CatmullRom/Gaussian) | +120/-7 | Triangle ~44% faster, Lanczos3 ~17% faster, CatmullRom ~27% faster, Gaussian ~28% faster, all in one build |
| 033 | Strong win | vendored pic-scale-safe, dst_height>=32 gate on resize_nearest's rayon dispatch | +8280/-5\* | closes Nearest's 26-37x small-resize regression - ~12-24x faster than ungated, matches true serial |
| 038 | Small win | vendored pic-scale-safe row-floor gate, measured at a genuinely small dst_height=6 output | +8508/-5\* | ~13-21% faster multi-core, ~6-15% faster single-core (Triangle/CatmullRom/Gaussian/Lanczos3) vs. no-floor pool |
| 048 | Strong win | vendored pic-scale-safe runtime AVX2 multiversioning (`is_x86_feature_detected!` + `#[target_feature]`) of the vertical convolution tap loop, default RUSTFLAGS | +8486/-2\* | ~27% faster single/multi-core (Lanczos3 4032x3024->400x300), bit-exact, works on a normal `cargo build --release` |
| 049 | Strong win | vendored pic-scale-safe runtime AVX2 multiversioning extended to the horizontal convolution pass and the floating-point vertical/horizontal paths, on top of candidate 048's vertical-only fix, default RUSTFLAGS | +8913/-2\* | ~40% faster single/multi-core (Lanczos3 4032x3024->400x300), ~39-40% faster on CatmullRom/Gaussian (floating-point path, untouched by 048), bit-exact |
| 053 | Small win | vendored pic-scale-safe runtime AVX-512 tier added to the vertical-convolution dispatcher, on top of candidates 048/049's AVX2/scalar runtime multiversioning, default RUSTFLAGS | +9042/-2\* | ~2-3.5% faster on top of 048/049's own AVX2 win (Lanczos3/Triangle 4032x3024->400x300 and a small-image case), bit-exact |
| 054 | Strong win | `#[inline(always)]` added to `brighten_in_place`'s candidate-003 `to_pixel_view_mut` fast path, recovering an inlining-context regression | +53/-2 | ~27% faster single/multi-core, 14/14 paired runs, bit-exact |
| 058 | Small win | vendored pic-scale-safe runtime AVX-512 tier extended to the horizontal-convolution and floating-point-vertical dispatchers, on top of candidate 053's vertical-only AVX-512 tier, default RUSTFLAGS | +9301/-2\* | ~1.4-3.6% faster (Lanczos3/Triangle/CatmullRom/Gaussian, single-core), no measurable change on 2 of 6 scenarios, bit-exact |
| 060 | Small win | `#[inline(always)]`/`#[inline]` added to `DynamicImage::get_pixel`/`put_pixel`, recovering lost cross-module inlining into `imageops::overlay`'s per-pixel loop | +23/-2 | ~7% faster single/multi-core on a DynamicImage overlay bench, 10/10 paired runs, bit-exact |
| 064 | Small win | same 060 fix, now benched for `tile`/`replace`/`vertical_gradient`/`horizontal_gradient` too | +42/-2 | ~10-25% faster on tile/replace/vertical_gradient (7-10/10 paired runs each); horizontal_gradient's delta was within noise, bit-exact |
