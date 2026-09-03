# pic-scale-safe horizontal + floating-point convolution: extending runtime AVX2 multiversioning

Candidate 048 added runtime AVX2 multiversioning (`is_x86_feature_detected!("avx2")` selecting an
explicit `#[target_feature(enable = "avx2")]` variant at runtime) to only the vertical convolution
tap loop in `fixed_point_vertical.rs`, measuring -27% (vs. candidate 044's -41% whole-crate
`RUSTFLAGS="-C target-feature=+avx2"` rebuild), and explicitly named the untouched horizontal pass
(`fixed_point_horizontal.rs`) and the floating-point paths
(`floating_point_vertical.rs`/`floating_point_horizontal.rs`) as the main attributed cause of that
gap ("What's not done" in 048's `RESULT.md`, backlog row 58 in `IDEAS.md`). This candidate extends
the same runtime-multiversioning pattern to those three files, on top of 048's vertical fix.

## What was tried

For each of the six functions below, the body was extracted into a shared `#[inline(always)]`
generic impl function (no target-feature attribute of its own, so it inlines cleanly into either
caller and inherits whichever target features that caller has). A new
`#[cfg(target_arch = "x86_64")] #[target_feature(enable = "avx2")] unsafe fn ..._avx2` wrapper was
added alongside it, and the *original* function name became a thin dispatcher: it calls
`std::is_x86_feature_detected!("avx2")` once and branches to the AVX2 wrapper or the scalar impl,
so no call site anywhere else in the crate needed to change (unlike 048's vertical dispatcher,
which is called once per column-tier per row so the check is hoisted to the outer loop; these
functions are already called at row/tap-tier granularity, so checking inside each is negligible
next to the work it does):

- `fixed_point_horizontal.rs`: `convolve_row_handler_fixed_point` (1-row-at-a-time) and
  `convolve_row_handler_fixed_point_4` (4-rows-at-once)
- `floating_point_horizontal.rs`: `convolve_row_handler_floating_point` and
  `convolve_row_handler_floating_point_4`
- `floating_point_vertical.rs`: `convolve_column_handler_floating_point_4` (the 4-wide tap tier
  `column_handler_floating_point` calls for `total_width >= 16`; the 1-column tail,
  `convolve_column_handler_floating_point`, was left scalar-only, same convention 048 used for
  fixed-point's own 1-column tail)

`lib.rs`'s `#![forbid(unsafe_code)]` -> `#![deny(unsafe_code)]` relaxation (from 048) is unchanged;
each new `unsafe fn` carries its own `#[allow(unsafe_code)]`.

Reused candidate 040/044/048's `bench_resize_dynamic` criterion bench (`benches/imageops.rs`,
temporarily reapplied, reverted before finishing) and 048's `paired_pic_scale.sh` (copied into this
candidate's own `raw/`). Two binaries, each in its own `CARGO_TARGET_DIR`, both built with default
RUSTFLAGS:

- S: registry `pic-scale-safe` 0.1.11, completely unmodified (same baseline as 048)
- E: vendored `pic-scale-safe` with 048's vertical multiversioning *plus* this candidate's
  horizontal + floating-point multiversioning

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S: registry 0.1.11, default RUSTFLAGS (baseline) | 19.644 ms | 18.771 ms | Lanczos3, 4032x3024->400x300 (fixed-point path) |
| E: this candidate, default RUSTFLAGS | 11.781 ms (**-40.0%**, 5/5 E-faster) | 11.278 ms (**-39.9%**, 5/5 E-faster) | Lanczos3, same scenario |
| S (baseline) | 7.321 ms | n/a | Triangle, same source/dest, single-core only (fixed-point path) |
| E: this candidate | 5.572 ms (**-23.9%**, 5/5 E-faster) | n/a | Triangle |
| S (baseline) | 194.27 us | n/a | Lanczos3, 400x300->32x24 (small image, fixed-point path) |
| E: this candidate | 122.53 us (**-36.9%**, 5/5 E-faster) | n/a | Lanczos3, same small scenario |
| S (baseline) | 122.85 us | 123.23 us | CatmullRom, 400x300->32x24 (floating-point path, previously fully untouched by 044-048) |
| E: this candidate | 74.384 us (**-39.5%**, 5/5 E-faster) | 74.852 us (**-39.3%**, 5/5 E-faster) | CatmullRom, same scenario |
| S (baseline) | 119.05 us | n/a | Gaussian, 400x300->32x24, single-core only (floating-point path) |
| E: this candidate | 70.898 us (**-40.4%**, 5/5 E-faster) | n/a | Gaussian |
| S vs S (same-binary control) | 18.931 -> 18.977 us (+0.2%, 1/5 "wins") | n/a | noise floor for the Lanczos3 comparison above |

5 alternating paired runs per comparison via `raw/paired_pic_scale.sh`. `taskset -c 3` for
single-core; unpinned for multi-core (24 logical threads, AMD Ryzen 9 7900, `uptime` load
1.2-5.1 during this session, re-checked below noise-relevant threshold before each run). Raw logs:
`raw/singlecore_S_vs_E_Lanczos3.log`, `raw/singlecore_S_vs_E_Triangle.log`,
`raw/singlecore_S_vs_E_small_Lanczos3.log`, `raw/singlecore_S_vs_E_CatmullRom.log`,
`raw/singlecore_S_vs_E_Gaussian.log`, `raw/multicore_S_vs_E_Lanczos3.log`,
`raw/multicore_S_vs_E_CatmullRom.log`, `raw/control_singlecore_S_vs_S_Lanczos3.log`,
`raw/hashes_S_baseline.txt`, `raw/hashes_E_avx2_multiversioned.txt`,
`raw/hashes_E_forced_off_fallback.txt`, `raw/test_S_baseline.txt`, `raw/test_E_patched.txt`,
`raw/avx2_disasm_row_fixed_point_u8_channels4.txt`, `raw/avx2_disasm_column_floating_point_f32.txt`.

## Result

Extending runtime AVX2 multiversioning from the vertical-only scope of candidate 048 to also cover
the horizontal convolution pass and the floating-point vertical/horizontal paths gives
**-40.0% (single-core) / -39.9% (multi-core)** on the Lanczos3 4032x3024->400x300 scenario (up from
048's -27.0%/-26.9%, and now matching candidate 044's -41% whole-crate `RUSTFLAGS` rebuild to
within about a point), **-23.9%** on Triangle (up from 048's -14.5%), **-36.9%** on a small-image
Lanczos3 scenario (up from 048's -26.8%), and, on the floating-point path that 048 left completely
untouched, **-39.5%/-39.3%** on CatmullRom (single/multi-core) and **-40.4%** on Gaussian
(single-core). All paired comparisons were 5/5 E-faster, far outside the +0.2% same-binary noise
floor, and independently reproduced via `just test`'s single-shot criterion comparison (-39.9%,
p<0.05, same scenario). This closes essentially all of the gap 048 attributed to the horizontal
pass and floating-point paths being left scalar.

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified two ways, both via a temporary `examples/resize_hash_check.rs`
  (`DynamicImage::resize_exact` hashed with `DefaultHasher` over `to_rgb8().into_raw()`,
  `Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/`Nearest`, 8 destination sizes on a 4032x3024
  source and 3 on a 400x300 source, 55 hashes total - same corpus 048 used):
  1. S (unmodified registry crate) vs E (this candidate, `is_x86_feature_detected!` running for
     real on this AVX2-capable host) - byte-identical hashes across all 55 configurations
     (`raw/hashes_S_baseline.txt` vs `raw/hashes_E_avx2_multiversioned.txt`, diff clean).
  2. All four new `has_avx2`/`std::is_x86_feature_detected!("avx2")` checks (in
     `fixed_point_horizontal.rs` x2, `floating_point_horizontal.rs` x2,
     `floating_point_vertical.rs` x1 - 5 call sites total) were temporarily forced to `false`
     (`&& false` appended, reverted immediately after this check, not part of the shipped patch)
     to force every call through the scalar impl - same 55 hashes, still byte-identical to the S
     baseline (`raw/hashes_E_forced_off_fallback.txt`, diff clean). This directly verifies the
     non-AVX2 fallback path for all six newly-multiversioned functions, not just that the AVX2
     path happens to match the baseline.
  - `objdump` on the AVX2-built symbols confirms real 256-bit `ymm`-operand codegen: 21 `ymm` vs.
    62 `xmm` register-operand instructions in
    `convolve_row_handler_fixed_point_avx2::<u8, i32, 4>`
    (`raw/avx2_disasm_row_fixed_point_u8_channels4.txt`), and 7 `ymm` vs. 4 `xmm` in
    `convolve_column_handler_floating_point_4_avx2::<f32, f32, f32, 4>`
    (`raw/avx2_disasm_column_floating_point_f32.txt`) - both AVX2 paths are genuinely exercised
    during the correctness check above, not silently falling back.
- **Is this a breaking change?** Same two-part answer as 048:
  - The convolution output itself is unchanged (bit-exact, see above) - not a breaking change for
    any caller.
  - No further crate-level guarantee changes beyond what 048 already made
    (`#![forbid(unsafe_code)]` -> `#![deny(unsafe_code)]`, three call sites there). This candidate
    adds three more `#[allow(unsafe_code)]`-scoped `unsafe fn`s (six new AVX2 wrapper functions
    total, each individually allowed) but does not touch the crate-level lint again - the policy
    change 048 already made is what enables this candidate's approach too, not an additional one.
- Project test suite (`cargo test --release --no-fail-fast`), run against both the unmodified
  registry crate and this candidate's patched vendored crate, both with default RUSTFLAGS:
  **622 passed, 1 failed, 4 ignored, identical failing test in both builds**
  (`raw/test_S_baseline.txt`, `raw/test_E_patched.txt`). The one failure,
  `regression::resizing_with_catmul`, is the same pre-existing, unrelated bug documented in
  `___OTHER___/003.RESIZE_EXACT_CATMULLROM_OFF_BY_ONE_ROUNDING` (also the same failure 048 saw) -
  not introduced or affected by this patch.

## Why the result looks the way it does

Same mechanism as 048 (`#[target_feature(enable = "avx2")]` on a function whose only caller is
gated by a cached `is_x86_feature_detected!` check, giving LLVM 256-bit `ymm` codegen for that
function while the rest of the crate stays at the generic x86-64 baseline), applied to the
remaining hot code paths 048 left scalar. Reaching -40.0%/-39.9% - within about a point of
candidate 044's -41% blanket-`RUSTFLAGS` rebuild - confirms 048's own diagnosis: the -27%-vs.-41%
gap was attributable almost entirely to the horizontal pass and floating-point paths being
unwidened, not to any inherent ceiling on what runtime multiversioning (vs. a compile-time flag)
can achieve. The floating-point path result (CatmullRom -39.5%, Gaussian -40.4%) is a new data
point 048 didn't have at all, since 048 only benchmarked filters that route through the
fixed-point path (Lanczos3/Triangle) - it shows the same magnitude of improvement holds on a code
path that was completely scalar (no AVX2 codegen anywhere) before this candidate.

## What's not done / open questions

- **No AVX-512 multiversioning was attempted** - same open item 048 left; the same pattern used
  here (an `#[target_feature(enable = "avx512f,avx512bw,avx512vl")]` tier alongside the AVX2 one)
  would be the natural way to add it, but wasn't done.
- **AArch64/NEON was not touched** - same as 048; the existing `#[cfg(target_arch = "aarch64")]`
  tiers are unaffected.
- **The 1-column/1-channel scalar tail functions were left un-multiversioned** in both the
  fixed-point (048's own choice) and floating-point (this candidate's choice) vertical paths -
  these only run for the last few pixels of a row that don't divide evenly into the wider tiers, so
  the AVX2 setup overhead would likely dominate any gain there; not measured directly.
- **`resize_nearest.rs` (the `Nearest` filter's dedicated code path) was not touched** by either
  048 or this candidate - it never routes through `column_handler_fixed_point` or any of the
  functions multiversioned here (confirmed via the `Nearest` control case in the hash-check corpus
  behaving identically to before). A separate multiversioning pass would be needed if `Nearest`
  throughput matters.
- No patch was kept applied anywhere - the working tree was reverted to a clean state
  (`git checkout -- Cargo.toml benches/imageops.rs`, `rm -rf vendor/ target-e target-s`, temporary
  `examples/resize_hash_check.rs` deleted) before this file was written, per the workflow's
  standing rule.
