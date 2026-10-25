# Consistency of the output formats (colour types, sample types, premultiply, orientation)

Logic bugs in the new save-stage code (`SaveStage::save_lowmem`, interleaving, BGR(A)/gray(+alpha) ordering, orientation) would not crash, so a differential oracle was needed.

Base commit 7f539c8 (jxl-rs main); the main tree was not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential: same file decoded as Rgb/Rgba/Bgr/Bgra/(Gray/GrayAlpha) x U8/U16/F16/F32, compared against the Rgb result of the same sample type | `harnesses/diff_formats` subcommand default, all 71 files in jxl/resources/test (not the conformance_test_images directory), premultiply on/off | No channel-order or alpha-interleave mismatch (exact equality required) |
| Differential: adjust_orientation=false decoded and transformed by hand vs adjust_orientation=true | all 8 orientation*.jxl files + bench_oriented_brg, every colour type x sample type, extra channels included | exact match for all |
| Cross-format sample-type consistency | U8 vs U16 within 1 level, F16 vs F32 within 2e-3 relative, extra channels U8 vs F32 | only observation: U8 output differs from round(U16/257) by +-1 level on ~10% of samples (symmetric, up to 1/255); not treated as a defect (not covered by the spec's pixel-exactness, precision choice), F32 extra channels are not clamped so they can be slightly negative where U8 clamps to 0 |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

All of the tested output paths agree. Premultiplied output for `alpha_associated` images (candle.jxl) was inspected by hand and is consistent with 'straight output un-premultiplies, premultiplied output passes the data through'.

## What's not done / open questions

- Animated files were skipped by the oracle (first frame only is not compared), and only the default sRGB/embedded output profile was used.
