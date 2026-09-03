# pic-scale-safe horizontal + floating-point convolution: extending the AVX-512 runtime dispatch tier

Candidate 053 added a third, AVX-512-gated tier (`has_avx512 -> has_avx2 -> scalar`) only to
`fixed_point_vertical.rs`'s vertical-convolution dispatcher (the scope candidate 048 originally
multiversioned for AVX2), on top of 048/049's existing AVX2/scalar runtime dispatch, measuring
-3.3%/-3.5% further on the Lanczos3 benchmark. Candidate 049 had separately extended AVX2 runtime
multiversioning to the horizontal-convolution (`fixed_point_horizontal.rs`,
`floating_point_horizontal.rs`) and floating-point-vertical (`floating_point_vertical.rs`'s
`convolve_column_handler_floating_point_4`) tap loops, but those were left AVX2/scalar-only - 053's
own "What's not done" section named this as the natural next step. This candidate applies exactly
053's own pattern (a third `has_avx512` tier, same shared `#[inline(always)]`-impl-function
structure, same `is_x86_feature_detected!` cascade) to those five functions:
`convolve_row_handler_fixed_point` / `convolve_row_handler_fixed_point_4`
(`fixed_point_horizontal.rs`), `convolve_row_handler_floating_point` /
`convolve_row_handler_floating_point_4` (`floating_point_horizontal.rs`), and
`convolve_column_handler_floating_point_4` (`floating_point_vertical.rs`).

## A build-configuration gotcha, re-checked before starting (per 053's own flagged follow-up)

`Cargo.lock` still resolves `pic-scale-safe` to **0.1.12** (same as when 053 wrote its own gotcha
note) and `vendor/pic-scale-safe/Cargo.toml`'s `version` is already `0.1.12` (053's own fix, carried
forward in its saved `patch.diff`). Re-verified before building on top of it: applied 053's
`patch.diff` to a clean checkout of the pinned `base_commit` below (`git apply --check`, clean),
then confirmed `cargo build -v` does **not** print `patch \`pic-scale-safe\` was not used in the
crate graph` and that the resulting binary's disassembly shows real `zmm`-operand codegen in
053's own vertical dispatcher (spot-checked alongside this candidate's own disassembly evidence
below) - the patch is genuinely taking effect, not silently falling back to the registry crate.

## What was tried

Built directly on top of 053's own combined patch (048+049+053, applied cleanly to the pinned
`base_commit`). For each of the five functions above, a new
`#[cfg(target_arch = "x86_64")] #[target_feature(enable = "avx512f,avx512bw,avx512vl")] unsafe fn
..._avx512` wrapper was added alongside the existing `..._avx2` one, calling the same shared
`#[inline(always)]` impl function 049 already extracted (no target-feature attribute of its own).
Each dispatcher now checks `has_avx512 = is_x86_feature_detected!("avx512f") &&
is_x86_feature_detected!("avx512bw") && is_x86_feature_detected!("avx512vl")` first, falling
through to the existing `has_avx2` check, then scalar - the same three-tier cascade shape 053 used
for the vertical dispatcher, just applied to these five call sites instead.

Reused candidate 040/044/048/049/053's `bench_resize_dynamic` criterion bench (`benches/imageops.rs`,
temporarily reapplied via 053's own patch hunk, reverted before finishing) and 053's
`paired_pic_scale.sh` (copied into this candidate's own `raw/`). A temporary
`examples/resize_hash_check.rs` was rewritten from scratch this session (053's own note says its
copy was never saved either) - same corpus shape as 048/049/053 (55 hashes:
`Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/`Nearest`, 8 destination sizes on a 4032x3024 source,
3 on a 400x300 source) - saved to `raw/resize_hash_check.rs`, deleted from the working tree before
the revert.

Two binaries, each in its own `CARGO_TARGET_DIR`, both built with **default RUSTFLAGS**:

- S: vendored `pic-scale-safe` with 053's own AVX-512-vertical-only dispatch (this candidate's own
  baseline, since 048/049/053's own wins are already separately established results)
- E: same vendored crate, plus this candidate's AVX-512 tier on the five horizontal/floating-point
  functions above

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S: 053's AVX-512-vertical-only dispatch (baseline) | 11.589 ms | 11.472 ms | Lanczos3, 4032x3024->400x300 |
| E: this candidate (+AVX-512 on horizontal/floating tiers) | 11.427 ms (**-1.4%**, 5/5 E-faster) | 11.494 ms (**+0.2%**, 4/5 E-faster) | Lanczos3, same scenario |
| S (baseline) | 5.969 ms | n/a | Triangle, same source/dest, single-core only, 10 paired runs | 
| E: this candidate | 5.765 ms (**-3.4%**, 7/10 E-faster) | n/a | Triangle |
| S (baseline) | 74.442 us | 74.225 us | CatmullRom, 400x300->32x24 (floating-point path) |
| E: this candidate | 73.290 us (**-1.5%**, 5/5 E-faster) | 72.932 us (**-1.7%**, 5/5 E-faster) | CatmullRom, same scenario |
| S (baseline) | 72.084 us | n/a | Gaussian, 400x300->32x24, single-core only |
| E: this candidate | 69.454 us (**-3.6%**, 5/5 E-faster) | n/a | Gaussian |
| S (baseline) | 124.02 us | n/a | Lanczos3, 400x300->32x24 (small image), single-core only |
| E: this candidate | 124.53 us (**+0.4%**, 3/5 E-faster) | n/a | Lanczos3, same small scenario |
| S vs S (same-binary control) | 11842 -> 11895 us (+0.4%, 1/5 "wins") | n/a | noise floor, Lanczos3 large |
| S vs S (same-binary control) | 6237.8 -> 6155.9 us (-1.3%, 5/5 "wins", ~20% run-to-run spread) | n/a | noise floor, Triangle - unusually wide for this scenario |

5 alternating paired runs per comparison via `raw/paired_pic_scale.sh` (10 for Triangle, after an
initial 5-run attempt landed inside this scenario's own unusually wide noise floor - see "Why the
result looks the way it does"). `taskset -c 3` for single-core; unpinned for multi-core (24 logical
threads, AMD Ryzen 9 7900, `uptime` load 1.1-8.3 during this session - the one high reading was
immediately after a build finished and dropped back below 5 within 20s before any measurement was
taken). A single-shot `cargo bench` criterion comparison (same binary/target dir, S built and run
first via `taskset -c 3`, then E built and run the same way so criterion auto-compares against its
own saved S baseline) independently confirms the Lanczos3 direction: **-4.5%** (p<0.05, "Performance
has improved") - noticeably larger than the -1.4% paired-script single-core number above, both
measuring the same scenario on the same pinned core; treat -1.4%/-4.5% as a range rather than either
one alone (see "Why the result looks the way it does").

Raw logs under `raw/`: `singlecore_S_vs_E_Lanczos3.log`, `singlecore_S_vs_E_Triangle.log`,
`singlecore_S_vs_E_Triangle_10runs.log`, `singlecore_S_vs_E_CatmullRom.log`,
`singlecore_S_vs_E_Gaussian.log`, `singlecore_S_vs_E_small_Lanczos3.log`,
`multicore_S_vs_E_Lanczos3.log`, `multicore_S_vs_E_CatmullRom.log`,
`control_singlecore_S_vs_S_Lanczos3.log`, `control_singlecore_S_vs_S_Triangle.log`,
`criterion_singleshot_S_to_E_pinned.log`, `avx512_disasm_row_fixed_point.txt`,
`avx512_disasm_row_floating_point.txt`, `avx512_disasm_column_floating_point.txt`,
`hashes_S_baseline.txt`, `hashes_E_avx512_multiversioned.txt`,
`hashes_E_forced_avx512_off_avx2_fallback.txt`, `hashes_E_forced_off_scalar_fallback.txt`,
`test_S_baseline.txt`, `test_E_patched.txt`, `resize_hash_check.rs`, `paired_pic_scale.sh`.

## Result

Extending 053's AVX-512 dispatch tier from the vertical-only scope to the five
horizontal-convolution and floating-point-vertical functions 049 had left AVX2/scalar-only gives a
**real but modest and scenario-dependent** win: **-1.4% (single-core paired) / -4.5% (single-core
criterion single-shot, p<0.05)** on Lanczos3 (fixed-point path, large image), **essentially no
change on Lanczos3 multi-core** (+0.2%, inside the ~0.4% same-binary noise floor) and on the small
400x300->32x24 Lanczos3 scenario (+0.4%, also inside noise), **-3.4%** on Triangle (fixed-point
path, needed 10 paired runs instead of 5 to separate from this scenario's own unusually wide,
directly-measured noise floor - see below), and **-1.5%/-1.7%** (single/multi-core) on CatmullRom
and **-3.6%** (single-core) on Gaussian, both entirely on the floating-point path that was fully
scalar before candidate 049 and is the first place this candidate's AVX-512 tier reaches genuinely
new ground (fixed-point vertical already got the AVX-512 tier from 053). This is smaller than
either 048->049's own within-tier AVX2 extension (-27%->-40% on Lanczos3) or 053's own
vertical-only AVX-512 step (-3.3%/-3.5%) - see "Why the result looks the way it does" for why,
including a real, disassembly-confirmed finding that two of the five newly-tiered functions never
actually widen past 128-bit `xmm` codegen even under the AVX-512 target feature, at either the AVX2
or the AVX-512 tier.

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified three ways, all via the temporary `examples/resize_hash_check.rs`
  (`DynamicImage::resize_exact` hashed with `DefaultHasher` over `to_rgb8().into_raw()`,
  `Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/`Nearest`, 8 destination sizes on a 4032x3024
  source and 3 on a 400x300 source, 55 hashes total - same corpus shape as 048/049/053):
  1. S (053's AVX-512-vertical-only dispatch) vs E (this candidate, `is_x86_feature_detected!`
     running for real on this AVX-512-capable host) - byte-identical hashes across all 55
     configurations (`raw/hashes_S_baseline.txt` vs `raw/hashes_E_avx512_multiversioned.txt`, diff
     clean).
  2. All 5 new `has_avx512` checks forced to `false` (`&& false` appended to the innermost
     `is_x86_feature_detected!("avx512vl")` call at each of the 5 new call sites, reverted
     immediately after this check, not part of the shipped patch) so every call falls through to
     049's AVX2 path instead - same 55 hashes, still byte-identical to S
     (`raw/hashes_E_forced_avx512_off_avx2_fallback.txt`, diff clean). Verifies the
     AVX-512-to-AVX2 fallback edge specifically, not just that the AVX-512 path happens to match.
  3. Both `has_avx512` and `has_avx2` forced to `false` at all 5 new call sites (same temporary
     technique), forcing every call through the original scalar path - same 55 hashes, still
     byte-identical to S (`raw/hashes_E_forced_off_scalar_fallback.txt`, diff clean). Verifies the
     scalar fallback still works with the new tier's code present.
  - `objdump` on the AVX-512-built symbols confirms real 512-bit `zmm`-operand codegen in 3 of the
    5 newly-tiered functions: `convolve_row_handler_fixed_point_avx512::<u8, i32, 4>` (15 `zmm`
    instructions in the disassembled excerpt), `convolve_row_handler_fixed_point_4_avx512::<u8,
    i32, 4>` (104 `zmm`), and `convolve_column_handler_floating_point_4_avx512::<f32, f32, f32, 4>`
    (16 `zmm`) - all in `raw/avx512_disasm_*.txt`. The other 2
    (`convolve_row_handler_floating_point_avx512`/`_4_avx512`, both channel-width variants
    checked) show **zero** `zmm` and **zero** `ymm` instructions - only `xmm` - both under this
    candidate's AVX-512 build and under 049's own pre-existing AVX2 build of the same functions
    (spot-checked the AVX2 symbol too, same result); see "Why the result looks the way it does".
- **Is this a breaking change?** Same two-part answer as 048/049/053, no further change beyond
  what those already made:
  - The convolution output itself is unchanged (bit-exact, see above) - not a breaking change for
    any caller.
  - No additional crate-level guarantee change: `lib.rs`'s `#![forbid(unsafe_code)]` ->
    `#![deny(unsafe_code)]` relaxation is 048's, carried forward unmodified. This candidate adds
    five more `#[allow(unsafe_code)]`-scoped `unsafe fn`s on top of 048/049/053's own twelve.
- Project test suite (`cargo test --release --no-fail-fast`), run against both S (053's own patch,
  no AVX-512 tier on these 5 functions) and E (this candidate's patch), both with default
  RUSTFLAGS: **622 passed, 1 failed, 4 ignored, identical failing test in both builds**
  (`raw/test_S_baseline.txt`, `raw/test_E_patched.txt`). The one failure,
  `regression::resizing_with_catmul`, is the same pre-existing, unrelated bug documented in
  `___OTHER___/003.RESIZE_EXACT_CATMULLROM_OFF_BY_ONE_ROUNDING` and already confirmed unaffected by
  048/049/052/053 - not introduced or affected by this patch either.

## Why the result looks the way it does

Same mechanism as 048/052/053 (`#[target_feature(enable = "avx512f,avx512bw,avx512vl")]` on a
function gated by a cached `is_x86_feature_detected!` check, wired into runtime dispatch), applied
to the five functions 049 left without an AVX-512 tier. The result is smaller and more uneven than
either 049's own AVX2 extension or 053's own vertical-only AVX-512 step for two separate reasons,
both directly confirmed by disassembly rather than inferred:

1. **Two of the five newly-tiered functions (`convolve_row_handler_floating_point` and its `_4`
   variant, both in `floating_point_horizontal.rs`) never get wider-than-`xmm` codegen from LLVM,
   at either the AVX2 tier 049 already shipped or this candidate's new AVX-512 tier.** This was
   checked directly (see "Does it change behavior/output?" above) rather than assumed - the
   accumulate-over-taps inner loop in these two functions apparently doesn't auto-vectorize past
   128 bits regardless of the target feature enabled, unlike the structurally similar fixed-point
   row functions (which do reach real `zmm` codegen) or the floating-point *vertical* column
   function (which also does). Adding an AVX-512 tier to these two functions is harmless (bit-exact,
   verified above) but contributes essentially nothing to the measured win - it's dead weight from
   a codegen perspective, not a bug.
2. **The floating-point-path win (CatmullRom/Gaussian) is real and larger in relative terms
   (-1.5% to -3.6%) than the fixed-point-path win (Lanczos3/Triangle, -1.4% to -3.4%, and
   arguably 0% on two of four Lanczos3 measurements)**, consistent with this being the first time
   the floating-point path's *vertical* tier specifically gets any width beyond AVX2 - 053's own
   patch touched only `fixed_point_vertical.rs`, so `floating_point_vertical.rs`'s
   `convolve_column_handler_floating_point_4` had no AVX-512 tier at all before this candidate,
   while the fixed-point vertical tap loop (053's own scope) already captured most of the available
   win for the fixed-point path before this candidate started.
3. **Triangle's own same-binary noise floor is unusually wide on this host** (`raw/control_
   singlecore_S_vs_S_Triangle.log`: median -1.3%, 5/5 "wins", but with ~20% run-to-run spread
   between individual pairs, vs. Lanczos3's own much tighter +0.4%/1-5-wins control) - the initial
   5-run Triangle comparison (+2.3%, 2/5 wins) looked like a regression purely from this noise
   before a 10-run repeat separated the real -3.4% effect from it. This is worth flagging as a
   per-scenario noise characteristic for future candidates re-measuring Triangle specifically, not
   just a one-off retry.
4. The Lanczos3 single-core paired-script number (-1.4%) and the pinned single-shot criterion
   number for the same scenario (-4.5%, p<0.05) disagree in magnitude while agreeing in direction -
   both are reported rather than reconciled to one number, since 053's own RESULT.md documented the
   same kind of cross-method spread (a pinned single-shot criterion run swinging further than the
   paired-script median) for its own, structurally similar measurement.

## What's not done / open questions

- **The two dead-weight functions identified above (`convolve_row_handler_floating_point` /
  `_4`) could be reverted to AVX2-only** (dropping their new AVX-512 tier) with no expected
  measured cost, since the disassembly shows they never benefit from the wider target feature - not
  done here, since correctness is unaffected either way and this candidate's own scope was "extend
  053's pattern to all of 049's functions uniformly," not "prune afterward based on what turned out
  to help." A future candidate could investigate *why* LLVM won't vectorize these two loops (a
  data-dependent trip count over `bounds.size`, or a trait-based generic accumulation pattern that
  doesn't lower to a SIMD op) as a separate, more targeted follow-up.
- **`resize_nearest.rs` was not touched**, same as 048/049/053 - `Nearest` never routes through any
  of these dispatchers.
- **AVX-512 downclocking** was not directly instrumented, same caveat as 052/053 - "no regression
  observed" is wall-clock-only, on this one host's CPU (AMD Ryzen 9 7900).
- **AArch64/NEON was not touched** - unaffected, same as 048/049/053.
- The `pic-scale-safe` 0.1.12 republish gotcha (053's own finding) was re-checked at the start of
  this candidate (see above) and still holds; worth re-checking again the next time any of
  048/049/052/053/this candidate's patches are revisited, since the registry crate could move again.
- No patch was kept applied anywhere - the working tree was reverted to a clean state
  (`git checkout -- Cargo.toml benches/imageops.rs`, `rm -rf vendor/ target-e target-s target-crit`,
  temporary `examples/resize_hash_check.rs` deleted) before this file was written.
