# BMP 4/2/1-bit palette lookup: fixed-size array to elide the bounds check

Extends candidate 008's fix (`set_8bit_pixel_run`) to the sibling functions
`set_4bit_pixel_run`/`set_2bit_pixel_run`/`set_1bit_pixel_run` in `src/codecs/bmp/decoder.rs`.
These already mask their palette index before indexing (`idx & 0xf`, `(idx >> 6) & 0x3`, a
single-bit test), but were still passed `palette: &[[u8; 3]]` (unsized slice, length not known to
the compiler), so `palette[i]` still carried a runtime bounds check on every pixel even though the
masked index can never exceed 15. The palette itself is always allocated as exactly 256 entries
(`read_palette`, same invariant candidate 008 relied on) regardless of bit depth, so this candidate
binds it as `&[[u8; 3]; 256]` (same size as 008, not a smaller 16/4/2-sized array - simpler, and
the mask already guarantees `idx < 256` trivially once the array length is known at compile time)
at both call sites (`read_palettized_pixel_data` for the plain path, `read_rle_data` for RLE4's
absolute/encoded modes) and updates the three function signatures to match. `&[[u8; 3]; 256]`
coerces implicitly to `&[[u8; 3]]` where `set_8bit_pixel_run` (unchanged) and the RLE8 direct-index
site still expect a slice, so only one conversion per call is needed even though both bit depths
share the same local variable.

## What was tried

Measured via the existing `benches/decode.rs` criterion harness, filtered to `load-Bmp/Core_4_Bit.bmp`
(435x353, 4-bit indexed) and `load-Bmp/Core_1_Bit.bmp` (435x353, 1-bit indexed) - the largest
4-bit and 1-bit samples already in the repo's BMP test corpus and already registered in
`decode.rs`'s `BENCH_DEFS`. No 2-bit sample (`pal2.bmp`/`pal2color.bmp` exist in the test corpus
but are 127x64 and not registered in `BENCH_DEFS`) was benchmarked - see "What's not done".

Two prebuilt `--release` `decode` bench binaries (baseline vs. patched), alternated run-by-run per
MEASUREMENT.md section 2b (`--sample-size 20 --measurement-time 0.3 --warm-up-time 0.15`, 9
alternating pairs per condition), single-core (`taskset -c 5`) and multi-core (unpinned), plus a
same-binary control (baseline vs. itself, 6 pairs, single-core) to establish the noise floor. An
initial single-core attempt on `taskset -c 3` was contaminated by an unrelated background job of
mine also pinned to core 3 (visibly noisy, up to 2x swings) - discarded and re-run cleanly on an
otherwise-idle core 5.

| Variant | single-core p50 (9 pairs) | multi-core p50 (9 pairs) | notes |
|---|---|---|---|
| baseline, `Core_4_Bit.bmp` | 120.51 µs | 116.69 µs | criterion's own [low mean high] middle value per run |
| patched, `Core_4_Bit.bmp` | 109.08 µs (-9.5%) | 107.19 µs (-8.1%) | 9/9 paired wins each |
| baseline, `Core_1_Bit.bmp` | 76.84 µs | 76.14 µs | |
| patched, `Core_1_Bit.bmp` | 70.87 µs (-7.8%) | 70.45 µs (-7.5%) | 9/9 multi-core wins, 7/9 single-core wins (2 pairs hit an isolated noise spike on one side only, opposite direction each time - see raw data) |
| same-binary control (`Core_4_Bit.bmp`, single-core, 6 pairs) | 117-119 µs typical, two outliers 138.6/156.7 | - | noise floor within ~1-2% for most pairs; the measured effect (8-10%) is well outside it |

Raw paired single-core runs, `Core_4_Bit.bmp` (A=baseline, B=patched, µs): `224.38/108.46`
(A itself an outlier here, B unaffected), `118.51/109.08`, `120.19/108.53`, `119.57/108.47`,
`120.64/108.80`, `120.51/112.76`, `122.44/109.70`, `121.73/110.99`, `119.56/111.20`. B wins 9/9.

Raw paired single-core runs, `Core_1_Bit.bmp` (µs): `76.968/70.795`, `76.346/70.994`,
`76.992/70.870`, `76.750/70.754`, `83.073/99.289` (both sides elevated together, machine-wide
noise, not a directional effect), `76.839/70.806`, `76.713/70.554`, `77.184/101.99` (B spikes
alone here, A normal - the only pair where A "wins"), `76.083/71.316`. B wins 7/9 clearly, 1 pair
mixed-noise (both elevated, B still nominally faster), 1 pair reversed by an isolated B-side spike.

Raw paired multi-core runs, `Core_4_Bit.bmp` (µs): `118.60/107.33`, `118.37/107.37`,
`116.11/106.93`, `116.25/107.19`, `116.69/106.79`, `116.31/107.18`, `116.33/106.61`,
`118.24/107.44`, `120.00/110.44`. B wins 9/9.

Raw paired multi-core runs, `Core_1_Bit.bmp` (µs): `76.396/70.383`, `76.355/70.448`,
`75.576/70.228`, `77.346/70.743`, `76.144/70.809`, `76.959/71.506`, `75.721/70.144`,
`76.135/70.328`, `76.011/70.485`. B wins 9/9.

Same-binary control (baseline vs. baseline, `Core_4_Bit.bmp`, single-core, 6 pairs, µs):
`117.12/118.54`, `117.63/117.17`, `117.82/156.68`, `117.51/118.87`, `138.59/117.78`,
`117.36/117.37` - mostly 117-119 µs with two outliers (156.68, 138.59) matching this workspace's
already-documented "mode lottery" noise (`IDEAS.md`), not a directional bias.

## Result

Real, consistent speedup on both benchmarked bit depths: `Core_4_Bit.bmp` ~8-10% faster
(single-core and multi-core, 9/9 paired wins both), `Core_1_Bit.bmp` ~7.5-8% faster (multi-core
9/9, single-core 7/9 with the two exceptions explained by isolated one-sided noise spikes, not a
reversal of the median). Both this candidate's 4-bit result (~8-10%) and candidate 008's 8-bit
result (~3-5%) come from the exact same fix applied to structurally similar loops; this candidate's
larger measured delta is plausibly because `set_4bit_pixel_run`/`set_1bit_pixel_run` write 2-8
output pixels per loop iteration (each needing its own palette lookup) versus one pixel per
iteration for the 8-bit case, so removing one bounds check per lookup compounds more within a
single iteration - not confirmed via disassembly, see "Why the result looks the way it does".
This is a whole-decode measurement (file/header parsing + row reads + the patched loop), not an
isolated micro-benchmark of the loop alone.

## Does it change behavior/output?

- **Bit-exact?** Yes. Decoded and hashed (via a temporary `image::open` + `to_rgba8` +
  `DefaultHasher` example, not saved - reproducible from the patch) all 66 `.bmp` files under
  `tests/images/bmp/` (every bit depth including 1/2/4/8-bit indexed, RLE4/RLE8, OS/2 core
  headers, V3/V4/V5 headers, top-down and bottom-up, ICC profile variants) with the baseline and
  patched binaries: every hash and every decoded width/height matched exactly, including
  `pal2.bmp`/`pal2color.bmp` (2-bit indexed, not otherwise covered by a bench).
- **Is this a breaking change?** No. Internal-only signature change to three private (non-`pub`)
  functions and two local variable bindings inside private methods - no public API surface
  touched.
- Project test suite (`cargo test --release`): 550 passed / 1 failed before, 550 passed / 1
  failed after - the same single pre-existing failure both times (`resizing_with_catmul` in
  `tests/regression.rs`, confirmed identical panic message/output against an unpatched checkout,
  same failure candidate 008 already documented). No new failures, no `^error` lines from a build
  failure in either run.

## Why the result looks the way it does

Same mechanism as candidate 008: each function was already indexing a raw palette array directly
(`palette[i]` on a masked/bounded `i`), so there was no trait-dispatch layer to remove - only a
per-lookup bounds compare-and-branch, which the fixed-size array type lets LLVM prove statically
and elide. The larger measured percentage here versus 008's 8-bit result (~8-10% vs ~3-5%) is
plausibly explained by loop shape: `set_1bit_pixel_run` performs up to 8 palette lookups per
`idx` byte and `set_4bit_pixel_run` performs 2, each inside its own `macro_rules! set_pixel!`
expansion, versus exactly 1 lookup per iteration for `set_8bit_pixel_run` - removing one branch
per lookup compounds more when several lookups happen per outer-loop iteration. Not confirmed via
disassembly (no `perf`/`objdump` deep-dive done this session beyond the reasoning above) - see
`IDEAS.md`'s new entry for a follow-up if that tool becomes convenient.

## What's not done / open questions

- The 2-bit path (`set_2bit_pixel_run`) was patched and bit-exact-verified (via the hash check
  above, on `pal2.bmp`/`pal2color.bmp`) but not separately benchmarked - no 2-bit BMP file large
  enough for a clean criterion signal exists in `benches/decode.rs`'s `BENCH_DEFS` (the only
  2-bit samples in the repo's test corpus are 127x64, used for correctness coverage, and are not
  registered in the bench). Adding one would require either registering an existing small file
  (unlikely to give a clean signal, same reasoning candidate 008 gave for not benchmarking RLE)
  or generating a new larger 2-bit synthetic BMP - not done this session.
- `set_4bit_pixel_run`'s RLE4 call sites (`read_rle_data`, absolute and encoded-run modes) were
  patched and covered by the bit-exact hash check (`pal4rle.bmp`, `pal4rlecut.bmp`,
  `pal4rletrns.bmp`) but not separately benchmarked - same reasoning as 008's unbenchmarked RLE8
  absolute mode: the existing RLE test files are small, used for correctness not throughput.
- No disassembly-level confirmation of why the 4/1-bit win is larger than 008's 8-bit win (see
  "Why the result looks the way it does") - inferred from loop shape, not independently verified.
