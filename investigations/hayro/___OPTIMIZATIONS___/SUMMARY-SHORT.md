# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 007 | Strong win | Content-stream dispatch caches the resolved `Tf` font by resource name, scoped per content stream, instead of re-resolving on every `Tf` operator | +19/-7, -16% to -18% render() p50 on `img.pdf` (both single-core and multi-core) |
| 010 | Small win | `hayro-jpeg2000`'s `round_f32` gates the `std` `x.round()` fast path on `sse4.1`/`neon`, falling back to the existing manual libm-free implementation otherwise | +31/-4, -3.2% single-core p50, -2.2% multi-core p50 on `jpx_decode_bench` |
| 005 | Small win | RGB->RGBA expansion for many small images (`img.pdf`'s real shape) | +19/-9, -2.7% isolated (map_array_collect), within noise end-to-end |
| 020 | Small win | Same `resolved_font_cache` fix as candidate 007, profiled via `valgrind --tool=callgrind` to resolve an inconclusive wall-clock reading on a cache-defeating input | +19/-7, -0.47% total instructions/render (deterministic), `Context::resolve_font()` calls drop 62.1% |
| 024 | Small win | Pre-size candidate 022's `resolved_font_cache` with `with_capacity` instead of growing it from empty via repeated inserts | +28/-9, cuts 30.8% of candidate 022's own worst-case regression (+0.368% -> +0.255% Ir vs. no-cache baseline) |
| 026 | Small win | Force `#[inline(never)]` on JPEG2000 IDWT horizontal-filter's 5 split-out step functions (candidate 014) instead of leaving inlining to LLVM's heuristic | +5/-0, -0.8% single-core p50, -0.5% multi-core p50 (close to noise floor) |
| 029 | Small win | Same `Tf`-resolution-cache fix as candidate 007, re-run on a real PDF with 218 `Tf` calls/page reusing 7 fonts | +19/-7, -1.4% single-core p50 (20/21 paired wins), -1.5% to -1.8% multi-core |
| vello/002 | Strong win | vello (vendored dependency): PNG DEFLATE compression level `Balanced` -> `Fast` | +5/-0, -87.8% isolated, -48.6% end-to-end (real `img.pdf`) |
| vello/003 | Small win | vello (vendored dependency): PNG DEFLATE compression level `Fast` -> `Fastest` | +1/-0, -20.7% isolated vs. `Fast`; -88.5% isolated vs. current `Balanced` default |
| vello/001 | Small win | vello (vendored dependency): `Pixmap::take_unpremultiplied` rewritten from float division to an integer LUT | +34/-14, -25.4% isolated; within noise end-to-end for hayro's 3 test PDFs |
| 040 | Strong win | `hayro-jpeg2000`'s `floor_f32` (Reversible53 IDWT path) gates the `std` `x.floor()` fast path on `sse4.1`/`neon`, same pattern as candidate 010's `round_f32` | +23/-2, -27% single-core p50, -25% multi-core p50 on a real filter_horizontal+filter_vertical microbenchmark, bit-exact |
| 041 | Small win | hayro's workspace `Cargo.toml` has no `[profile.release]` section, preventing cross-crate LTO inlining across the hayro/vello boundary | +4/-0, -3.58% to -9.46% single-core median across two paired-run rounds, bit-exact |
| 043 | Small win | PDF FlateDecode stream decompression reads into a `Vec::new()`, forcing `read_to_end`'s default grow-from-empty doubling instead of a pre-sized output buffer | +10/-2, isolated bench: -4.5% single-core / -2.6% to -7.2% multi-core p50, bit-exact |
| 045 | Small win | JBIG2's `Bitmap::combine()` (maintainer-flagged bottleneck) redoes per-word bounds-checked source lookups from scratch for every destination word of every symbol/pattern placement | +98/-11, -2.5% single-core / -1.9% multi-core p50 on symbol-heavy JBIG2 content, bit-exact |
