# pic-scale-safe vertical convolution: AVX-512 target-feature rebuild (measurement only)

`IDEAS.md` (2026-08-26) noted that candidate 044 only checked `-C target-feature=+avx2` for
`pic-scale-safe`'s vertical-convolution tap loop (`fixed_point_vertical.rs`), even though this
host also supports AVX-512 (`avx512f`/`avx512bw`/`avx512vl`/etc per `lscpu`). This candidate does
the same measurement-only check as 044, but with
`-C target-feature=+avx512f,+avx512bw,+avx512vl` instead of `+avx2`: no source code was changed
anywhere in `image` or `pic-scale-safe`, only the `RUSTFLAGS` used to build the same completely
unmodified registry crate (0.1.11). This is a build-configuration investigation, not a
source-level fix (see "Does it change behavior/output?" for why there is correctly no
`patch.diff`).

## What was tried

Reused candidate 044's `bench_resize_dynamic`-style criterion bench (a `avx512_check_resize`
group with Lanczos3/Triangle on a 4032x3024->400x300 resize, plus a 400x300->32x24 small-image
Lanczos3 case, temporarily appended to `benches/imageops.rs`, reverted before finishing) and
044's `paired_pic_scale.sh` paired-comparison script, unmodified. Three independent builds of the
unmodified registry `pic-scale-safe` 0.1.11 crate, each in its own `CARGO_TARGET_DIR`:

- **S: baseline** - default `RUSTFLAGS` (generic x86-64, same as every other candidate's builds)
- **E: AVX-512** - `RUSTFLAGS="-C target-feature=+avx512f,+avx512bw,+avx512vl"`
- **AVX2** - `RUSTFLAGS="-C target-feature=+avx2"` (candidate 044's flag, rebuilt fresh this
  session for a direct same-session AVX2-vs-AVX-512 comparison, not reusing 044's old binary)

**Disassembly** (`objdump -d -C -M intel --no-show-raw-insn`, `nm --demangle` to locate symbols,
same method as candidates 040/044) of
`pic_scale_safe::fixed_point_vertical::column_handler_fixed_point::<u8, i32>`:

- S (baseline): 118 `xmm`-operand instructions, zero `ymm`/`zmm` (`raw/base_col_u8_disasm.txt`) -
  matches candidates 040/044's finding for the default build.
- E (AVX-512): 9 `xmm`, 3 `ymm`, **32 `zmm`**-operand instructions (`raw/avx512_col_u8_disasm.txt`)
  - the compiler auto-vectorizes the same safe-Rust tap loop to 512-bit AVX-512 width once the
    target features are available, with no source change, going a step further than candidate
    044's AVX2 rebuild (which showed 62 `ymm`, zero `zmm`).

**Correctness (bit-exactness)** across all three configurations, via a temporary
`examples/resize_hash_check.rs` (not part of any permanent change) hashing `DynamicImage`s
resized with `resize_exact` (`to_rgb8().into_raw()`, `DefaultHasher`) across
`Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/`Nearest`, 8 destination sizes on a 4032x3024 source
and 3 on a 400x300 source (55 hashes total per build): S, E (AVX-512), and AVX2 all produced
byte-identical hash output (`raw/hashes_baseline.txt`, `raw/hashes_avx512.txt`,
`raw/hashes_avx2.txt`, pairwise `diff` clean).

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S: baseline, default RUSTFLAGS | 18.808 ms | 18.739 ms | Lanczos3, 4032x3024->400x300 |
| E: AVX-512 | 10.418 ms (**-44.6%**, 5/5 E-faster) | 10.400 ms (**-44.5%**, 5/5 E-faster) | Lanczos3, same scenario |
| S (baseline) | 7.384 ms | n/a | Triangle, same source/dest, single-core only |
| E: AVX-512 | 5.154 ms (**-30.2%**, 5/5 E-faster) | n/a | Triangle |
| S (baseline) | 219.66 us | n/a | Lanczos3, 400x300->32x24 (small image) |
| E: AVX-512 | 147.66 us (**-32.8%**, 5/5 E-faster) | n/a | Lanczos3, same small scenario |
| S vs S (same-binary control) | 18.884 -> 18.999 us (+0.6%, 2/5 "wins") | n/a | noise floor for the Lanczos3 comparison above |
| AVX2 (this session's fresh build) vs E: AVX-512 | 11.091 -> 10.422 ms (**-6.0%**, 5/5 AVX-512-faster) | n/a | Lanczos3, same scenario, both built this session for a direct comparison |

5 alternating paired runs per comparison via `raw/paired_pic_scale.sh` (candidate 044's script,
reused verbatim). `taskset -c 3` for single-core; unpinned for multi-core (24 logical cores,
`uptime` load ~0.1-1.7 during this session). This host's CPU supports AVX-512
(`avx512f`/`avx512bw`/`avx512vl`/`avx512dq`/`avx512cd`/`avx512vbmi`/etc per `lscpu`), so this
rebuild flag is guaranteed to run on the measuring machine; portability to AVX-512-lacking
hardware is exactly why this isn't proposed as a blanket source change (see below).

Raw logs under `raw/`: `singlecore_S_vs_E_Lanczos3.log`, `singlecore_S_vs_E_Triangle.log`,
`singlecore_S_vs_E_small_Lanczos3.log`, `multicore_S_vs_E_Lanczos3.log`,
`control_singlecore_S_vs_S_Lanczos3.log`, `singlecore_avx2_vs_avx512_Lanczos3.log`,
`base_col_u8_disasm.txt`, `avx512_col_u8_disasm.txt`, `hashes_baseline.txt`, `hashes_avx512.txt`,
`hashes_avx2.txt`, `test_S_baseline_noavx512.txt`, `test_E_avx512.txt`, `paired_pic_scale.sh`.

## Result

Confirms the `IDEAS.md` hypothesis: an AVX-512-width rebuild of the completely unmodified
`pic-scale-safe` 0.1.11 crate is **-44.6% (single-core) / -44.5% (multi-core)** on the Lanczos3
4032x3024->400x300 scenario, **-30.2%** on Triangle, and **-32.8%** on a small-image Lanczos3
scenario - all 5/5 paired runs faster, all far outside the +0.6% same-binary noise floor measured
on the same scenario. This is a **further win beyond AVX2's ~41-45%**: a direct, same-session
AVX2-vs-AVX-512 comparison (both freshly built this session, same scenario) shows AVX-512
**-6.0%** faster than AVX2, 5/5 paired runs faster - a real but much smaller marginal gain than
the ~41-45% jump from baseline to AVX2 in the first place, consistent with 512-bit `zmm` (16x
`i32` lanes) only doubling AVX2's 256-bit `ymm` (8x `i32` lanes) width, while AVX2 already doubled
baseline SSE2's 128-bit `xmm` (4x `i32` lanes) width - each successive doubling buys a smaller
absolute wall-clock improvement once non-vectorized setup/row-hop overhead becomes a larger share
of the remaining time. No AVX-512 downclocking regression was observed on this host/workload;
this candidate's answer to `IDEAS.md`'s open question is "further win," not "diminishing/negative
return," for this specific workload and CPU.

## Does it change behavior/output?

- **Bit-exact?** Yes, confirmed across all three configurations (S baseline, E AVX-512, and a
  freshly-built AVX2) - see "What was tried" above. Expected: same fixed-point integer arithmetic
  (`J = i32`), just executed with a wider SIMD width; a 512-bit AVX-512 add/multiply of the same
  operands in the same order produces the same per-lane result as the 128-bit SSE2 or 256-bit
  AVX2 versions, so bit-exactness is unsurprising and confirmed, not merely assumed.
- **Is this a breaking change?** N/A - **this candidate made no source change to `image` or
  `pic-scale-safe`**, only to the `RUSTFLAGS` used to build an otherwise-untouched checkout for
  measurement. There is correctly **no `patch.diff`** in this folder: nothing was fixed in the
  tested project's own source. The actionable finding is a build-configuration one (see "What's
  not done" below for why it isn't proposed as a blanket default), not a code change - per
  SKILL.md's "patch.diff is mandatory whenever the fix is a change to the tested project" section,
  a pure measurement/investigation with no source patch correctly has no `patch.diff`.
- Project test suite (`cargo test --release --no-fail-fast`), run both with default `RUSTFLAGS`
  and with `-C target-feature=+avx512f,+avx512bw,+avx512vl`, both against the completely
  unmodified registry crate (no vendoring): **622 passed, 1 failed, 4 ignored, identical in both
  builds** (`raw/test_S_baseline_noavx512.txt`, `raw/test_E_avx512.txt`). The one failure,
  `regression::resizing_with_catmul`, is the same pre-existing, unrelated bug documented in
  `___OTHER___/003.RESIZE_EXACT_CATMULLROM_OFF_BY_ONE_ROUNDING` and already confirmed unaffected
  by the `RUSTFLAGS` change in candidate 044 - not introduced or affected here either.

## Why the result looks the way it does

`-C target-feature=+avx512f,+avx512bw,+avx512vl` (like candidate 044's `+avx2`) lets LLVM's
auto-vectorizer widen the same safe-Rust integer tap loop with zero source changes - this time
from 256-bit AVX2 (`ymm`, 8x `i32` lanes) to 512-bit AVX-512 (`zmm`, 16x `i32` lanes), on top of
the same 128-bit-to-256-bit jump candidate 044 already measured. The crate never uses `std::arch`
intrinsics or `#[target_feature]` anywhere - it relies entirely on the default auto-vectorizer,
which only ever emits instructions legal for the crate's actual compile-time target features.
Since the registry crate is built against the generic x86-64 baseline for portability, this is
the same mechanism as 044, just carried one register-width step further. The marginal AVX2-to-
AVX-512 gain (-6.0%) is much smaller than the baseline-to-AVX2 gain (~41-45%) because non-
vectorized parts of the resize pipeline (row setup, dispatch, memory traffic for row hops) don't
share in a further width doubling, so they become a proportionally larger share of what's left to
speed up - a classic diminishing-returns shape for successive SIMD-width doublings on a mixed
workload, not evidence of AVX-512 downclocking (which would show up as a *negative* AVX2-vs-
AVX-512 delta, not observed here).

## What's not done / open questions

- **No source-level change is proposed here**, for the same reason as candidate 044: forcing
  `-C target-feature=+avx512f,+avx512bw,+avx512vl` as a blanket default for the whole `image`
  crate would illegal-instruction-crash on the (still large, in 2026) population of x86_64 CPUs
  without AVX-512 support - a real portability regression for a general-purpose, widely-deployed
  crate. This candidate stayed a measurement, confirming the AVX-512 data point `IDEAS.md` asked
  for, not proposing an unsafe default.
- This candidate did not attempt a **safe, runtime-detected** way to get this speedup shipped
  (e.g. `is_x86_feature_detected!("avx512f")` plus a `#[target_feature(enable = "avx512f")]`
  multiversioned variant of the tap loop) - candidate 044 already flagged this as a separate,
  larger, riskier follow-up (manual `unsafe fn` + intrinsics or a multiversioning macro, its own
  correctness/UB review) and this candidate's AVX2-vs-AVX-512 data point is additional evidence
  for prioritizing that follow-up (both AVX2 and now AVX-512 give large, real, bit-exact wins on
  this host), but building the multiversioning dispatch itself is still out of scope here.
- The AVX2-vs-AVX-512 comparison (-6.0%) had only one round of 5 paired runs on one scenario
  (Lanczos3, single-core), unlike the primary S-vs-E comparison which had 3 scenarios plus a
  same-binary noise-floor control - it is a real, 5/5-consistent result, but with a narrower
  evidence base than the headline baseline-vs-AVX-512 numbers above.
- AVX-512 downclocking (a known real-world tradeoff on some Intel client/consumer parts, less so
  on server parts) was not directly instrumented (no frequency counter sampling during the bench
  runs) - the conclusion "no regression observed" is based on wall-clock time only, on this one
  host's CPU. A different AVX-512-capable CPU model could show a different tradeoff; this result
  is host-specific like all of this workspace's other measurements.
- Whether downstream consumers who already build with `RUSTFLAGS="-C target-cpu=native"` on an
  AVX-512-capable host are already getting this speedup today with zero code changes was not
  verified beyond this measurement, same caveat as candidate 044.
- No patch was kept applied anywhere - the working tree was reverted to a clean state
  (`git checkout -- benches/imageops.rs`, `rm examples/resize_hash_check.rs`,
  `rm -rf target-avx2 target-avx512`) before this file was written, per the workflow's standing
  rule, regardless of the (informative, but non-source-patch) result.
