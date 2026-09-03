# pic-scale-safe vertical convolution: AVX2 target-feature rebuild (measurement only)

Candidate 040's disassembly found only 128-bit SSE2 `xmm` ops (`movdqa`/`pmaddwd`/`paddd`) in
`fixed_point_vertical.rs`'s vertical-convolution tap loop, even though candidates 039/040 assumed
`BUFFER_SIZE=8` for `J=i32` was "one AVX2 register" - suggesting the crate is compiled against the
generic x86-64 baseline with no AVX2 codegen for this loop. `IDEAS.md` (2026-08-26) proposed
rebuilding the *same, unmodified* crate (both the shipping "double" 32-column tier and candidate
040's unmerged "octo" 64-column tier) with `RUSTFLAGS="-C target-feature=+avx2"` to check whether a
wider vector width changes the register-budget picture, before trying more source-level tier
variants. This candidate does exactly that: no source code was changed anywhere in `image` or
`pic-scale-safe` - only the `RUSTFLAGS` used to build the same source was changed. This is a
build-configuration investigation, not a source-level fix (see "Does it change behavior/output?"
for why there is correctly no `patch.diff`).

## What was tried

Reused candidate 040's `bench_resize_dynamic` criterion bench (temporarily reapplied to
`benches/imageops.rs`, reverted before finishing, not part of a permanent source change) and its
`paired_pic_scale.sh` paired-comparison script. Two independent axes, each built in its own
`CARGO_TARGET_DIR` to keep binaries side by side:

- **Registry pic-scale-safe 0.1.11, completely unmodified** (no vendoring, no `[patch.crates-io]`)
  - S1: default `RUSTFLAGS` (generic x86-64 baseline, same as every prior candidate's builds)
  - E1: `RUSTFLAGS="-C target-feature=+avx2"`
- **Vendored pic-scale-safe with candidate 040's unmerged "octo" 64-column tier** (`vendor/`,
  `[patch.crates-io]`, patch reapplied verbatim from `040.../patch.diff`, itself unmodified further)
  - S2: default `RUSTFLAGS`
  - E2: `RUSTFLAGS="-C target-feature=+avx2"`

**Disassembly** (`objdump -d -C -M intel --no-show-raw-insn`, `nm --demangle` to locate symbols,
same method as candidate 040):

- S1's `column_handler_fixed_point::<u8, i32>`: 118 `xmm`-operand instructions, **zero** `ymm`/`zmm`
  - confirms candidate 040's finding (`raw/base_col_u8_disasm.txt`).
- E1's same function: 5 `xmm`, **62 `ymm`**-operand instructions (`vpmaddwd`/`vpaddd`/`vmovdqa`/
  `vpermq`/`vpsrad`/`vpmovzxbd` all with `ymm` operands, `vzeroupper` at function exit) - the
  compiler auto-vectorizes the same safe-Rust tap loop to 256-bit AVX2 width once the target
  feature is available, with no source change (`raw/avx2_col_u8_disasm.txt`).
- E2's `convolve_column_handler_fixed_point_direct_buffer_octo::<u8, i32>` (candidate 040's tier,
  built under AVX2): 2 `xmm`, 135 `ymm`-operand instructions, **and no `[rsp]`/`[rbp]`-relative
  vector-register memory operands anywhere in the function** - i.e. none of candidate 040's
  observed partial spill (3 of 8 accumulators round-tripping through the stack every tap) is
  present once building under AVX2 (`raw/octo_avx2_disasm.txt`).

**Correctness (bit-exactness) across all four configurations**, via a temporary
`examples/resize_hash_check.rs` (not part of any permanent change) hashing `DynamicImage` clones
resized with `resize_exact` (`to_rgb8().into_raw()`, `DefaultHasher`) across
`Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/`Nearest`, 8 destination sizes on a 4032x3024 source
and 3 on a 400x300 source (40 hashes total per build): S1, E1, S2, E2 all produced byte-identical
hash output (`raw/hashes_{S1,E1,S2,E2}_*.txt`, pairwise `diff` clean).

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S1: registry 0.1.11, default RUSTFLAGS (baseline) | 18.766 ms | 18.703 ms | Lanczos3, 4032x3024->400x300 |
| E1: registry 0.1.11, `+avx2` | 10.978 ms (**-41.5%**, 5/5 E-faster) | 11.048 ms (**-40.9%**, 5/5 E-faster) | Lanczos3, same scenario |
| S1 (baseline) | 7.325 ms | n/a | Triangle, same source/dest, single-core only |
| E1: `+avx2` | 5.502 ms (**-24.9%**, 5/5 E-faster) | n/a | Triangle |
| S1 (baseline) | 199.66 us | n/a | Lanczos3, 400x300->32x24 (small image) |
| E1: `+avx2` | 119.59 us (**-40.1%**, 5/5 E-faster) | n/a | Lanczos3, same small scenario |
| S1 vs S1 (same-binary control) | 18.966 -> 19.102 ms (+0.7%, 1/5 "wins") | n/a | noise floor for the Lanczos3 comparison above |
| S2: vendored octo tier, default RUSTFLAGS | 19.879 ms | n/a | Lanczos3, 4032x3024->400x300 (matches candidate 040's own S baseline) |
| E2: vendored octo tier, `+avx2` | 10.532 ms (**-47.0%** vs S2, 5/5 E-faster) | n/a | Lanczos3, same scenario |
| E1 (double tier, `+avx2`) vs E2 (octo tier, `+avx2`) | 11.064 -> 10.686 ms (**-3.4%**, 4/5 E2-faster) | n/a | both AVX2-built; octo tier now marginally ahead of the shipping double tier once built with AVX2, opposite of candidate 040's SSE2-build result (+5.6%) |

5 alternating paired runs per comparison via `raw/paired_pic_scale.sh` (candidate 040's script,
reused verbatim). `taskset -c 3` for single-core; unpinned for multi-core (24 logical cores,
`uptime` load ~1.6-2.8 during this session). This host's CPU supports AVX2 and AVX-512
(`lscpu`), so `-C target-feature=+avx2` is guaranteed to run on the measuring machine; portability
to AVX2-lacking hardware is exactly why this isn't proposed as a blanket source change (see below).

Raw logs under `raw/`: `singlecore_S_vs_E_Lanczos3.log`, `singlecore_S_vs_E_Triangle.log`,
`singlecore_S_vs_E_small_Lanczos3.log`, `multicore_S_vs_E_Lanczos3.log`,
`control_singlecore_S_vs_S_Lanczos3.log`, `singlecore_octo_S2_vs_E2_Lanczos3.log`,
`singlecore_E1double_vs_E2octo_avx2_Lanczos3.log`, `base_col_u8_disasm.txt`,
`avx2_col_u8_disasm.txt`, `octo_avx2_disasm.txt`, `hashes_S1_registry_noavx2.txt`,
`hashes_E1_registry_avx2.txt`, `hashes_S2_octo_noavx2.txt`, `hashes_E2_octo_avx2.txt`,
`test_S_baseline_noavx2.txt`, `test_E_avx2.txt`.

## Result

Confirms the `IDEAS.md` hypothesis directly: an AVX2-width rebuild of the completely unmodified
`pic-scale-safe` 0.1.11 crate is **-41.5% (single-core) / -40.9% (multi-core)** on the Lanczos3
4032x3024->400x300 scenario, **-24.9%** on Triangle, and **-40.1%** on a small-image Lanczos3
scenario - all 5/5 paired runs faster, all far outside the +0.7% same-binary noise floor measured
on the same scenario. This is not specific to candidate 040's octo tier: it reproduces on the
*shipping, unmerged-candidate-free* registry crate with zero source changes, confirming the
codegen-width theory, not a property of any one tier's shape.

As a secondary result, candidate 040's octo tier - a **regression** (+5.2% to +7.6%) when compared
under the crate's default (SSE2-only) build - becomes a small, noisy **win** (-3.4%, 4/5 faster)
over the shipping double tier once both are built under AVX2, consistent with the disassembly
showing the partial register spill that caused 040's regression is gone once building under AVX2
gives the compiler twice the per-register lane width. This secondary comparison had fewer repeat
runs than the primary S1-vs-E1 comparison and a smaller effect size closer to the noise floor - it
supports the "spill is gone" theory but is a much weaker result than the primary ~41% finding.

## Does it change behavior/output?

- **Bit-exact?** Yes, confirmed across all four configurations (S1, E1, S2, E2) - see "What was
  tried" above. Expected: same fixed-point integer arithmetic (`J = i32`), just executed with a
  wider SIMD width; a 256-bit AVX2 add/multiply of the same operands in the same order produces
  the same per-lane result as the 128-bit SSE2 version, so bit-exactness is unsurprising and
  confirmed, not merely assumed.
- **Is this a breaking change?** N/A - **this candidate made no source change to `image` or
  `pic-scale-safe`**, only to the `RUSTFLAGS` used to build an otherwise-untouched checkout for
  measurement. There is correctly **no `patch.diff`** in this folder: nothing was fixed in the
  tested project's own source. The actionable finding is a build-configuration one (see "What's
  not done" below for why it isn't proposed as a blanket default), not a code change - per
  SKILL.md's "patch.diff is mandatory whenever the fix is a change to the tested project" section,
  a pure measurement/investigation with no source patch correctly has no `patch.diff`.
- Project test suite (`cargo test --release --no-fail-fast`), run both with default `RUSTFLAGS`
  and with `-C target-feature=+avx2`, both against the completely unmodified registry crate (no
  vendoring, no 040 patch): **622 passed, 1 failed, 4 ignored, identical in both builds**
  (`raw/test_S_baseline_noavx2.txt`, `raw/test_E_avx2.txt`). The one failure,
  `regression::resizing_with_catmul`, is the same pre-existing, unrelated bug documented in
  `___OTHER___/003.RESIZE_EXACT_CATMULLROM_OFF_BY_ONE_ROUNDING` - not introduced or affected by the
  `RUSTFLAGS` change.

## Why the result looks the way it does

`-C target-feature=+avx2` (or `target-cpu=native` on an AVX2-capable host) lets LLVM's
auto-vectorizer widen the same safe-Rust integer tap loop from 128-bit SSE2 (`xmm`, 4x `i32` lanes)
to 256-bit AVX2 (`ymm`, 8x `i32` lanes) with **zero source changes** - the crate's own code never
uses `std::arch` intrinsics or `#[target_feature]`, it relies entirely on the default
auto-vectorizer, which only ever emits instructions legal for the crate's actual compile-time
target features. Since this project (like most published crates) is built against the generic
x86-64 baseline for portability, LLVM never had the option to emit `ymm` instructions in any
prior candidate's measurements - every accumulator-tier-sizing analysis in candidates 039/040 was
implicitly reasoning about a 128-bit register budget while describing it as "one AVX2 register."
Doubling the real lane width roughly halves the number of loop iterations needed for the same
column count, which is consistent with the ~41% wall-clock improvement (not exactly 2x, since
setup/row-hop overhead and non-vectorized portions of the function don't share in the speedup).
The secondary octo-tier result (regression under SSE2, small win under AVX2) is explained by the
same register-width change removing candidate 040's observed partial spill: 8 `[J; 8]`
accumulators need 16 XMM registers under SSE2 (more than are available, forcing 3 of 8 to spill)
but only 8 YMM registers under AVX2 (fits comfortably), which matches the disassembly finding of
zero `[rsp]`/`[rbp]` vector spills in the AVX2-built octo tier versus candidate 040's confirmed
3-of-8 spill under the default build.

## What's not done / open questions

- **No source-level change is proposed here.** Forcing `-C target-feature=+avx2` as a blanket
  default for the whole `image` crate (e.g. via `.cargo/config.toml` or a `[profile]` rustflags
  entry) would make the resulting binary illegal-instruction-crash on any CPU without AVX2 - a
  real portability regression for a general-purpose, widely-deployed crate that currently makes no
  such assumption anywhere else in its own build. This candidate deliberately stayed a measurement
  (confirming the *theory*, per `IDEAS.md`'s framing "a cheap, high-leverage check before trying
  more source-level tier variants") rather than proposing an unsafe default.
- A **safe** way to get this speedup into the shipped crate - e.g. runtime CPU-feature detection
  (`is_x86_feature_detected!("avx2")`) with an explicit `#[target_feature(enable = "avx2")]`
  variant of the tap loop selected at runtime (multiversioning), the same pattern several other
  SIMD-heavy Rust crates use - was not attempted; it is a materially larger, riskier source change
  than anything else in this workspace's candidates so far (manual `unsafe fn` + intrinsics or a
  `#[multiversed]`-style macro, plus its own correctness/UB review) and deserves its own dedicated
  candidate, not a follow-on to a measurement-only investigation. Left as a new `IDEAS.md` entry.
- Whether downstream consumers of `image`/`pic-scale-safe` who already build with
  `RUSTFLAGS="-C target-cpu=native"` (a common practice for e.g. server-side image-processing
  pipelines that control their own deployment hardware) are already getting this speedup today
  with zero code changes was not verified beyond this measurement - this candidate's own numbers
  are direct evidence that they are, but no downstream project was checked.
- The secondary octo-tier-under-AVX2 result (-3.4%) had only one round of 5 paired runs, smaller
  effect size than the primary finding, and was not cross-checked with a same-binary noise-floor
  control the way the primary S1-vs-E1 comparison was - treat it as suggestive, not as strong
  confirmation that the octo tier should be merged even under an AVX2 build.
- No AVX-512 rebuild was tried, even though this host's CPU supports it (`lscpu` reports
  `avx512f`/`avx512bw`/etc.) - `target-feature=+avx2` was chosen to match the `IDEAS.md` entry
  exactly; a further AVX-512 data point was out of this candidate's scope but might be worth its
  own follow-up.
- No patch was kept applied anywhere - the working tree was reverted to a clean state
  (`git checkout -- benches/imageops.rs`, `rm -rf target-avx2`, temporary
  `examples/resize_hash_check.rs` deleted) before this file was written, per the workflow's
  standing rule, regardless of the (informative, but non-source-patch) result.
