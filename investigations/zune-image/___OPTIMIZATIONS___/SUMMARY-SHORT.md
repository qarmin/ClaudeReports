# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 001 | Strong win | PNG Paeth SSE4.1 defilter builds/extracts each pixel into a 128-bit register via a per-byte `pinsrb`/`pextrb` chain instead of one GPR assemble + single `movq` | +52/-20 | -25.0% decode time, 9/9 paired runs |
| 003 | Small win | HEIC YCbCr->RGB color conversion was pure scalar despite sharing zune-jpeg's recently-vectorized coefficients/loop shape - ported an AVX2 kernel from zune-jpeg | +204/-3 | -1.5% single-core decode time, paired 8/9 |
| 004 | Strong win | Composite Over/Multiply/Screen (u8) did a per-pixel f32 convert/multiply/clamp/round/convert-back instead of reusing premul_alpha.rs's fastdiv-style integer div255 technique | +147/-12 | single-core 2.3-2.9x, multi-core 2.0-2.3x faster, paired 9/9, median -59.4% |
| 005 | Strong win | premultiply_u8/u16 and unpremultiply_u8/u16 ran plain scalar per-channel loops instead of using the crate's existing simd_fn! AVX2/NEON dispatch macro, unlike the module's own SIMD f32 path | +98/-13 | u8 -43% faster, u16 -25-27% faster (single/multi-core), all paired 9/9 |
| 008 | Small win | QOI encode wrote each opcode byte-by-byte through the generic ZWriter/ZByteWriterTrait sink abstraction across the zune-qoi -> zune-core crate boundary instead of buffering into a local Vec<u8> and flushing once | +41/-49 | single-core -1.44% (15/15 paired), multi-core -2.53% (14/15 paired); initial ~16% figure was build-heat confounded, corrected via alternating comparison |
