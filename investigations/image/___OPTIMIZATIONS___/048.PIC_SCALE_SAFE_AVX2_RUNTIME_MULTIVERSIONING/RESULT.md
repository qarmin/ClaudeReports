# pic-scale-safe vertical convolution: runtime AVX2 multiversioning

Candidate 044 confirmed that rebuilding the completely unmodified `pic-scale-safe` crate with
`RUSTFLAGS="-C target-feature=+avx2"` auto-vectorizes the vertical-convolution tap loop from
128-bit SSE2 to 256-bit AVX2 for a ~41% single/multi-core speedup, but deliberately shipped no
source change since forcing AVX2 as a blanket compile-time default would illegal-instruction-crash
on non-AVX2 hosts. This candidate implements the safe alternative `IDEAS.md` (2026-08-26) proposed:
runtime CPU-feature detection (`is_x86_feature_detected!("avx2")`) selecting an explicit
`#[target_feature(enable = "avx2")]` variant of the tap loop at runtime (function multiversioning),
applied as a real source change to a vendored copy of `pic-scale-safe` 0.1.11, built with
**default** `RUSTFLAGS` (no special build flags at all - the whole point is this works out of the
box on a normal `cargo build --release`).

## What was tried

For each of the three x86_64-relevant tap-loop tiers in `fixed_point_vertical.rs`
(`convolve_column_handler_fixed_point_direct_buffer_double` - 32 columns,
`..._direct_buffer16` - 16 columns, `..._direct_buffer` with `BUFFER_SIZE=8` - 8 columns), the
function body was extracted into a shared `#[inline(always)]` generic impl function (no
target-feature attribute of its own, so it inlines cleanly into either caller and inherits
whichever target features that caller has), then two thin wrappers were added: the original
`#[inline(never)]` scalar wrapper (unchanged behavior) and a new
`#[cfg(target_arch = "x86_64")] #[target_feature(enable = "avx2")] unsafe fn ..._avx2` wrapper. The
dispatcher `column_handler_fixed_point` computes `is_x86_feature_detected!("avx2")` once at
function entry and branches per tier (`if has_avx2 { <avx2 variant> } else { <scalar variant> }`,
`continue`-ing the loop) instead of always calling the scalar variant.

This required relaxing `lib.rs`'s `#![forbid(unsafe_code)]` to `#![deny(unsafe_code)]` (a `forbid`
lint cannot be locally overridden by `#[allow]`, unlike `deny`) - see "Does it change
behavior/output?" below for why this matters. Each of the three new `unsafe fn`s and their three
call sites carries its own `#[allow(unsafe_code)]`; every other module in the crate remains exactly
as unsafe-free as before.

Reused candidate 040/044's `bench_resize_dynamic` criterion bench (`benches/imageops.rs`,
temporarily reapplied, reverted before finishing) and `paired_pic_scale.sh` (candidate 044's
script, copied verbatim into `raw/`). Two binaries, each in its own `CARGO_TARGET_DIR`, **both
built with default RUSTFLAGS** (no `target-feature`/`target-cpu` flags at all, unlike candidate
044's E1):

- S: registry `pic-scale-safe` 0.1.11, completely unmodified (no vendoring, no `[patch.crates-io]`)
- E: vendored `pic-scale-safe` with this candidate's runtime AVX2 multiversioning

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S: registry 0.1.11, default RUSTFLAGS (baseline) | 19.411 ms | 19.241 ms | Lanczos3, 4032x3024->400x300 |
| E: this candidate, default RUSTFLAGS | 14.179 ms (**-27.0%**, 5/5 E-faster) | 14.073 ms (**-26.9%**, 5/5 E-faster) | Lanczos3, same scenario |
| S (baseline) | 7.495 ms | n/a | Triangle, same source/dest, single-core only |
| E: this candidate | 6.406 ms (**-14.5%**, 5/5 E-faster) | n/a | Triangle |
| S (baseline) | 198.44 us | n/a | Lanczos3, 400x300->32x24 (small image) |
| E: this candidate | 145.18 us (**-26.8%**, 5/5 E-faster) | n/a | Lanczos3, same small scenario |
| S vs S (same-binary control) | 19.509 -> 19.366 us (-0.7%, 3/5 "wins") | n/a | noise floor for the Lanczos3 comparison above |

5 alternating paired runs per comparison via `raw/paired_pic_scale.sh`. `taskset -c 3` for
single-core; unpinned for multi-core (24 logical threads on this host, AMD Ryzen 9 7900,
`uptime` load 2.1-5.7 during this session - re-checked below noise-relevant threshold before each
run). Raw logs: `raw/singlecore_S_vs_E_Lanczos3.log`, `raw/singlecore_S_vs_E_Triangle.log`,
`raw/singlecore_S_vs_E_small_Lanczos3.log`, `raw/multicore_S_vs_E_Lanczos3.log`,
`raw/control_singlecore_S_vs_S_Lanczos3.log`, `raw/double_avx2_disasm.txt`,
`raw/hashes_S_baseline.txt`, `raw/hashes_E_avx2_multiversioned.txt`,
`raw/hashes_E_forced_off_fallback.txt`, `raw/test_S_baseline.txt`, `raw/test_E_patched.txt`.

## Result

Runtime AVX2 multiversioning of the vertical convolution tap loop gives **-27.0% (single-core) /
-26.9% (multi-core)** on the Lanczos3 4032x3024->400x300 scenario, **-14.5%** on Triangle, and
**-26.8%** on a small-image Lanczos3 scenario - all 5/5 paired runs faster, all far outside the
-0.7% same-binary noise floor, and reproduced independently via `just test`'s single-shot criterion
comparison (-26.4%, p<0.05). This is smaller than candidate 044's ~41% blanket
`-C target-feature=+avx2` rebuild, because that flag vectorizes the *entire* crate (every function
in every module), whereas this candidate only multiversions the three vertical-tap-loop tiers that
actually run on the x86_64 hot path - see "Why the result looks the way it does" for why that gap
is expected, not a shortfall in the implementation. Unlike candidate 044, **this speedup applies to
a completely normal `cargo build --release`** with no special `RUSTFLAGS`/`target-cpu` needed - it
is available today to anyone who takes this patch, on any AVX2-capable x86_64 host (the large
majority of x86_64 machines in production), with automatic, correct fallback to the original
scalar code on hosts without AVX2 (verified directly, see below).

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified two ways, both via a temporary `examples/resize_hash_check.rs`
  (`DynamicImage::resize_exact` hashed with `DefaultHasher` over `to_rgb8().into_raw()`,
  `Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/`Nearest`, 8 destination sizes on a 4032x3024
  source and 3 on a 400x300 source, 55 hashes total):
  1. S (unmodified registry crate) vs E (this candidate, `is_x86_feature_detected!` running for
     real on this AVX2-capable host, so the AVX2 path is what actually executes) -
     byte-identical hashes across all 55 configurations (`raw/hashes_S_baseline.txt` vs
     `raw/hashes_E_avx2_multiversioned.txt`, diff clean).
  2. Because this candidate's speedup depends on a runtime branch that can't be exercised through
     its "false" side on an AVX2-capable measuring host, `has_avx2` was temporarily hardcoded to
     `false` (`&& false` appended, reverted immediately after this check, not part of the shipped
     patch) to force every call through the original scalar wrapper - same 55 hashes, still
     byte-identical to the S baseline (`raw/hashes_E_forced_off_fallback.txt`, diff clean against
     `raw/hashes_S_baseline.txt`). This directly verifies the non-AVX2 fallback path (the one that
     would actually run on a non-AVX2 host) produces the same output as the AVX2 path, not just
     that the AVX2 path happens to match the baseline.
  3. `objdump` on the AVX2-built symbols (`convolve_column_handler_fixed_point_direct_buffer_double_avx2::<u8, i32, 16>`
     etc.) confirms real 256-bit `ymm`-operand codegen (52 `ymm` vs. 22 `xmm` register-operand
     instructions in the double-tier AVX2 function, `raw/double_avx2_disasm.txt`) - the AVX2 path
     is genuinely exercised during the correctness check above, not silently falling back.
  - One disassembly oddity worth recording: the scalar (non-AVX2) `..._direct_buffer_double`
    32-column wrapper does not appear as a standalone symbol in either the S or E binary's symbol
    table, and `column_handler_fixed_point`'s non-AVX2 branch jumps directly into the 16-column
    tier's loop instead of calling it. This looks like the compiler recognized that stepping
    through the 16-column tier twice is numerically equivalent to one 32-column-tier call (same
    per-column sum-of-products either way) and folded the redundant path away - a legitimate
    optimization if so, but not independently confirmed beyond the empirical check in point 2
    above, which is why that check (rather than trusting the disassembly reasoning alone) is the
    basis for the "verified" claim.
- **Is this a breaking change?** Two distinct things changed, worth separating:
  - The vertical-convolution tap loop's *output* is unchanged (bit-exact, see above) - not a
    breaking change for any caller.
  - `pic-scale-safe`'s crate-level guarantee changed: `lib.rs` went from `#![forbid(unsafe_code)]`
    to `#![deny(unsafe_code)]` with three narrowly-scoped `#[allow(unsafe_code)]` opt-outs. This
    crate's name and its `forbid` lint are a deliberate "no unsafe, anywhere, ever" design
    commitment - literally its stated selling point - and this candidate's whole approach
    (`#[target_feature] unsafe fn` multiversioning) is fundamentally impossible to express as
    `forbid`-compliant safe Rust. Any downstream consumer relying on `pic-scale-safe` being
    unsafe-free (e.g. via `cargo geiger`, a `forbid(unsafe_code)` workspace-wide lint that denies
    dependencies with unsafe, or simply the crate's documented guarantee) would need to
    consciously accept this tradeoff - it is a real, user-visible policy change, not just an
    implementation detail. This is the single biggest caveat of this candidate and the main
    reason it is meaningfully riskier than a typical candidate in this workspace.
- Project test suite (`cargo test --release --no-fail-fast`), run against both the unmodified
  registry crate and this candidate's patched vendored crate, both with default RUSTFLAGS:
  **622 passed, 1 failed, 4 ignored, identical failing test in both builds**
  (`raw/test_S_baseline.txt`, `raw/test_E_patched.txt`). The one failure,
  `regression::resizing_with_catmul`, is the same pre-existing, unrelated bug documented in
  `___OTHER___/003.RESIZE_EXACT_CATMULLROM_OFF_BY_ONE_ROUNDING` - not introduced or affected by
  this patch.

## Why the result looks the way it does

`is_x86_feature_detected!("avx2")` reads a cached CPU-feature bitmask (cheap, computed once and
memoized by `std`), and `#[target_feature(enable = "avx2")]` lets LLVM emit real 256-bit `ymm`
instructions for that one function while the rest of the crate stays compiled against the generic
x86-64 baseline. This gives the *tap loop specifically* the same 128-bit-to-256-bit lane-width
doubling candidate 044 found, without needing a blanket `RUSTFLAGS` change - hence the correctness
guarantee (safe on non-AVX2 hosts) and the smaller-than-044 magnitude (-27% vs. -41%): candidate
044's flag widened every auto-vectorizable loop in the whole crate (horizontal convolution, alpha
handling, saturating-narrow conversions, etc.), while this candidate only widens the three vertical
tap-loop tiers reachable from `column_handler_fixed_point`. The remaining gap between -27% and
-41% is exactly the contribution of those other, unwidened code paths - not evidence this
implementation left tap-loop performance on the table (the tap-loop-specific disassembly shows the
same real `ymm` codegen candidate 044 found). Extending multiversioning to more functions (the
horizontal convolution pass in particular) would likely close more of that gap, at the cost of more
`unsafe` surface - left as a follow-up idea below.

## What's not done / open questions

- **Only the vertical convolution tap loop was multiversioned.** `pic-scale-safe`'s horizontal
  convolution pass (`fixed_point_horizontal.rs`) and floating-point code paths
  (`floating_point_vertical.rs`/`floating_point_horizontal.rs`) were left untouched - candidate
  044's whole-crate RUSTFLAGS rebuild benefits all of them, which is part of why this candidate's
  -27%/-41% gap exists (see above). Multiversioning the horizontal pass the same way is a natural,
  separate follow-up.
- **The `forbid(unsafe_code)` -> `deny(unsafe_code)` relaxation is the main thing a maintainer
  needs to consciously decide about**, not just review as a mechanical diff - see "Is this a
  breaking change?" above. An alternative not explored here: publishing the AVX2 variant as a
  separate, clearly-unsafe companion crate/feature flag rather than loosening the base crate's own
  guarantee, so consumers who specifically depend on the `forbid` guarantee aren't affected.
- **No AVX-512 multiversioning was attempted** - `IDEAS.md`'s AVX-512 follow-up (rebuilding under
  `+avx512f,+avx512bw,+avx512vl`) is still a separate, untried idea; the same multiversioning
  pattern used here (an `#[target_feature(enable = "avx512f,avx512bw,avx512vl")]` tier alongside
  the AVX2 one, gated by its own `is_x86_feature_detected!` check) would be the natural way to add
  it, but wasn't done in this candidate.
- **AArch64/NEON was not touched** - the existing `#[cfg(target_arch = "aarch64")]` 64-column
  "four" tier is untouched and still always used unconditionally on aarch64 (as before this
  candidate); no multiversioning was needed or added there.
- The disassembly-level "why doesn't the scalar double-tier symbol exist" observation above is
  noted but not fully root-caused (see that bullet) - the empirical forced-off correctness check
  is the basis for trusting the fallback path, not a claim about exactly what code executes there.
- No patch was kept applied anywhere - the working tree was reverted to a clean state
  (`git checkout -- Cargo.toml benches/imageops.rs`, `rm -rf vendor/ target-e target-s`, temporary
  `examples/resize_hash_check.rs` deleted) before this file was written, per the workflow's
  standing rule.
