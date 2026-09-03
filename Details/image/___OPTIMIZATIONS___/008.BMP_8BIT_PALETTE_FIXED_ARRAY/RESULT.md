# BMP 8-bit palette lookup: fixed-size array to elide the bounds check

`src/codecs/bmp/decoder.rs`'s `set_8bit_pixel_run` (the per-pixel-index-to-RGB expansion loop for
8-bit indexed BMPs, called once per row from both `read_palettized_pixel_data`'s plain path and
`read_rle_data`'s RLE8 absolute-mode path) indexes `palette[*idx as usize]` where `idx: u8`. The
palette is always allocated as exactly 256 entries (`read_palette`'s comment: "Allocate 256
entries even if palette_size is smaller, to prevent corrupt files from causing an out-of-bounds
array access"), but it was passed around as a `&[[u8; 3]]` slice (length not known to the
compiler), so `palette[*idx as usize]` still carried a runtime bounds check on every pixel even
though a `u8` index into a 256-entry array can never be out of range. Binding the palette as a
`&[[u8; 3]; 256]` (fixed-size array reference, via `<&[[u8; 3]; 256]>::try_from(slice)`) once per
row/call, instead of as `&[[u8; 3]]`, lets the compiler statically prove `idx as usize < 256`
always holds and elide the bounds check inside the hot loop. Only the 8-bit path was changed (the
suggested starting point in `IDEAS.md`'s entry for this candidate); `set_4bit_pixel_run`/
`set_2bit_pixel_run`/`set_1bit_pixel_run` mask their index to 4/2/1 bits before indexing and were
left untouched (one fix per candidate).

## What was tried

Measured via the existing `benches/decode.rs` criterion harness, filtered to
`load-Bmp/Core_8_Bit.bmp` (435x353, 8-bit indexed, the largest 8-bit sample already in the repo's
BMP test corpus and already registered in `decode.rs`'s `BENCH_DEFS` - no new bench needed). Two
prebuilt `--release` `decode` bench binaries (baseline vs. patched), alternated run-by-run per
MEASUREMENT.md section 2b (`taskset -c 3`, `--measurement-time 1.5 --warm-up-time 0.3`,
9 alternating pairs), plus an unpinned (multi-core, realistic wall-clock) pass of the same shape,
plus a same-binary control (baseline vs. itself, 6 pairs) to establish the noise floor.

| Variant | single-core p50 (9 pairs) | multi-core p50 (9 pairs) | notes |
|---|---|---|---|
| baseline | 103.26 µs | 102.08 µs | `load-Bmp/Core_8_Bit.bmp`, criterion's own [low mean high] middle value per run |
| patched (fixed-size palette array) | 97.90 µs (-5.2%) | 98.98 µs (-3.0%) | 8/9 paired wins each (single-core and multi-core) |
| same-binary control (baseline vs. baseline, single-core, 6 pairs) | 102-109 µs range | 102-104 µs range | noise floor is within ~1-2% for most pairs, one ~5% outlier; smaller than the measured effect |

Raw paired single-core runs (A=baseline, B=patched, µs): `102.12/97.250`, `105.38/101.10`,
`103.29/97.900`, `104.02/98.365`, `103.26/97.206`, `103.25/97.610`, `103.63/97.063`,
`101.85/103.40` (only pair where A won), `103.05/101.04`.

Raw paired unpinned runs (µs): `103.68/96.943`, `101.60/96.823`, `102.08/99.867`,
`102.80/99.289`, `101.52/97.013`, `103.70/99.464`, `103.68/98.856`, `101.42/98.984`,
`101.48/124.53` (only pair where A won - this B value is itself an outlier, well above every
other B value in the run, consistent with the machine's known bimodal scheduling noise documented
in this workspace's `README.md`/`MEASUREMENT.md`, not a real regression in that one run).

## Result

Real but modest speedup: ~5% faster single-core (`taskset -c 3`), ~3% faster unpinned
(multi-core/realistic wall-clock), both 8/9 paired wins, on the one BMP file in the repo's test
corpus large enough to give a clean per-pixel-loop signal (435x353, 8-bit indexed,
`Core_8_Bit.bmp`). This is a whole-decode measurement (file/header parsing + row reads + the
patched loop), not an isolated micro-benchmark of the loop alone, so the loop's own speedup is
larger than 3-5% but diluted by the fixed per-call overhead outside it. The effect is bigger than
the same-binary noise floor (control pairs varied ~1-2%, one outlier ~5%) but the two are close
enough that this should be read as a confirmed small win, not a dramatic one - consistent with
`IDEAS.md`'s original "lower confidence" framing for this candidate.

## Does it change behavior/output?

- **Bit-exact?** Yes. Decoded and hashed (via a temporary `image::open` + `to_rgba8` +
  `DefaultHasher` example, not saved - reproducible from the patch) all 66 `.bmp` files under
  `tests/images/bmp/` (every bit depth, RLE4/RLE8, OS/2 core headers, V3/V4/V5 headers, top-down
  and bottom-up, ICC profile variants) with the baseline and patched binaries: every hash and
  every decoded width/height matched exactly.
- **Is this a breaking change?** No. Internal-only signature change to two private (non-`pub`)
  functions (`set_8bit_pixel_run`'s `palette` parameter type, and the two call sites' local
  `palette`/`p` bindings) - no public API surface touched.
- Project test suite (`cargo test --release`): 550 passed / 1 failed before, 550 passed / 1
  failed after - the same single failure both times (`resizing_with_catmul` in
  `tests/regression.rs`, confirmed pre-existing by running it against an unpatched `git stash`
  checkout: identical panic message/output). No new failures, no `^error` lines from a build
  failure in either run.

## Why the result looks the way it does

The palette-lookup itself (`palette[*idx as usize]`) is a single 3-byte load per pixel; removing
its bounds check removes one compare-and-branch per pixel from the loop, which is a small but
real per-pixel saving. It doesn't approach the size of the wins measured for candidates 001/004
in this workspace (which removed a much heavier cost: generic-trait `get_pixel`/`put_pixel`
dispatch) because this loop was already indexing a raw `&mut [u8]` slice directly - there was no
trait-dispatch layer to remove here, only one branch per iteration, matching `IDEAS.md`'s original
"lower confidence" note that BMP's indexed-expansion loop is a scalar table lookup with limited
headroom, not a case of "generic dispatch masquerading as a hot loop" like the affine/sample.rs
candidates.

## What's not done / open questions

- `set_4bit_pixel_run`/`set_2bit_pixel_run`/`set_1bit_pixel_run` were not converted to the same
  fixed-size-array pattern - their indices are masked to 4/2/1 bits before indexing (`idx & 0xf`,
  `(idx >> 6) & 0x3`, etc.), so the compiler may already be able to prove those in-bounds against
  a `&[[u8; 3]; 16]`/`&[[u8; 3]; 4]`/`&[[u8; 3]; 2]`-sized slice without needing the array-type
  change - not measured this session (see `IDEAS.md`'s new entry).
- Only one real-world test file (`Core_8_Bit.bmp`, 435x353) was large enough in this repo's
  existing BMP corpus to give a clean criterion signal; a synthetic much-larger 8-bit BMP (e.g.
  matching this workspace's usual 4032x3024 scale) was not built - the existing file's ~100 µs
  per-decode time was already precise enough (criterion's own spread was under 1% within a single
  binary's 100-sample run) that a bigger synthetic file was not judged necessary, but would give a
  cleaner separation between "loop cost" and "fixed per-call overhead" if someone wants a tighter
  number later.
- The RLE8 absolute-mode call site (`read_rle_data`, using `pal8rle.bmp`/similar RLE test files)
  was patched and hash-verified for correctness but not separately benchmarked - the existing BMP
  bench corpus's RLE files are tiny (used for correctness coverage, not throughput) and did not
  seem worth a dedicated measurement pass for this candidate.
