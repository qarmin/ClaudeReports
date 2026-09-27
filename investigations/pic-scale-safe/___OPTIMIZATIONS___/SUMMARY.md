# Optimization results

**Impact** (derived from the Result column, for scanning at a glance - not a recommendation, this
classifies the measured magnitude/direction only, it doesn't decide what to do about it): **Strong
win** (large, confirmed improvement) - **Small win** (real but modest improvement) - **No change**
(null result / within noise) - **Mixed** (improves one thing, regresses another) - **Small/Strong
regression** (would make things worse) - **N/A** (pure measurement/characterization, no patch
compared against a baseline).

**Every candidate in this workspace so far (001-012) was ported from `image` repo's
`___OPTIMIZATIONS___/` workspace**, where it was found and measured while optimizing `image`'s own
delegation to this crate (`pic-scale-safe`). None of the timing numbers below were re-measured
against this repo directly - see each candidate's own `RESULT.md` header for the originating
folder and an explicit "not re-measured" note. What *was* independently verified in this session,
for every candidate: the patch applies cleanly against this repo's current `HEAD`
(`b50e020d72a1c0063f39c2c0166343d4e44790ca`) via `git apply --check`, `cargo check --features
rayon` compiles with no errors (and, for candidate 007, no warnings), and `cargo test --release
--features rayon` still passes this crate's own 5-test suite. Candidates 001-004 form one
progression (each supersedes the multi-core regression the previous one found); 007-010 form a
second, independent progression (AVX2 then AVX-512 runtime multiversioning); 011-012 are a third,
independent, **100% safe-Rust** progression (wider column-block tiers for cache locality in the
vertical convolution) - both regressed, see their own `RESULT.md` for why, and read them before
attempting further safe-Rust auto-vectorization work on that function. Candidates 005/006 are
measurement-only investigations with no source patch, motivating 007-010.

**Project policy (2026-08-29, confirmed by the repo owner): unsafe code is never in scope in
this crate, regardless of measured speedup.** `#![forbid(unsafe_code)]` in `src/lib.rs` is a
permanent, non-negotiable design commitment - the entire point of the crate's name. Candidates
007/008/009/010 below all require relaxing that lint to `deny` with scoped `#[allow(unsafe_code)]`
opt-outs for `#[target_feature]` multiversioning; they are kept here as a historical, fully-measured
record of what unsafe SIMD intrinsics *would* buy, but they are explicitly **not adoptable** in this
repo under any circumstance. Unsafe/SIMD-intrinsics optimization work belongs in the sibling crate
`~/Projekty/Obce/pic-scale` instead, never here. See each of those four candidates' own `RESULT.md`
for the same note inline.

**Candidate 013 (new work this session, not ported) found that this repo's own
`.cargo/config.toml` sets `-Ctarget-cpu=native` for local builds - a setting that does not
propagate to downstream consumers via Cargo, and that flips the direction of at least one real
optimization (013 itself: a win under the generic baseline consumers actually get, a regression
under this repo's own native-cpu dev builds).** Keep this in mind when reading any candidate's
numbers measured on this machine: absolute timings below reflect whichever build config was
actually used for that candidate - see each `RESULT.md`.

Sort by impact (biggest real win first; noise/regressions at the bottom).

| # | Impact | Candidate | Result | Changes behavior? | Fixed by | Link |
|---|---|---|---|---|---|---|
| 013 | Mixed | 100% safe-Rust: the alpha channel's premultiply write (`div_by_255(255*a)`/`div_by_2pn_m1(max_colors*a,n)`) is mathematically always `== a` (verified exhaustively) - removing it breaks the AVX-512 vectorizer's ability to batch 16 pixels/iteration (this repo's own native-cpu dev build), but is a pure win under the generic baseline most real consumers get | native-cpu: **+90% to +224% regression**; `RUSTFLAGS=""` baseline: **-22% to -44% win** (both directions confirmed via `just test` from a fresh clone, multiple paired runs) | no, bit-exact (200+ randomized trials against a reference implementation, 0 mismatches) | | [`013.ALPHA8_SKIP_REDUNDANT_ALPHA_WRITE/RESULT.md`](013.ALPHA8_SKIP_REDUNDANT_ALPHA_WRITE/RESULT.md) |
| 007 | Strong win | Vertical convolution tap loop (`fixed_point_vertical.rs`) never uses SIMD wider than SSE2 because the crate is built against the generic x86-64 baseline - runtime `is_x86_feature_detected!("avx2")` + `#[target_feature(enable = "avx2")]` multiversioning widens it to AVX2 on capable hosts with a verified-correct scalar fallback | -27.0%/-26.9% (single/multi-core) on a large Lanczos3 resize, -14.5% Triangle, -26.8% small-image Lanczos3 (all 5/5 paired) | yes - relaxes `#![forbid(unsafe_code)]` to `#![deny(unsafe_code)]`, see RESULT.md | | [`007.AVX2_VERTICAL_RUNTIME_MULTIVERSIONING/RESULT.md`](007.AVX2_VERTICAL_RUNTIME_MULTIVERSIONING/RESULT.md) |
| 008 | Strong win | Extends 007's AVX2 multiversioning to the horizontal convolution pass and floating-point vertical/horizontal paths, which 007 left scalar | -40.0%/-39.9% large Lanczos3 (up from 007's -27%), -23.9% Triangle, -39.5%/-39.3% CatmullRom and -40.4% Gaussian on the floating-point path (all 5/5 paired) | yes - same `unsafe_code` policy change as 007, no further relaxation needed | | [`008.AVX2_HORIZONTAL_FLOATING_RUNTIME_MULTIVERSIONING/RESULT.md`](008.AVX2_HORIZONTAL_FLOATING_RUNTIME_MULTIVERSIONING/RESULT.md) |
| 001 | Strong win (vs. today's ungated `rayon` feature) / wash vs. `rayon`-off | `resize_nearest`'s unconditional `par_chunks_exact_mut` pays rayon dispatch/join overhead even for tiny outputs where it dwarfs the actual work - add a `dst_height >= 32` minimum-row gate, falling back to the existing serial loop below it | vs. today's ungated rayon: -91.8% single-core / -95.9% multi-core (8/8, 10/10 pairs); vs. `rayon`-off: wash on both axes | no, bit-exact | | [`001.RESIZE_NEAREST_RAYON_ROW_GATE/RESULT.md`](001.RESIZE_NEAREST_RAYON_ROW_GATE/RESULT.md) |
| 003 | Mixed | Replaces 002's flat row-count gate with an always-dispatch per-filter capped thread pool (`num_threads(4)`/`(6)` depending on filter) in `fixed_point_dispatch.rs`/`floating_point_dispatch.rs` | multi-core: -36.3% to -46.9% vs. 002's flat gate, -10.4% to -37.4% vs. ungated rayon (all 6/6); single-core: +0.7% to +13.6% regression vs. flat gate/serial (0-2/6) | no, bit-exact | | [`003.DISPATCH_PER_FILTER_CAPPED_POOL/RESULT.md`](003.DISPATCH_PER_FILTER_CAPPED_POOL/RESULT.md) |
| 009 | Small win | Adds a third `has_avx512` dispatch tier on top of 007/008's AVX2/scalar cascade, for the vertical tap loop only | -2.2% to -3.5% on top of 007/008's already-measured AVX2 win (4-5/5 pairs); small enough to be noise-sensitive on a loaded host | yes - same `unsafe_code` policy as 007, no further relaxation | | [`009.AVX512_VERTICAL_RUNTIME_DISPATCH_TIER/RESULT.md`](009.AVX512_VERTICAL_RUNTIME_DISPATCH_TIER/RESULT.md) |
| 010 | Small win | Extends 009's AVX-512 dispatch tier to the horizontal/floating-point paths 008 already multiversioned for AVX2 | -1.4% to -3.6% on 5 of the touched functions; 2 of 5 disassembly-confirmed to never widen past 128-bit regardless of target feature (harmless, contributes nothing) | yes - same `unsafe_code` policy as 007, no further relaxation | | [`010.AVX512_HORIZONTAL_FLOATING_RUNTIME_DISPATCH_TIER/RESULT.md`](010.AVX512_HORIZONTAL_FLOATING_RUNTIME_DISPATCH_TIER/RESULT.md) |
| 004 | Small win | Adds `MIN_ROWS_FOR_POOL = 8` to 003's per-filter pool, falling back to serial below it - fixes 003's small single-core regression for genuinely tiny outputs without losing 003's multi-core win at the workspace's own established scenario | at `dst_height=6` (below floor): -6.5% to -18.8% vs. 003 on both axes (most 5/5); at `dst_height=24` (above floor): identical to 003, unchanged | no, bit-exact | | [`004.DISPATCH_LOW_ROW_FLOOR_CAPPED_POOL/RESULT.md`](004.DISPATCH_LOW_ROW_FLOOR_CAPPED_POOL/RESULT.md) |
| 005 | N/A | Measurement only: rebuilding the unmodified crate with `RUSTFLAGS="-C target-feature=+avx2"` auto-vectorizes the vertical tap loop with zero source changes | -41.5%/-40.9% large Lanczos3, -24.9% Triangle, -40.1% small-image Lanczos3 (all 5/5); not proposed as a build default (illegal-instruction risk on non-AVX2 hosts) | N/A - measured/quantified only, no patch applied to real source | | [`005.AVX2_TARGET_FEATURE_REBUILD_MEASUREMENT/RESULT.md`](005.AVX2_TARGET_FEATURE_REBUILD_MEASUREMENT/RESULT.md) |
| 006 | N/A | Measurement only: same as 005 but with `+avx512f,+avx512bw,+avx512vl` | -44.6%/-44.5% large Lanczos3, -30.2% Triangle, -32.8% small-image Lanczos3 vs. baseline; -6.0% further vs. a fresh AVX2 build (all 5/5) | N/A - measured/quantified only, no patch applied to real source | | [`006.AVX512_TARGET_FEATURE_REBUILD_MEASUREMENT/RESULT.md`](006.AVX512_TARGET_FEATURE_REBUILD_MEASUREMENT/RESULT.md) |
| 002 | Mixed | Extends 001's flat `dst_height >= 32` gate mechanically to `fixed_point_dispatch.rs`/`floating_point_dispatch.rs`'s 6 dispatch functions (Triangle/CatmullRom/Gaussian/Lanczos3) | single-core: -4.1% to -11.6% vs. ungated rayon (6/6), wash vs. serial; multi-core: Triangle -3.2% win (6/6), but CatmullRom/Gaussian/Lanczos3 +25% to +41% **regression** vs. ungated rayon (0-1/6) | no, bit-exact | | [`002.DISPATCH_FLAT_ROW_GATE/RESULT.md`](002.DISPATCH_FLAT_ROW_GATE/RESULT.md) |
| 012 | Small/Strong regression | 100% safe-Rust follow-up to 011: 8 separate register-sized `[J; 8]` accumulators (64 columns) in the vertical tap loop, instead of one giant one - disassembly shows each is 128-bit XMM not 256-bit YMM, so the compiler still partially spills 3 of 8 | +5.2% to +7.6% slower across Lanczos3/Triangle/small-image (0-1/5 faster in each) | no, bit-exact | | [`012.VERTICAL_CONVOLUTION_MULTI_ACCUMULATOR/RESULT.md`](012.VERTICAL_CONVOLUTION_MULTI_ACCUMULATOR/RESULT.md) |
| 011 | Small/Strong regression | 100% safe-Rust attempt: a wider (512-element) column-block tier in the vertical convolution cascade, aiming to amortize per-row cache-hop cost - regresses because the accumulator no longer fits in vector registers and spills to the stack every tap | +97-104% slower (~2x) on Lanczos3/Triangle, 0/5 faster in every comparison | no, bit-exact | | [`011.VERTICAL_CONVOLUTION_BLOCKED_TRAVERSAL/RESULT.md`](011.VERTICAL_CONVOLUTION_BLOCKED_TRAVERSAL/RESULT.md) |
