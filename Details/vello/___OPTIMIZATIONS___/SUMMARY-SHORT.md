# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 007 | Strong win | GPU coarse-rasterization PTCL construction had no occlusion check, so tiles with hundreds of stacked fully-covering opaque draws executed every one of them per pixel | +34/-2 | 400-layer opaque-overdraw scene: ~4x faster multi-core, ~10x single-core |
| 012 | Strong win | Pixmap::take_unpremultiplied always unpremultiplied every pixel even when the whole background was one opaque viewport-sized fill_rect | +653/-37 | take_unpremultiplied_only ~30,000x faster; full render+take pipeline ~65-75% lower |
| 005 | Strong win | highp fine-rasterizer's f32<->u8 tile pack/unpack used scalar per-pixel conversion instead of the crate's existing SIMD helpers | +46/-27 | pack ~3.8x faster, unpack ~1.5x faster (AVX2) |
| 004 | Strong win | Blurred-rounded-rect (box-shadow) SDF eval called scalar libm powf() per pixel instead of a vectorized approximation | +60/-23 | AVX2 ~38-42% faster, SSE4.2 ~15-26% faster |
| 011 | Strong win | vello_cpu's image/gradient painters re-splat a scalar position into SIMD every pixel chunk instead of keeping it vectorized and advancing it | +275/-174 | unpinned AVX2 mostly -2% to -22%; single-core mostly -15% to -50% |
| 017 | Strong win | vello_hybrid's Renderer::render always redrew the whole frame, with no way to redraw only a changed region or take an integer-rect-clip fast path | +2911/-215 | damage-region render ~7-8x faster multi-core, ~24-25x faster single-core; rect-clip fast path ~2.5x faster |
| 003 | Small win | Solid-color fill compositing redundantly multiplied by src_alpha even when fully opaque | +42/-3 | opaque+masked path ~26-35% faster single-core |
| 006 | Small win | BGRA8->RGBA8 image load used a scalar per-pixel channel swap instead of the crate's existing SIMD pattern | +60/-5 | ~10-14% faster image load |
| 009 | Small win | Every LINETO segment in the CPU flatten-shader mirror is degree-raised to a cubic and run through the full Euler-spiral machinery despite being collinear | +267/-1 | fill p50 ~1.66x faster; stroke p50 ~1.25x faster |
| 014 | Small win | vello_hybrid's ColorMatrix filters (grayscale/sepia) ran a full unpremultiply/apply/repremultiply round trip per pixel instead of applying the matrix directly in premultiplied space | +943/-18 | 2048x2048 ColorMatrix render: ~12-15% faster single-core, ~7-10% faster multi-core (llvmpipe) |
