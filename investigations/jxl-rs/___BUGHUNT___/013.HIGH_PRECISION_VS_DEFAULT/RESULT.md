# high_precision=true vs default output

The default 'low precision' path (f16 / approximations) must stay close to the high-precision path; a large divergence would flag a bug in one of the fast paths.

Base commit 7f539c8; main tree not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `harnesses/diff_formats hp`: f32 Rgb output with `high_precision` false vs true | all 71 files in resources/test and the 38 conformance images | max abs difference 3.2e-4 (multiple_layers_noise_spline.jxl), mostly exactly 0 |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

The two paths agree to 3.2e-4 on every file.

## What's not done / open questions

- Only files from the repo were used, no fuzzed inputs.
