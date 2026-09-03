# pic-scale-safe vertical convolution: runtime AVX-512 dispatch tier

Candidates 048/049 added runtime AVX2 multiversioning (`is_x86_feature_detected!("avx2")`
selecting an explicit `#[target_feature(enable = "avx2")]` variant at runtime, falling back to the
original scalar code) to `pic-scale-safe`'s vertical convolution, horizontal convolution, and
floating-point tap loops. Candidate 052 separately confirmed, via a whole-crate
`RUSTFLAGS="-C target-feature=+avx512f,+avx512bw,+avx512vl"` rebuild of the *unmodified* registry
crate, that AVX-512 gives a further real, bit-exact ~6.0% win over AVX2 on the same vertical tap
loop, with no downclocking regression on this host - but only via a global rebuild, not via actual
runtime dispatch. This candidate adds a third tier to 048/049's existing runtime cascade for the
vertical-convolution dispatcher (`column_handler_fixed_point` in `fixed_point_vertical.rs`):
`has_avx512` (checked first) -> `has_avx2` -> scalar, so AVX-512-capable hosts get the extra win
automatically while every other host keeps 048/049's already-validated AVX2/scalar behavior
unchanged. The horizontal and floating-point tiers 049 added were left AVX2/scalar-only (not
extended to AVX-512 in this candidate - see "What's not done" below).

## A build-configuration gotcha found while setting this up

`pic-scale-safe` published version **0.1.12** to crates.io since candidates 048/049/052 were
written (which vendored/measured against 0.1.11). The main `Cargo.toml`'s dependency line
(`pic-scale-safe = "0.1.5"`) resolves to the newest matching version, so `Cargo.lock` now pins
0.1.12 - and Cargo's `[patch.crates-io]` mechanism silently **does not apply** a path-patch whose
own `Cargo.toml` still declares `version = "0.1.11"` against that resolution (`cargo build -v`
prints `warning: patch \`pic-scale-safe v0.1.11 (...)\` was not used in the crate graph` and
quietly compiles the *unmodified* registry 0.1.12 crate instead - `Cargo.lock` also grows a second,
easy-to-miss `[[patch.unused]]` block recording this). This was caught only because the resulting
"S" and "E" benchmark binaries built byte-for-byte identical (`md5sum` match) and the "AVX-512"
binary's own disassembly showed zero `zmm`/`ymm` instructions in the dispatcher - i.e. **neither
build was actually using the vendored/patched crate at all**. The fix applied here: bump
`vendor/pic-scale-safe/Cargo.toml`'s `version` to `0.1.12` to match. `fixed_point_vertical.rs`
(the only file this candidate touches) is byte-identical between the 0.1.11 and 0.1.12 registry
tarballs (confirmed via `diff -rq` on the two extracted registry sources), so this version bump
has zero effect on 048/049's own already-shipped patch content, only on whether the patch actually
applies going forward. This is a durable, project-level gotcha (not specific to this candidate) -
recorded in `projects/image-rs.md` and flagged as a follow-up in `IDEAS.md` (048/049/052's own
patches/measurements should be spot-checked against current `HEAD`'s resolved `pic-scale-safe`
version next time they're touched).

## What was tried

In `fixed_point_vertical.rs`, for each of the three tap-loop tiers 048 already multiversioned for
AVX2 (`convolve_column_handler_fixed_point_direct_buffer` - 8 columns,
`..._direct_buffer16` - 16 columns, `..._direct_buffer_double` - 32 columns), a third wrapper was
added: `#[cfg(target_arch = "x86_64")] #[target_feature(enable = "avx512f,avx512bw,avx512vl")]
unsafe fn ..._avx512`, calling the same shared `#[inline(always)]` impl function 048 already
extracted (no target-feature attribute of its own, so it inlines cleanly and inherits whichever
target features the caller has - unchanged from 048). The dispatcher `column_handler_fixed_point`
now computes `has_avx512 = is_x86_feature_detected!("avx512f") &&
is_x86_feature_detected!("avx512bw") && is_x86_feature_detected!("avx512vl")` once at function
entry (alongside 048's existing `has_avx2` check) and, per tier, branches `if has_avx512 { ...avx512
variant } else if has_avx2 { ...avx2 variant } else { scalar variant }`.

Built on top of 048/049's combined patch (applied to a clean checkout of the pinned `base_commit`
below) rather than from scratch, since 052's own AVX2-vs-AVX-512 measurement was specifically on
this dispatcher. Reused candidate 040/044/048/049's `bench_resize_dynamic` criterion bench
(`benches/imageops.rs`, temporarily reapplied, reverted before finishing) and 048's
`paired_pic_scale.sh` script (copied verbatim into this candidate's own `raw/`). A temporary
`examples/resize_hash_check.rs` was rewritten from scratch this session (048/049's own copy was
never saved to their folders, since it isn't part of either's shipped patch) using the same
procedurally-generated source images `benches/imageops.rs` already uses (no external test-data
files needed) - deleted before the tree was reverted.

Two binaries, each in its own `CARGO_TARGET_DIR`, both built with **default RUSTFLAGS** (no
special `target-feature`/`target-cpu` flags):

- S: vendored `pic-scale-safe` with 048/049's AVX2/scalar-only dispatch (no AVX-512 tier) - this
  candidate's own baseline, since 048/049's AVX2 win is already an established, separate result
- E: same vendored crate, plus this candidate's AVX-512 tier added to the vertical dispatcher

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S: 048/049 AVX2/scalar dispatch (baseline) | 11.743 ms | 11.554 ms | Lanczos3, 4032x3024->400x300 |
| E: this candidate (+AVX-512 tier) | 11.358 ms (**-3.3%**, 5/5 E-faster) | 11.148 ms (**-3.5%**, 5/5 E-faster) | Lanczos3, same scenario |
| S (baseline) | 5.694 ms | n/a | Triangle, same source/dest, single-core only |
| E: this candidate | 5.519 ms (**-3.1%**, 4/5 E-faster) | n/a | Triangle |
| S (baseline) | 125.56 us | n/a | Lanczos3, 400x300->32x24 (small image) |
| E: this candidate | 122.79 us (**-2.2%**, 4/5 E-faster) | n/a | Lanczos3, same small scenario |
| S vs S (same-binary control) | 11.879 -> 11.929 us (+0.4%, 1/5 "wins") | n/a | noise floor for the Lanczos3 comparison above |

5 alternating paired runs per comparison via `raw/paired_pic_scale.sh` (048's script, reused
verbatim). `taskset -c 3` for single-core; unpinned for multi-core (24 logical threads, AMD Ryzen
9 7900, `uptime` load 1.2-4.6 during this session, re-checked below the core count before each
run). A single-shot `cargo bench` criterion comparison (same binary/target dir, S built and run
first via `taskset -c 3`, then E built and run the same way so criterion auto-compares against its
own saved S baseline) independently confirms the direction: **-4.2%** (p<0.05, "Performance has
improved") on a first attempt, **-5.1%** on a repeat (both `taskset`-pinned); an earlier *unpinned*
attempt at this same single-shot comparison showed no significant change (p=0.66) purely from
background system noise swamping a ~3% effect - included in "What's not done" below as a caveat on
this candidate's overall confidence, not as a contradicting result (the pinned, noise-floor-aware
paired comparisons above are the primary evidence).

Raw logs under `raw/`: `singlecore_S_vs_E_Lanczos3.log`, `singlecore_S_vs_E_Triangle.log`,
`singlecore_S_vs_E_small_Lanczos3.log`, `multicore_S_vs_E_Lanczos3.log`,
`control_singlecore_S_vs_S_Lanczos3.log`, `criterion_singleshot_S_to_E_pinned.log`,
`avx512_disasm_double_u8.txt`, `hashes_S_baseline.txt`, `hashes_E_avx512_multiversioned.txt`,
`hashes_E_forced_avx512_off_avx2_fallback.txt`, `hashes_E_forced_off_scalar_fallback.txt`,
`test_S_baseline.txt`, `test_E_patched.txt`, `paired_pic_scale.sh`.

## Result

Adding a third, AVX-512-gated tier to 048/049's existing runtime AVX2/scalar dispatch gives
**-3.3% (single-core) / -3.5% (multi-core)** on the Lanczos3 4032x3024->400x300 scenario on top of
048/049's own already-measured AVX2 win, **-3.1%** on Triangle, and **-2.2%** on a small-image
Lanczos3 scenario - 4-5 of 5 paired runs faster in every scenario, and a `taskset`-pinned
single-shot criterion comparison independently confirms the direction (-4.2%/-5.1%, p<0.05 both
times). This is real but far more modest than candidate 052's own **-6.0%** whole-crate-rebuild
AVX2-vs-AVX-512 data point, and closer in magnitude to the same-binary noise floor (+0.4%, 1/5
"wins") than any other win in this workspace's `pic-scale-safe` series - see "Why the result looks
the way it does" below for the most likely explanation (this candidate only widens the vertical
tap loop specifically, the same narrower-scope-than-052 gap 048 itself already documented relative
to 044's own whole-crate AVX2 rebuild).

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified three ways, all via the temporary `examples/resize_hash_check.rs`
  (`DynamicImage::resize_exact` hashed with `DefaultHasher` over `to_rgb8().into_raw()`,
  `Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/`Nearest`, 8 destination sizes on a 4032x3024
  source and 3 on a 400x300 source, 55 hashes total):
  1. S (048/049's AVX2/scalar dispatch) vs E (this candidate, `is_x86_feature_detected!` running
     for real on this AVX-512-capable host, so the new AVX-512 path is what actually executes) -
     byte-identical hashes across all 55 configurations (`raw/hashes_S_baseline.txt` vs
     `raw/hashes_E_avx512_multiversioned.txt`, diff clean).
  2. `has_avx512` forced to `false` (`&& false` appended, reverted immediately after this check,
     not part of the shipped patch) so every call falls through to 048/049's AVX2 path instead -
     same 55 hashes, still byte-identical to S (`raw/hashes_E_forced_avx512_off_avx2_fallback.txt`,
     diff clean). Verifies the AVX-512-to-AVX2 fallback edge of the new three-way cascade
     specifically, not just that the AVX-512 path happens to match.
  3. Both `has_avx512` and `has_avx2` forced to `false` (same temporary technique), forcing every
     call through the original scalar path - same 55 hashes, still byte-identical to S
     (`raw/hashes_E_forced_off_scalar_fallback.txt`, diff clean). Verifies the scalar fallback
     still works with the new tier's code present (not just removed, as 048/049 verified).
  - `objdump` on the AVX-512-built double-tier symbol
    (`convolve_column_handler_fixed_point_direct_buffer_double_avx512::<u8, i32, 16>`) confirms
    real 512-bit `zmm`-operand codegen (28 `zmm`-operand instructions, 0 `ymm`, 10 `xmm` in the
    disassembled excerpt, `raw/avx512_disasm_double_u8.txt`) - the AVX-512 path is genuinely
    exercised during the correctness check above, not silently falling back.
- **Is this a breaking change?** Same two-part answer as 048/049, no further change beyond what
  048 already made:
  - The convolution output itself is unchanged (bit-exact, see above) - not a breaking change for
    any caller.
  - No additional crate-level guarantee change: `lib.rs`'s `#![forbid(unsafe_code)]` ->
    `#![deny(unsafe_code)]` relaxation is 048's, carried forward unmodified here. This candidate
    adds three more `#[allow(unsafe_code)]`-scoped `unsafe fn`s (one per tap-loop tier), each
    individually allowed, on top of 048/049's own nine.
- Project test suite (`cargo test --release --no-fail-fast`), run against both S (048/049's
  vendored crate, no AVX-512 tier) and E (this candidate's patched vendored crate), both with
  default RUSTFLAGS: **622 passed, 1 failed, 4 ignored, identical failing test in both builds**
  (`raw/test_S_baseline.txt`, `raw/test_E_patched.txt`). The one failure,
  `regression::resizing_with_catmul`, is the same pre-existing, unrelated bug documented in
  `___OTHER___/003.RESIZE_EXACT_CATMULLROM_OFF_BY_ONE_ROUNDING` and already confirmed unaffected by
  048/049/052 - not introduced or affected by this patch either.

## Why the result looks the way it does

Same mechanism as 048/052 (`#[target_feature(enable = "avx512f,avx512bw,avx512vl")]` on a function
gated by a cached `is_x86_feature_detected!` check, giving LLVM 512-bit `zmm` codegen for that one
function while the rest of the crate stays at whatever the caller's own target features are), now
wired into runtime dispatch instead of a whole-crate `RUSTFLAGS` rebuild. The magnitude
(-3.3%/-3.5%) is noticeably smaller than 052's own whole-crate-rebuild AVX2-vs-AVX-512 delta
(-6.0%): 052 rebuilt the *entire* `pic-scale-safe` crate (including the horizontal convolution pass
and floating-point paths 049 separately multiversioned for AVX2 but this candidate did not extend
to AVX-512) under a single blanket flag, so every auto-vectorizable loop in the crate got the wider
lane width, not just the vertical tap loop this candidate touches. This is the same
narrower-scope-vs-whole-crate-rebuild gap 048's own `RESULT.md` documented relative to 044's
whole-crate AVX2 rebuild (-27% multiversioned-vertical-only vs. -41% whole-crate) - here the
*already-much-smaller* second-order AVX2-to-AVX-512 step (052's own -6.0%, itself smaller than the
baseline-to-AVX2 first-order jump) gets divided further by the same narrower-scope factor, landing
in a magnitude close enough to this measurement's own noise floor that the unpinned single-shot
criterion attempt failed to detect it at all (see "What was tried" above) even though the
`taskset`-pinned paired comparisons and a pinned single-shot criterion run both detected it
consistently.

## What's not done / open questions

- **Only the vertical convolution tap loop got an AVX-512 tier.** The horizontal convolution pass
  and floating-point paths 049 already multiversioned for AVX2 were left AVX2/scalar-only - adding
  an AVX-512 tier there too, the same way, is a natural follow-up and per "Why the result looks the
  way it does" above would likely close more of the gap to 052's own -6.0% whole-crate data point.
- **This candidate's effect size (-2 to -3.5%) is small enough to be noise-sensitive on a loaded
  host** - the unpinned single-shot criterion attempt failed to detect any change at all (p=0.66)
  purely from background load, even though the mechanism and the pinned measurements are real and
  consistent (5/5 or 4/5 paired wins in every scenario, well above the 0.4%/1-5 same-binary noise
  floor). A reader re-measuring this on a busier or different host should expect more
  run-to-run variance than 048/049/052's own much-larger effect sizes.
- **AVX-512 downclocking** was not directly instrumented here either (same caveat as 052) - "no
  regression observed" is wall-clock-only, on this one host's CPU (AMD Ryzen 9 7900, which per
  052 does not appear to downclock under this workload).
- **The `pic-scale-safe` 0.1.12 republish gotcha** (see above) means 048/049/052's own saved
  `patch.diff`/measurements should be spot-checked (`git apply --check`, and if it applies, a
  fresh `cargo build -v` grep for `patch \`pic-scale-safe\` was not used`) the next time any of
  them is revisited, since the registry crate could move again by then.
- **AArch64/NEON was not touched** - same as 048/049; unaffected.
- No patch was kept applied anywhere - the working tree was reverted to a clean state
  (`git checkout -- Cargo.toml benches/imageops.rs`, `rm -rf vendor/ target-e target-s
  target/criterion`, temporary `examples/resize_hash_check.rs` deleted) before this file was
  written, per the workflow's standing rule.
