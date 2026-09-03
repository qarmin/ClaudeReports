# Vertical convolution: register-resident multi-sub-block accumulator tier (regression)

**Ported from `image` repo's
`___OPTIMIZATIONS___/040.PIC_SCALE_SAFE_VERTICAL_CONVOLUTION_MULTI_ACCUMULATOR/`.** Originally
measured on 2026-08-26 via a vendored `pic-scale-safe` 0.1.11 copy inside the `image` project.
**Not re-measured in this workspace** - numbers below are copied verbatim. The patch was rebuilt
by hand against this repo's current `HEAD` (`b50e020`), inserting the new
`convolve_column_handler_fixed_point_direct_buffer_octo` function and its cascade call site
(unchanged in shape from 0.1.11). `git apply --check` passes, `cargo check --features rayon`
compiles clean, and `cargo test --release --features rayon` passes 5/5 (verified this session).

**100% safe-Rust - no `unsafe`, no `#[target_feature]`.** This is the direct follow-up to
candidate 011's failed 512-element block tier, and it also regressed - read both before attempting
further work in this area.

## What was tried

Candidate 011 (2026-08-25) found that a single 512-element accumulator regresses ~2x due to
register spilling. This candidate tries the natural follow-up: instead of one giant spilled
accumulator, keep several *separate*, register-sized accumulators (8 x `[J; 8]` = 64 columns) live
simultaneously across one pass over the taps, so the row-hop cost is amortized 8x while each
individual accumulator was assumed small enough to stay resident in a vector register - matching
the shape the existing `..._direct_buffer_double` (8 x 4-wide = 32 columns) and aarch64-only
`..._four` (4 x 16-wide = 64 columns) tiers already use at smaller widths. Added
`convolve_column_handler_fixed_point_direct_buffer_octo` (8 separate `[J; 8]` accumulators,
`s0..s7`) to `fixed_point_vertical.rs`, wired into the cascade right before the existing 32-column
"double" tier.

**Disassembly check (the verification this candidate specifically called for):** built in release
mode and disassembled the `<u8, i32>` instantiation with `objdump -d -C -M intel
--no-show-raw-insn`. Result: each accumulator is only 4 x i32 = 16 bytes (one **XMM** register,
not YMM - this file never emits a 256-bit AVX2 load/store anywhere; `punpcklbw`/`pmaddwd`/`paddd`
are all 128-bit SSE2 ops), so 8 accumulators need 8 XMM registers plus 2-3 more temporaries
(zero-vector, broadcast weight, scratch load) per tap. The compiler could not keep all of them
resident: the generated tap loop spills **3 of the 8** accumulators to the stack every iteration
(`movdqa xmm13,[rsp]` / `[rsp+0x10]` / `[rsp+0x20]`, load-add-store each tap), while the other 5
stay in XMM registers across the whole loop - a genuinely different, *partial* spill pattern from
candidate 011's fully-spilled 512-wide accumulator, but still a real spill.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline (registry 0.1.11) | 18.853 ms | 19.140 ms | Lanczos3, 4032x3024->400x300 |
| this candidate (+64-col "octo" tier) | 19.915 ms (**+5.6%**, 0/5 faster) | 20.143 ms (**+5.2%**, 0/5 faster) | same |
| baseline | 7.369 ms | n/a | Triangle |
| this candidate | 7.933 ms (**+7.6%**, 1/5 faster) | n/a | same |
| baseline | 195.6 us | n/a | Lanczos3, 400x300->32x24 (small image) |
| this candidate | 209.7 us (**+7.2%**, 0/5 faster) | n/a | same |
| same-binary control | 18.853 -> 18.859 ms (+0.0%, 2/5 "wins") | n/a | noise floor |

## Result

**Regression** in every scenario tried: +5.2% to +7.6% slower, 0-1 of 5 paired runs faster,
outside the +0.0% same-binary noise floor. Smaller than candidate 011's ~2x regression (a partial,
3-of-8 spill instead of a full 2 KiB spill), but still consistently worse than the existing
upstream 32-column "double" tier. Not proposed for adoption.

## Does it change behavior/output?

- **Bit-exact?** Yes - the new tier calls the same per-column accumulation arithmetic (same
  weights, same tap order), just partitioned into 8 separate 8-wide accumulators instead of the
  existing tiers' partitioning; this crate's accumulator type (`J = i32`, fixed-point) has no
  floating-point rounding-order sensitivity, so bit-exactness is expected and was confirmed (40
  hashes across 5 filters x multiple source/dest sizes, byte-identical before/after in the
  original session).
- **Is this a breaking change?** No - not a winning variant, bit-exact output. No `unsafe`
  anywhere in this patch.
- Project test suite: `cargo test --release --features rayon` - **5 passed, 0 failed** (this
  session, against current `HEAD`). Original measurement additionally quoted `image`'s own full
  suite: 622 passed, 1 pre-existing unrelated failure, identical before/after.

## Why the result looks the way it does

The candidate's premise - "keep several register-sized accumulators live, unlike candidate 011's
one huge spilled accumulator" - assumed the *existing* smaller tiers already achieve full register
residency at 8-wide, and that the only obstacle to widening was picking a size under the register
budget. The disassembly shows that assumption doesn't fully hold even at 64 columns: `J = i32`
accumulators in this file are 128-bit (XMM, 4 lanes), not 256-bit (YMM, 8 lanes) as the candidate's
own description assumed - so each "8-wide" accumulator is *two* XMM registers' worth, not one. 8
such accumulators need 16 XMM registers just for the accumulators, leaving none for the per-tap
zero-vector/broadcast-weight/scratch-load also live in the loop body, forcing a partial spill.
That's a smaller loss than candidate 011's full-buffer spill (3/8 of the working set round-trips
through L1 each tap instead of the full buffer every tap), matching the smaller regression
magnitude (+5-8% here vs. +97-104% there) - but still worse than the existing 32-column tier,
which uses the same `[J; 4]`-per-store shape at half the total width and is presumed (not
disassembled) not to spill at that narrower width. **Practical conclusion for this crate, without
`RUSTFLAGS`/`#[target_feature]`: the safe-Rust register budget for this loop shape caps out
somewhere between the existing 32-column tier and 64 columns - there is no free win left in
"just add a wider tier" for the vertical convolution path.**

## What's not done / open questions

- Whether an 8 x 4-wide (32-column) "octo-at-double-width" tier - i.e. the same multi-accumulator
  shape at the existing 32-column granularity - would fully avoid spilling, was not measured; it
  would likely just reimplement the existing `..._double` tier.
- Whether `J = i64` (used for higher bit depths elsewhere in this crate) hits the register budget
  differently was not measured; a narrower-lane-per-register accumulator type would spill sooner,
  not later, so this is unlikely to flip the result.
- No explicit-SIMD (`std::arch`) rewrite was attempted, and per this repo's project policy (see
  `SUMMARY.md`), never will be - `unsafe` is out of scope here regardless of any measured upside.
