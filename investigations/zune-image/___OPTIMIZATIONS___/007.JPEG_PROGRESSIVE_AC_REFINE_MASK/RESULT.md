# Progressive JPEG AC refinement: u64 initialized-coefficient bitmask (re-derivation of upstream PR #438)

`crates/zune-jpeg/src/bitstream.rs` (`BitStreamHuffman::decode_mcu_ac_refine` and
`decode_mcu_ac_first`), plus the buffer-layout plumbing needed to carry a per-block `u64` mask
alongside the coefficient storage (`decoder.rs`'s `progressive_mcus_buffer`/`progressive_scan_buffer`,
`mcu.rs`, `mcu_prog.rs`). This was flagged directly by the batch's own candidate list, citing
upstream PR [etemesi254/zune-image#438](https://github.com/etemesi254/zune-image/pull/438)
("Rewrite progressive AC refinement", by `197g`, opened 2026-08-31, still open/unmerged as of
this session) which claims ~1.7x on "some progressive benchmarks". `decode_mcu_ac_refine`'s inner
loop (`'advance_nonzero'` in this repo's base) repeatedly tests `*coefficient != 0` on individual
`i16` array elements (addressed via `UN_ZIGZAG`) to decide whether a bit belongs to an
already-initialized coefficient (read a correction bit) or a still-zero one (decrement a run
counter). The PR's idea: track "already initialized" as a `u64` bitmask in spectral (zigzag)
order, set once by `decode_mcu_ac_first`/`decode_mcu_ac_refine` themselves and carried forward
across scans in the decoder's own persistent buffers, so later refine scans test a cheap bit
instead of re-reading/re-deriving from the `i16` array each time, and can short-circuit into a
cheaper loop shape once nothing is left to initialize. It also replaces the EOB-run path's
`&block[1..] != &[0; 63]` linear scan (called out in this repo's own code comment as
perf-sensitive - "changing this to iter_any makes perf regress by 10%") with a mask-based check.

rustc 1.98.1 (48a229cea 2026-09-01), cargo 1.98.1 (797e8a9bc 2026-08-05). Worked in an isolated
git worktree (`___OPTIMIZATIONS___/.worktrees/007`, detached at `base_commit`), not the main
project tree.

## What was tried

Fetched the real upstream diff (`gh`/`curl` against
`https://patch-diff.githubusercontent.com/raw/etemesi254/zune-image/pull/438.diff`) and rebased it
onto this workspace's `base_commit` (`533296caf9a6783691d42b2dfbc623029216cbaf`), which has
diverged from the PR's own base (added cancellation support, `McuDecodeOutput`, per-row
checkpointing, etc. in between). `patch -p1` applied 19 of 25 hunks in `mcu_prog.rs` plus all of
`bitstream.rs`/`bitstream_arith.rs`/`decoder.rs` cleanly with only line-offset fuzz; the remaining
6 rejected hunks in `mcu_prog.rs` and 1 in `mcu.rs` were all trivial type-signature updates
(`[Vec<i16>; MAX_COMPONENTS]` -> `[ProgressiveBufferData; MAX_COMPONENTS]` on function parameters
whose bodies were otherwise untouched, plus one `let mut mcu_width;` -> `let mcu_width;`
unused-mut cleanup) that just needed re-applying against this repo's current parameter names
(`output: &mut McuDecodeOutput<'_, '_>` instead of the PR's `pixels: &mut [u8]`) - fixed by hand
with `sed`/`Edit`, not a re-derivation of the algorithm itself, which came through unmodified. The
resulting tree builds clean (only the two pre-existing `#![macro_use]` warnings, no new ones).

Bench harness: reused candidate 002's `bench_decode.rs` (per-iteration timing + fnv1a64 hash of
`JpegDecoder::decode()` output, unmodified, copied into this candidate's own `bench/`), pointed at
`test-images/jpeg/benchmarks/speed_bench_prog.jpg` (1.29 MB, progressive, 420 subsampling - the
project's own curated progressive speed-bench input). Machine: AMD Ryzen, 24 logical cores,
`uptime` load average 0.6-1.4 throughout (noisier at a couple of points, re-measured then).

| Variant | single-core p50 (`taskset -c2`) | single-core p50 (repeat) | multi-core p50 (unpinned) | notes |
|---|---|---|---|---|
| baseline | 245.9 ms | 244.7-247.3 ms (3 runs) | 245.9-246.0 ms (2 runs) | unconditional `*coefficient != 0` test per loop iteration |
| patched | 246.0-249.5 ms (3 runs) | - | 244.9-245.9 ms (2 runs) | `u64` initialized-coefficient bitmask, straightened loop |

Paired alternating comparison (`___OPTIMIZATIONS___/tools/paired.sh`, baseline vs. patched
binaries confirmed non-identical via `md5sum` first):

```
taskset -c2, RUNS=9,  iters=15: median 244.714 -> 245.572 ms (+0.4%), B faster in 2/9  -> NOT consistent
unpinned,    RUNS=9,  iters=15: median 243.477 -> 244.932 ms (+0.6%), B faster in 1/9  -> consistent (A, i.e. baseline, faster)
unpinned,    RUNS=9,  iters=15: median 243.804 -> 245.291 ms (+0.6%), B faster in 2/9  -> NOT consistent
taskset -c4, RUNS=15, iters=15: median 245.620 -> 245.919 ms (+0.1%), B faster in 4/15 -> NOT consistent
```

`perf record -F 997 --call-graph dwarf` (15 iterations, same input) self-time share of
`decode_mcu_ac_refine` out of total sampled cycles:

```
baseline: 11.23% self
patched:   9.75% self
```

## Result

**Time only.** No measurable whole-decode wall-clock win on this input: three of four paired
alternating comparisons came back "NOT consistent" (the medians are not evidence per
`paired.sh`'s own threshold), and the one that did classify as "consistent" showed the *baseline*
as the consistently faster side (+0.6%, patched slower in 8/9 pairs) - i.e. across the four
comparisons the signal is flat-to-very-slightly-against the patch, never a win. This lines up with
what `perf` shows: `decode_mcu_ac_refine`'s own self-time share of total decode cycles drops from
~11.2% to ~9.8% (a real, measurable ~13% relative reduction in that function's own cost), but that
function is only about a tenth of total decode cycles for this full end-to-end benchmark (which
also spends real time in Huffman DC-refine decode, IDCT, upsampling, and color conversion) - so
even a genuine win inside the function is arithmetically capped at roughly 1.5 percentage points
of total wall-clock time, which sits at or below this machine's own measured run-to-run noise
floor (baseline repeat p50s spread ~244-248 ms, ~1.6%, with zero source change). Upstream's own
claimed ~1.7x is plausible for a benchmark that isolates the entropy-decode/AC-refine stage in
particular (their PR description says "some progressive benchmarks" without specifying which), or
for an image/successive-approximation profile where AC refinement is a much larger share of total
work than it is for this project's own curated `speed_bench_prog.jpg`; it does not reproduce as a
whole-`decode()` win on this workspace's harness and this input.

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified across eight different progressive JPEGs, not just the primary
  bench input: `speed_bench_prog.jpg` (`569766d874657049`), `speed_bench_prog_h_sampling.jpg`
  (`d278d593a6d63674`), `speed_bench_prog_v_sampling.jpg`/`speed_bench_prog_hv_sampling.jpg`
  (`c7f20d0e0ea6af37`), `Kiara_limited_progressive_four_components.jpg` (`5d9c95b5ffae38e7`),
  `down_sampled_grayscale_prog.jpg` (`f388b3ad70e099b0`), `progressive_dc_huffman_table_1.jpg`
  (`84fd85727710ce10`), and `app14/progressive_huffman_rgb.jpg` /
  `app14/progressive_huffman_rgb_interleaved.jpg` (`bb2cbe5e5ab51325`) - every hash identical
  between baseline and patched binaries, stable across every iteration of every run. Specifically
  checked a restart-marker (DRI) progressive image as the task asked:
  `test-images/jpeg/progressive_restart_420.jpg` decodes to identical hash
  (`4402320069f11e8b`) baseline vs. patched.
- **Is this a breaking change?** No. Internal-only: a new private `mask: &mut u64` parameter on
  two private `BitStream` trait methods, and a new private `ProgressiveBufferData` struct
  (`{ mcu: Vec<i16>, init_mask: Vec<u64> }`) replacing the plain `Vec<i16>` the decoder used
  internally for `progressive_mcus_buffer`/`progressive_scan_buffer`. No public API, file format,
  or documented behavior change.
- Project test suite (`crates/zune-jpeg` only - the only crate whose source changed),
  `cargo test --release -p zune-jpeg --no-fail-fast`, run through the build/test lock, before and
  after:
  - **lib tests**: 13 passed / 1 failed, **identical before and after** - the one failure
    (`decoder::planar_layout_helpers::set_options_refreshes_cached_dispatch`) is the same
    pre-existing, unrelated failure already documented in candidate 002's `RESULT.md` (an
    `assert_eq!` on two independently-obtained function-pointer addresses, reproduces identically
    with the patch fully reverted).
  - **integration tests**: 148 passed / 0 failed / 1 ignored, **identical before and after**
    (9+0+18+2+23+1+46+7+11+10+8+13 across the 12 test binaries, same split both times).
  - Total: **161 passed / 1 failed / 1 ignored, identical set of passing/failing/ignored tests
    before and after** - the patch introduces no new failures and fixes none.

## Why the result looks the way it does

The mechanism is real (perf's own self-time attribution confirms `decode_mcu_ac_refine` got
measurably cheaper per call), but that function is a minority contributor to total decode cycles
for a realistic full-pipeline `decode()` call on this project's curated progressive bench image -
Huffman DC-refine decode (~5%), `finish_progressive_decoding` (IDCT/output assembly, ~6%), and
everything else (upsampling, color conversion, memory allocation/zeroing) account for the rest.
A ~13% relative improvement inside an ~11%-of-total function caps the theoretical whole-program
win at roughly 1.5 percentage points - below this machine's measured noise floor for this bench
(~1.6% spread across baseline repeats with zero source change). This is consistent with, not
contradictory to, upstream's own claimed 1.7x: that number most plausibly comes from a benchmark
that isolates the AC-refine/entropy-decode stage itself (or an image where successive
approximation makes that stage dominate total time), which is a different measurement question
than "does `JpegDecoder::decode()` get faster end-to-end for a typical progressive JPEG."

## What's not done / open questions

- Not benchmarked against an image deliberately constructed to make AC refinement dominate total
  decode time (e.g. many successive-approximation scans, large spectral bands, little DC/IDCT
  work relative to AC refinement) - such an image might reproduce upstream's claimed 1.7x where
  this project's own curated `speed_bench_prog.jpg` does not. Left for `IDEAS.md` if someone wants
  to chase the isolated-stage number specifically.
- Not measured for peak RSS/allocation count - the new `ProgressiveBufferData.init_mask: Vec<u64>`
  field is a real (small: one `u64` per coefficient block, i.e. 1/64th the size of the `i16`
  coefficient buffer it sits next to) permanent additional allocation per component per decode;
  not expected to be significant but not quantified here since the task was framed as a time
  investigation.
- The patch is a faithful rebase of upstream's own, still-open, unmerged PR - not independently
  redesigned. If upstream's PR is later merged or substantially revised, this candidate's own
  `patch.diff` should be treated as a point-in-time snapshot, not a live mirror.
