# idct_simd_and_scaled_decode

Two related discovery questions from IDEAS.md: (1) which `dequantize_and_idct_block_8x8` path
(scalar vs SSSE3/NEON) actually runs on this bench machine (AMD Ryzen 9 7900, x86_64), and how
that's decided; (2) whether the scalar-only reduced-size IDCT paths (`_4x4`/`_2x2`/`_1x1`, used by
`Decoder::scale()` for thumbnail-style decoding) are a large enough fraction of realistic
decode time to be worth SIMD-optimizing, and whether their scalar code has the same class of
inefficiency (redundant recomputation, missed hoisting) found elsewhere this session.

This is a pure investigation candidate - no patch was applied, nothing kept in the working tree.
**No `patch.diff` in this folder is correct, not a completeness gap**: no fix to the tested
project's own source was made (see SKILL.md's "patch.diff is mandatory... / The only two valid
states"). All source edits described below (temporary `eprintln!`/`Instant` probes in
`src/arch/mod.rs`, `src/idct.rs`, `src/lib.rs`) were reverted via `git checkout -- src/` before
this file was written; final `git status` in the project root shows nothing but
`___OPTIMIZATIONS___/` untracked. The two probes are saved separately as
`probe_q1_arch_detection.diff` (Q1's `eprintln!`) and `probe_q2_idct_timing.diff` (Q2's
`AtomicU64` counters) so this folder's `justfile` can reproduce both without Q1's per-call
`eprintln!` skewing Q2's timing numbers - see the justfile's own header comment.

## Question 1: which IDCT path runs, and how is it selected?

`src/arch/mod.rs::get_dequantize_and_idct_block_8x8()` uses **runtime** feature detection
(`is_x86_feature_detected!("ssse3")`, the `std`-provided macro, cached internally after first
check) on x86/x86_64 - not a compile-time `#[cfg(target_feature = ...)]` gate. `src/idct.rs`'s
`dequantize_and_idct_block_8x8` calls this once per 8x8 block and, when it returns `Some`, calls
the returned `unsafe fn` pointer (`arch::ssse3::dequantize_and_idct_block_8x8`) instead of the
scalar `dequantize_and_idct_block_8x8_inner`. This whole `arch` module (and the runtime-detection
call) only exists when the crate is **not** built with the `platform_independent` feature
(`#[cfg(not(feature = "platform_independent"))]` in both `src/lib.rs`'s `mod arch;` and
`src/idct.rs`'s call site) - the harness's default build (no explicit `--features`) does not set
this, so the dispatch code is compiled in.

Confirmed empirically, not just read: added a temporary `eprintln!` in both branches of
`get_dequantize_and_idct_block_8x8`'s `is_x86_feature_detected!` check, rebuilt
`___OPTIMIZATIONS___/harness/bench_decode` (`cargo +stable build --release`, no non-default
features), and ran it once on `benches/tower.jpg` (1 iter = 12288 blocks). Every single one of
the 12288 lines printed was `"CANDIDATE010_PROBE: ssse3 idct path selected"` - the scalar-fallback
branch never fired once. Cross-checked with `objdump -d` on the built binary: the
`jpeg_decoder[..]::arch::ssse3::dequantize_and_idct_block_8x8` symbol's disassembly contains real
SSE2/SSSE3 vector instructions (66x `movdqa`, 16x `movdqu`, 8x each of `punpckhdq`/`punpckhqdq`/
`punpckhwd`/`punpckldq`/`punpcklqdq`/`punpcklwd`) - the SIMD path is genuinely compiled and
genuinely executed on this machine's default build, not merely present-but-unused. The probe was
reverted (`git checkout -- src/arch/mod.rs`) immediately after this check.

As corroborating context (not the main question, but relevant to how much this matters): built a
second scratch harness copy against `jpeg-decoder` with `default-features = false, features =
["rayon", "platform_independent"]` (forces the scalar 8x8 fallback for every block, since the
whole `arch` module is compiled out under this feature) and compared full-scale decode of
`benches/large_image.jpg` (unpinned, 30 iters, single-run, not a full paired.sh batch - this is
context, not this candidate's headline measurement): SIMD (ssse3) min/mean = 29.0/30.2 ms vs.
scalar-only min/mean = 36.4/37.2 ms - roughly 20-25% slower without the SSSE3 path at full scale,
consistent with the arch dispatch mattering for real on this machine.

## Question 2: how much does scalar-only reduced-size IDCT cost, relative to total decode?

Public API found (per the task's step 1 instruction): `Decoder::scale(requested_width,
requested_height) -> Result<(u16, u16)>` (`src/decoder.rs:278`). It calls `read_info()`, then
`crate::idct::choose_idct_size(frame.image_size, requested_size)` to pick one of `{8, 4, 2, 1}`
(the closest scale that still covers the request), then `frame.update_idct_size(idct_size)`. The
existing `examples/decode.rs` does not use this API; it was exercised only via a temporary
`--scale W H` flag added to a **local scratch copy** of `bench_decode` (at `/tmp/bench_decode_scaled/`,
not committed anywhere in `___OPTIMIZATIONS___/harness/bench_decode/`, per the task's instruction
that this harness modification stays local/scratch unless every future candidate would want it -
this one is a narrow one-off for this investigation, not proposed as a permanent harness addition).

Measured `benches/large_image.jpg` (2268x1512, no chroma subsampling per the project's own notes)
at 4 requested sizes chosen to land on each of the 4 `choose_idct_size` scales: full (8/8, no
`--scale`), `--scale 1134 756` (4/8), `--scale 567 378` (2/8), `--scale 284 189` (1/8). Also added
a temporary instrumentation probe (`AtomicU64` nanosecond/call-count accumulators in
`src/idct.rs`'s `dequantize_and_idct_block`, exposed via `#[doc(hidden)] pub fn` in `src/lib.rs`,
all reverted afterward) to measure IDCT's own cumulative wall time and call count, separate from
total decode time:

| Requested scale | decode() min/mean (30 iters) | cumulative IDCT-wrapper nanos/iter | IDCT calls/iter |
|---|---|---|---|
| full (8/8, SIMD) | 29.96 / 34.34 ms | 6.08 ms | 161028 |
| 1/2 (4/8, scalar 4x4) | 18.97 / 20.11 ms | 5.39 ms | 161028 |
| 1/4 (2/8, scalar 2x2) | 13.73 / 14.22 ms | 3.08 ms | 161028 |
| 1/8 (1/8, scalar 1x1) | 12.61 / 13.42 ms | 2.96 ms | 161028 |

Call count is identical across all 4 scales (161028/iter) - expected, since entropy decode always
visits every coefficient block in the bitstream regardless of the requested output scale; only
the per-block IDCT computation and the final output/upsample/color-convert size shrink.

The "cumulative IDCT-wrapper nanos" column is dominated by measurement overhead at the smaller
scales, not real computation: calling `std::time::Instant::now()` twice (plus an atomic
`fetch_add`) 161028 times per iteration has a floor of its own, and the 2/8 and 1/8 rows converge
to ~3.0 ms almost regardless of how trivial that scale's actual arithmetic is (2x2 is 4 additions
+ a shift; 1x1 is one multiply-add + a shift) - this matches the project's own documented
"Instrumentation-overhead gotcha" in `projects/jpeg-decoder.md` (per-block `Instant` timers at
tens-of-thousands-of-calls-per-decode frequency measurably inflate results). Treating ~3.0 ms/iter
as this measurement's overhead floor: real IDCT compute is roughly 6.08-3.0 = ~3.1 ms at full
scale (SIMD 8x8, ~9% of the 34.3 ms mean), ~5.39-3.0 = ~2.4 ms at 1/2 scale (scalar 4x4, ~12% of
the 20.1 ms mean), and indistinguishable from zero at 1/4 and 1/8 scale (scalar 2x2/1x1, at most
low single-digit % of 13-14 ms mean, likely much less).

## Result

No patch attempted - this is what the investigation itself found:

- **Question 1**: confirmed, via a passing probe (12288/12288 SIMD-branch hits) and disassembly
  (real SSE2/SSSE3 instructions in the compiled function), that the default `cargo +stable build
  --release` on this machine runs the SSSE3 8x8 IDCT path via `is_x86_feature_detected!`, not the
  scalar fallback. SIMD vs. forced-scalar-8x8 comparison shows a real ~20-25% full-decode wall
  time difference on `large_image.jpg`, so this dispatch is not a no-op curiosity - it matters.
- **Question 2**: reduced-size (2/8, 1/8 scale) IDCT cost is at most low single digits of a
  percent of total decode wall time, likely close to zero once the ~3ms/iter instrumentation
  floor is subtracted - entropy decode (Huffman, unaffected by requested scale) dominates
  reduced-size decode almost entirely. 1/2 scale (4x4, still scalar-only) shows a more meaningful
  ~12% share, but its absolute value is small (~2.4ms out of ~20ms) and 1/2-scale requests are a
  less common thumbnail-generation choice than 1/4 or 1/8 in practice. No SIMD gap here looks
  worth chasing with the numbers actually measured.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source. All temporary instrumentation
(`eprintln!` probes in `src/arch/mod.rs`; `AtomicU64` counters in `src/idct.rs`; `#[doc(hidden)]
pub fn` accessors in `src/lib.rs`) was reverted via `git checkout -- src/` before this file was
written. `git status` in the project root shows nothing but `___OPTIMIZATIONS___/` untracked.
Project test suite was not re-run for this candidate (no patch to the real tree was ever left in
place to test) - the baseline 12 lib + 4 rayon + 2 doctest pass count recorded in
`projects/jpeg-decoder.md` for this same HEAD (`eb2d7c0`) stands unchanged.

## Why the result looks the way it does

Entropy (Huffman) decode cost is scale-invariant by construction - the JPEG bitstream must be
fully parsed (every AC run/category decoded) regardless of what `idct_size` the caller requests,
since coefficient positions can't be skipped without knowing the run-lengths that precede them.
IDCT cost, by contrast, shrinks steeply with scale: 8x8 does a real 2D separable IDCT with many
fixed-point multiplies (helped a lot by SSSE3), 4x4 does a related but ~4x-smaller-footprint
transform (still scalar), 2x2 is 4 additions, and 1x1 is a single multiply-add. So as scale drops,
the (already dominant) fixed entropy-decode cost becomes an even larger share of a shrinking
total, squeezing IDCT's contribution toward zero at 1/4 and 1/8 scale - exactly what was measured.
The `_4x4`/`_2x2`/`_1x1` scalar code itself (read against HEURISTICS.md's patterns) looks fine:
no allocations, no per-pixel redundant `Vec<Vec<_>>` double-indexing of the kind found and fixed
in the lossless path (candidates 005/006), and the handful of `stbi_f2f(<float literal>)` calls
that appear to run repeatedly inside each loop are pure functions of compile-time-constant
arguments that LLVM constant-folds away at this optimization level - there's no real per-call
recomputation to hoist.

## What's not done / open questions

- The reported IDCT-share percentages at full/1-2 scale (~9%/~12%) are still contaminated by the
  ~3ms/iter instrumentation floor established at 1/4-1/8 scale; a cleaner isolation (e.g. `perf
  stat -r N` cycle counting, not installed on this machine per `projects/jpeg-decoder.md`, or a
  coarser-grained timer wrapping whole scan-decode instead of per-block) would sharpen this, but
  the qualitative conclusion (reduced-size IDCT is not a meaningful fraction of reduced-size
  decode time) is robust to the exact percentage.
- Only one image (`benches/large_image.jpg`, no chroma subsampling) was used for the scale sweep;
  a subsampled real-world image might show a different balance since chroma blocks would shrink
  under IDCT scale reduction differently than luma - not checked.
- `--scale` was only exercised via a local scratch harness copy (`/tmp/bench_decode_scaled/`,
  discarded, not part of any `___OPTIMIZATIONS___/` artifact) per the task's instruction that this
  stays local unless a future candidate would want it permanently - if a future candidate wants to
  explore scaled decoding further, it will need to re-add a similar flag (the code above shows
  exactly what's needed: `dec.scale(w, h)` before `dec.decode()`).
- Not investigated: whether `UpsamplerGeneric`/color-convert cost (also scale-dependent, shrinks
  with output size) is itself a meaningful fraction of reduced-size decode time, separate from
  IDCT - plausible given IDCT's share drops to ~zero but decode time doesn't drop to zero either;
  left as an open lead below.
