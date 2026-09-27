# BMP 2-bit palette lookup: bench corpus gap fix + direct measurement

Candidate 011 patched and bit-exact-verified `set_2bit_pixel_run` in `src/codecs/bmp/decoder.rs`
(the same `&[[u8; 3]; 256]` fixed-size-array fix as its 4/1-bit siblings) but could not measure it
directly: the repo's only 2-bit BMP samples (`pal2.bmp`/`pal2color.bmp`) are 127x64 and not
registered in `benches/decode.rs`'s `BENCH_DEFS`, too small for a clean criterion signal. This
candidate builds a larger synthetic 2-bit indexed BMP, registers it in the bench corpus, and
re-applies candidate 011's identical decoder fix to measure `set_2bit_pixel_run` on its own for
the first time.

A new 700x700, 2-bit indexed, standard-40-byte-header BMP (`tests/images/bmp/images/pal2_large.bmp`,
120 KiB, a 4-color palette with a repeating `(x+y) & 3` diagonal-stripe index pattern, bottom-up
row order matching the existing `pal2.bmp`) was generated and added to the bench corpus, registered
as a new entry in `benches/decode.rs`'s BMP `BENCH_DEFS` list. A matching reference PNG
(`tests/reference/bmp/images/pal2_large.bmp.png`, generated via `image::open` +
`PngEncoder::new_with_quality(_, CompressionType::Best, FilterType::Adaptive)` to keep it small -
4.3 KiB) was added so the existing `tests/reference_images.rs` harness covers it like every other
corpus file. The bench/corpus addition is bundled into this candidate's own `patch.diff` (same
convention as candidates 001/002/004/006 bundling their own new bench cases), alongside the
decoder fix itself, since the new file is needed to reach a measurable signal for this specific
function at all - see `README.md`'s "Instrumentation added for this workspace" for where this is
documented at the workspace level.

## What was tried

Two prebuilt `--release` `decode` criterion bench binaries (baseline: bench-corpus addition only,
no decoder fix; patched: bench-corpus addition + candidate 011's decoder fix), alternated run-by-
run per MEASUREMENT.md section 2b (`--sample-size 20 --measurement-time 0.3 --warm-up-time 0.15`,
9 alternating pairs per condition, filtered to `load-Bmp/pal2_large.bmp`), single-core
(`taskset -c 5`) and multi-core (unpinned), plus a same-binary control (baseline vs. itself, 6
pairs, single-core) to establish the noise floor. `uptime` load average was 2.95-8.85 during the
session (well under this machine's 24 cores; no re-baseline needed).

| Variant | single-core p50 (9 pairs) | multi-core p50 (9 pairs) | notes |
|---|---|---|---|
| baseline, `pal2_large.bmp` (700x700, 2-bit) | 258.80 µs | 259.00 µs | criterion's own [low mid high] middle value per run |
| patched, `pal2_large.bmp` | 238.49 µs (-7.9%) | 237.59 µs (-8.3%) | 9/9 paired wins each, single-core and multi-core |
| same-binary control (single-core, 6 pairs) | 256-259 µs typical, one outlier 271.07 | - | noise floor within ~0-5%; the measured effect (~8%) is well outside it |

Raw paired single-core runs (A=baseline, B=patched, µs): `257.16/236.54`, `259.44/239.99`,
`258.04/238.79`, `257.68/238.10`, `262.40/238.28`, `258.60/237.49`, `262.95/238.49`,
`258.80/239.23`, `259.69/239.50`. B wins 9/9.

Raw paired multi-core runs (µs): `258.22/237.59`, `259.54/237.50`, `258.94/237.91`,
`259.58/237.35`, `261.49/237.44`, `259.98/237.27`, `258.00/239.10`, `257.33/238.14`,
`259.00/238.40`. B wins 9/9.

Same-binary control (baseline vs. baseline, single-core, 6 pairs, µs): `257.17/257.28`,
`271.07/258.35`, `259.13/257.31`, `256.04/257.76`, `256.42/256.26`, `256.40/256.31` - mostly
256-259 µs with one mild outlier (271.07), not a directional bias.

## Result

Real, consistent speedup: ~7.9% faster single-core, ~8.3% faster multi-core, 9/9 paired wins in
both conditions, well outside the ~0-5% same-binary noise floor measured on this file/size. This
closes the gap `IDEAS.md` flagged: `set_2bit_pixel_run` now has its own direct, clean criterion
signal, in line with candidate 011's 4-bit (~8-10%) and 1-bit (~7.5-8%) results and consistent with
the same mechanism (eliding a per-lookup bounds check on a masked palette index once the palette
type's length, 256, is known to the compiler).

## Does it change behavior/output?

- **Bit-exact?** Yes. `image::load_from_memory_with_format` + `to_rgb8` + `DefaultHasher` on
  `pal2_large.bmp` gives an identical hash (`dc675d08c3831c2a`) built with and without the decoder
  patch (only the decoder.rs half of this candidate's diff toggled, bench-corpus addition held
  constant). Also verified indirectly via `tests/reference_images.rs`: `pal2_large.bmp` passes
  against its golden reference PNG both with and without the decoder patch applied.
- **Is this a breaking change?** No. Same internal-only signature change as candidate 011
  (private, non-`pub` functions/locals only); this candidate's own additions are a new test-data
  file, a new reference PNG, and one new bench-corpus entry - no source behavior change beyond
  011's already-characterized one.
- Project test suite (`cargo test --release`): 551 passed / 1 failed before (bench-corpus addition
  only, no decoder patch), 551 passed / 1 failed after (decoder patch applied on top) - the same
  single pre-existing failure both times (`resizing_with_catmul` in `tests/regression.rs`, the same
  failure candidates 007/008/011/016/019 already documented as pre-existing and unrelated). The new
  `tests/reference_images.rs` case for `pal2_large.bmp` passes in both runs (counted within the 551).

## Why the result looks the way it does

Same mechanism as candidate 011: `set_2bit_pixel_run` already masks its index (`(idx >> 6) & 0x3`,
etc.) before indexing the palette, so the fix removes a now-provably-unreachable bounds
compare-and-branch per lookup once the palette is typed as `&[[u8; 3]; 256]` instead of an unsized
`&[[u8; 3]]`. The measured ~8% here sits almost exactly between candidate 008's 8-bit result
(~3-5%, one palette lookup per iteration) and candidate 011's 4-bit result (~8-10%, two lookups per
iteration) - `set_2bit_pixel_run` unpacks 4 sub-indices per `idx` byte per its
`macro_rules! set_pixel!` loop body, more lookups per iteration than 4-bit but the same order of
magnitude effect as 4-bit rather than a further compounding beyond it. Not confirmed via
disassembly (no `perf`/`objdump` deep-dive this session, `perf` still not installed per
`README.md`) - consistent with, but not proof of, the same "more lookups per iteration compounds
more" explanation candidate 011 proposed.

## What's not done / open questions

- No disassembly-level confirmation of why 2-bit's win (~8%) doesn't clearly exceed 4-bit's
  (~8-10%) despite unpacking more sub-indices per byte (4 vs. 2) - same open item candidate 011
  already left for the 4-bit-vs-8-bit comparison, now also open for 2-bit-vs-4-bit.
- The new `pal2_large.bmp` uses a synthetic, highly regular diagonal-stripe index pattern (not a
  photographic/real-world palette distribution) - chosen for a clean, reproducible generator, not
  because it's representative of real 2-bit BMP content in the wild (2-bit indexed BMP is a rare
  format outside test corpora regardless).
- Did not sweep alternate sizes for `pal2_large.bmp` (e.g. confirming the ~8% figure holds at
  2x/4x this file's dimensions) - one size was measured, matching the level of rigor candidate 011
  applied to `Core_4_Bit.bmp`/`Core_1_Bit.bmp`.
