<!--
Measurement-only investigation - no patch to the tested project's own source. See "Does it
change behavior/output?" below for the explicit N/A statement this workspace requires.
-->

# Row/iterator machinery isolation: does `ChunksExactMut` + `Option`-returning `.next()` + the outer per-row closure explain the BMP decode's residual?

Follow-up to candidate 046, answering the exact "What's not done" item it left open and the
top-of-backlog `IDEAS.md` entry it produced. Candidate 046 found store+lookup+unpack (all
deliberately branch-free, direct-pointer-store, const-generic-specialized to remove any dispatch
confound) still explains only 22.6-32.3% of real BMP decode time at 8/4/2/1-bit, leaving a
residual of 67.7-77.4%. The next never-yet-isolated component it named is the row/iterator
*machinery* around the real `set_Nbit_pixel_run` functions (`src/codecs/bmp/decoder.rs`):
`ChunksExactMut`, the `Option`-returning `pixel_iter.next()` calls (instead of a raw pointer
write), and the outer per-row closure `with_rows_resumable` invokes once per scanline (instead of
one flat loop over the whole image) - none of which candidate 046's (or 037/041/042's) synthetic
loop reproduces.

This is **not** a change to `image`'s own source - no code path in the real crate was touched,
patched, or built differently. It's a standalone measurement program (`bench/`, no dependency on
the `image` crate), extending candidate 046's harness with two new variants rather than starting a
new one, per this skill's "Bench harnesses: grow a few, don't multiply".

## What was tried

Base commit referenced (context only, not built/patched): `44ce9226c541dd2b11897b9ef07156a28871cdb7`
(same tree as candidates 037/041/042/046 - unmodified, `git status` confirmed clean before and
after this candidate).

Extended candidate 046's `bench/src/main.rs` (kept every prior function - `fill_store_natural`/
`fill_store_volatile`/`fill_lookup_natural`/`fill_lookup_volatile`/`fill_unpack_natural`/
`fill_unpack_volatile`/`fill_lookup_volatile_specialized`/`fill_unpack_volatile_specialized` -
unchanged, re-measured this session rather than reused from a prior session, per `MEASUREMENT.md`'s
paired-comparison discipline) with two new functions, both const-generic-specialized over `DENSITY`
like candidate 046's own `_specialized` pair (so the comparison against `unpack_volatile_specialized`
attributes only the added iterator/closure work, not a dispatch-specialization confound):

- `fill_unpack_volatile_specialized_chunksmut::<DENSITY>` - identical extraction+lookup to
  `fill_unpack_volatile_specialized`, but the store goes through a real
  `std::slice::ChunksExactMut<u32>` (chunk size 1, standing in for the real decoder's
  `row.chunks_exact_mut(num_channels)`) with an `Option`-returning `.next()` call, matched the
  same way `set_8bit_pixel_run`/`set_4bit_pixel_run`/`set_2bit_pixel_run`/`set_1bit_pixel_run`
  match it (`if let Some(pixel) = pixel_iter.next() { ... } else { <early exit> }`) - here the
  else-branch is `unreachable!()` since the harness's own row/density accounting guarantees the
  iterator never runs dry, but the branch is still emitted and real, not deleted by construction
  (confirmed by `objdump`, see below). Flat loop over the whole buffer, no per-row split - isolates
  the iterator-machinery cost alone.
- `fill_unpack_volatile_specialized_rowclosure::<DENSITY>` - the same extraction/lookup/
  `ChunksExactMut`+`Option` store as `_chunksmut`, but invoked once per "row" through a new
  `with_rows_like<F: FnMut(&mut [u32]) -> u32>` helper - a separate, `#[inline(never)]` function
  matching the real decoder's `with_rows_resumable` shape exactly (a distinct top-level function
  generic over the row-processing closure, slicing the buffer into rows and calling `func(row)`
  once per row) - so a fresh `ChunksExactMut` is constructed per row (mirroring
  `let mut pixel_iter = row.chunks_exact_mut(num_channels);` being called once per scanline in
  `read_palettized_pixel_data`), not once for the whole image.

Row width per size regime was chosen to divide the buffer's pixel count evenly at every density
(1/2/4/8) and to roughly match a real scanline width at that scale: `SMALL_ROW_WIDTH = 440`
(153,560 / 440 = 349 rows, order of magnitude of Core_8_Bit.bmp/Core_4_Bit.bmp's 435-pixel width);
`LARGE_ROW_WIDTH = 4032` (12,192,768 / 4032 = 3024 rows - literally this workspace's usual
4032x3024 "realistic photo" scale, so row count and width both match a real image exactly).

**Correctness of both new variants is verified on every run**, not just assumed: a new
`verify_iterator_variants_correctness()` function (alongside candidate 046's existing
`verify_unpack_correctness()`, both run unconditionally at binary startup) compares every output
pixel of `fill_unpack_volatile_specialized_chunksmut`/`fill_unpack_volatile_specialized_rowclosure`
against the same from-scratch reference extraction used by candidate 046, for all 4 densities, on a
3-row / 32-bytes-per-row test buffer (exercising the row-splitting accounting the `_rowclosure`
variant adds, not just a flat-buffer check). All four run logs (`evidence/{single,unpinned}_run{1,2}.txt`)
contain both `verify_unpack_correctness: OK` and `verify_iterator_variants_correctness: OK` lines.

Same methodology as candidates 041/042/046: 2 buffer-size regimes (`small` = 153,560 pixels
matching `Core_8_Bit.bmp`/`Core_4_Bit.bmp`'s 435x353 scale; `large` = 4032x3024 = 12,192,768
pixels), all 40 configs (4 densities x 10 variants) measured in true round-robin, 30 rounds, ~10%
warmup discarded, `min_ns` across 4 runs (2x `taskset -c 3`, 2x unpinned) as the stable metric.
`uptime` was 1.15-1.51 for all runs (well under the 24-core budget). Raw output:
`evidence/single_run{1,2}.txt`, `evidence/unpinned_run{1,2}.txt`, parsed best-min table in
`evidence/parsed_best_min.json`.

`objdump -d -C -M intel` (`evidence/objdump_full.txt`) confirms: neither new variant contains any
SIMD instructions (`grep -c "xmm\|ymm"` over each function's disassembly returns 0, same as
`unpack_volatile_specialized` itself - none of these functions auto-vectorize). `_chunksmut`/
`_rowclosure` do each retain a `core::panicking::panic` call (the `unreachable!()` else-arm of the
`Option` match on `pixel_iter.next()`) that `unpack_volatile_specialized`'s raw-pointer version has
no equivalent of - never executed given the harness's own accounting, but present in codegen the
same way candidate 046 found for its own runtime-density `unreachable!()` case. `with_rows_like`
additionally emits one `panic_const_div_by_zero` call from computing `buf.len() / row_width` inside
the function (`row_width` is a runtime, not const-generic, parameter, so LLVM can't prove it's
non-zero) - a one-time check outside the per-pixel hot loop, not present in the real decoder (which
receives `height` directly as a parameter rather than deriving a row count via division), noted as
a minor structural difference from the real code, not expected to affect the per-pixel timing.

Best-min ns/pixel per config (`small` scale, matching candidate 037's real-decode corpus scale):

| density | lookup_volatile_specialized | unpack_volatile_specialized | unpack_volatile_specialized_chunksmut | unpack_volatile_specialized_rowclosure |
|---|---|---|---|---|
| 1 | 0.1924 | 0.2267 | 0.3827 | 0.3910 |
| 2 | 0.1913 | 0.2561 | 0.2944 | 0.2978 |
| 4 | 0.1360 | 0.2578 | 0.3019 | 0.2984 |
| 8 | 0.1375 | 0.2604 | 0.3339 | 0.3429 |

(`large`-scale figures tell the same qualitative story - full table in
`evidence/parsed_best_min.json`; magnitudes match small-scale closely at every density, unlike
candidate 046's own `large`/`small` divergence at density 8.)

## Result

**Isolated fairly (both new variants const-generic-specialized like `unpack_volatile_specialized`,
so neither carries a dispatch-overhead confound), the row/iterator machinery's own added cost over
the direct-pointer-store floor is real and, unlike candidate 046's unpack-arithmetic finding, does
not grow monotonically with bit depth - it is largest at 8-bit (density 1) and smallest at 4-bit
(density 2):**

| Bit depth | Density | Real decode ns/px (candidate 037) | Unpack floor (046), specialized | Row/iterator floor, specialized (`_rowclosure`) | Iterator machinery's own added cost | Iterator machinery as % of real decode time |
|---|---|---|---|---|---|---|
| 8-bit | 1 | 1.0357 | 0.2267 (21.9%) | 0.3910 (37.8%) | +0.1643 | +15.9% |
| 4-bit | 2 | 1.0844 | 0.2561 (23.6%) | 0.2978 (27.5%) | +0.0417 | +3.8% |
| 2-bit | 4 | 0.8698 | 0.2578 (29.6%) | 0.2984 (34.3%) | +0.0406 | +4.7% |
| 1-bit | 8 | 0.8337 | 0.2604 (31.2%) | 0.3429 (41.1%) | +0.0825 | +9.9% |

The `_chunksmut` (flat-loop, no per-row split) and `_rowclosure` (per-row closure) numbers are
close to each other at every density (within 0.3-2.7% of one another) - the outer per-row closure
itself adds only a small, mostly-within-noise amount on top of `ChunksExactMut`+`Option` alone.
Nearly all of the row/iterator machinery's measured cost comes from the `ChunksExactMut` +
`Option`-returning `.next()` store mechanism itself, not from the per-row closure indirection.

Even with the row/iterator machinery included, `unpack + iterator machinery` together explain only
37.8% / 27.5% / 34.3% / 41.1% of real decode time at 8/4/2/1-bit respectively - a residual of
62.2% / 72.5% / 65.7% / 58.9% remains unaccounted for at every bit depth. This is a real
narrowing of the gap candidate 046 left open (its own residuals were 77.4% / 75.7% / 71.1% /
67.7%), but the majority of real decode time is still unexplained at every bit depth measured.

## Does it change behavior/output?

**Measurement-only investigation - N/A, no patch applied to the tested project's own source.**
`git status` in the main project's working tree was confirmed clean (only `___OPTIMIZATIONS___/`/
`___OTHER___/` untracked) both before and after this candidate - there is correctly **no
`patch.diff`** in this folder; see `SKILL.md`'s "`patch.diff` is mandatory whenever the fix is a
change to the tested project" section for why a measurement-only candidate like this one is one of
the two valid states with no patch.

- **Bit-exact?** N/A - no source change. The synthetic bench's own new-variant correctness is
  verified per-pixel against a from-scratch reference on every run (see above); all four runs
  printed both `verify_unpack_correctness: OK` and `verify_iterator_variants_correctness: OK`.
- **Is this a breaking change?** N/A - no source change.
- Project test suite (`cargo test --release`, unmodified `main` tree at the same commit as
  candidates 041/042/046, run once since there is no before/after to compare - per `SKILL.md`'s
  explicit exception for a pure investigation with no patch, this run was extra rigor, not a strict
  requirement): **550 passed / 1 failed**, the same single pre-existing failure every prior
  BMP-decoder-related candidate in this workspace has recorded (`resizing_with_catmul` in
  `tests/regression.rs` - a pre-existing, unrelated CatmullRom-filter regression test failure).
  Matches candidates 041/042/046's own 550/1 exactly (same tree, no fixture drift). Full log:
  `evidence/cargo_test.log`.

## Why the result looks the way it does

`ChunksExactMut`+`Option`-returning `.next()` (vs. a raw pointer write) is a real, measurable cost
- roughly 0.04-0.16 ns/pixel depending on density - but the size of that cost varies by density in
a way not simply proportional to how many `.next()` calls happen per input byte (which is exactly
`density` at every setting, same as the number of sub-pixel extractions). If the iterator's own
per-call overhead were constant, the added cost should track lockstep with `unpack_volatile_specialized`'s
own per-density shape; instead density 1 shows by far the largest jump (+0.164 ns/px, +15.9% of
real decode time) while density 2 shows the smallest (+0.042 ns/px, +3.8%) - a pattern that
doesn't fit a single simple mechanism. `objdump` symbol sizes (`nm --print-size`, captured in
`evidence/objdump_full.txt`) show `fill_unpack_volatile_specialized::<1>` (168 bytes) is unusually
large relative to its own neighbors (`<2>`=68, `<4>`=114, `<8>`=212 bytes) given how trivial its
loop body is (no shift/mask needed at density 1 - the byte *is* the index) - suggesting LLVM found
some additional optimization (partial unrolling or a different loop restructuring, though not SIMD
- no `xmm`/`ymm` instructions appear in any of these functions) specifically for the raw-pointer
density-1 case that the `ChunksExactMut`-based version does not get, which would make the *floor*
unusually fast at density 1 rather than the iterator version unusually slow - not confirmed by a
full instruction-level read of both functions side by side (see "What's not done" below).

The `_rowclosure` variant adding only marginally more than `_chunksmut` alone (0.3-2.7% at every
density, often within this session's own measurement noise) suggests the per-row closure
indirection itself - a separate `#[inline(never)]` function generic over `FnMut`, called once per
row rather than once for the whole buffer - is cheap relative to the per-pixel iterator machinery,
at least at these row counts (349/3024 rows). This is consistent with the real decoder's own
per-row closure being a comparatively rare event (once per scanline, not once per pixel) next to
the per-pixel `ChunksExactMut::next()` calls that dominate a row's own cost.

Combined with candidate 046, this narrows (but does not close) the gap: unpack arithmetic alone
explained +3.5% to +14.9% of real decode time (growing with bit depth); the row/iterator machinery
adds a further +3.8% to +15.9% (largest at 8-bit, not monotonic with bit depth). Together with
candidate 042's lookup floor, this whole 037/041/042/046/047 line has now isolated
store+lookup+unpack+iterator-machinery as explaining 27.5-41.1% of real decode time - still a
minority at every bit depth. What remains unmodeled: the bounds check itself (still never isolated
in any microbenchmark in this line, since 037/041/042/046/047 are all deliberately built without
one - see "What's not done" below), and I/O / buffer-fill overhead upstream of the per-pixel loop
(`read_scanline`'s own file reads, not touched by any microbenchmark in this line).

## What's not done / open questions

- The bounds check itself (the actual branch/panic-path overhead from `palette[idx as usize]`
  where `palette: &[[u8; 3]]` is an unsized slice, per candidates 008/011/025's own findings) is
  still never isolated in any microbenchmark in this line (037/041/042/046/047) - all are
  deliberately built branch-free via `get_unchecked`/fixed-size arrays. The only evidence on its
  cost remains candidate 037's real before/after wall-clock delta, indirect. This is now the sole
  remaining component named in candidate 046's own "What's not done" list that has no isolated
  microbenchmark of its own.
- The density-1-specific codegen difference between `unpack_volatile_specialized::<1>` (168 bytes,
  unusually large for its trivial loop body) and its neighbors was not fully explained by
  instruction-level reading - only inferred from symbol size and the absence of SIMD instructions.
  A side-by-side `objdump` read of `unpack_volatile_specialized::<1>` and
  `unpack_volatile_specialized_chunksmut::<1>`'s full disassembly (not just size/SIMD-presence)
  would confirm what specifically differs (partial unrolling, a different loop restructuring, or
  something else) rather than leaving it as a size-based inference.
- I/O / buffer-fill overhead upstream of the per-pixel loop (`read_scanline`'s own file reads,
  called once per row in the real decoder before `set_Nbit_pixel_run` runs) has no isolated
  microbenchmark in this line either - not attempted this session, and not named in candidate 046's
  own "What's not done" list (a new gap noticed while writing this candidate, added to `IDEAS.md`).
- As with candidates 041/042/046, the "real decode ns/pixel" column reuses candidate 037's own
  numbers (same repo/commit, different session) rather than being re-measured in the same session
  as this microbenchmark.
- Real hardware execution-port/IPC counters (`perf`) remain unavailable in this container (no
  `perf` binary, `sudo`/`apt` blocked) - the mechanistic explanations above are plausible and
  consistent with the measured shape, but not counter-confirmed.
- Only one machine/session was used for this candidate's own new measurements.
