# pic-scale-safe vertical convolution: register-resident multi-sub-block accumulator tier

Candidate 039 (2026-08-25) tried widening `column_handler_fixed_point`'s vertical-convolution
cascade with a single 512-element `BUFFER_SIZE` tier and found a ~2x regression, attributed to the
wide accumulator spilling out of vector registers. It left an untried follow-up in `IDEAS.md`
(2026-08-26): instead of one giant spilled accumulator, keep several *separate*, register-sized
accumulators (e.g. 8 x 8-wide = 64 columns) live simultaneously across one pass over the taps, so
the row-hop cost is amortized 8x while each individual accumulator stays small enough to plausibly
stay resident in a vector register. Notably, `fixed_point_vertical.rs` (pic-scale-safe 0.1.11)
already uses this general shape at smaller widths -
`convolve_column_handler_fixed_point_direct_buffer_double` keeps 8 x 4-wide accumulators live for
32 columns, and (aarch64-only) `..._four` keeps 4 x 16-wide accumulators live for 64 columns - so
this candidate adds one more tier following the same pattern, for x86_64: an 8 x 8-wide (64-column)
"octo" tier, inserted into the cascade above the existing 32-column "double" tier.

## What was tried

Vendored `pic-scale-safe` 0.1.11 into `vendor/pic-scale-safe/` (matching candidate 039's method,
`[patch.crates-io]` in `Cargo.toml`) and added a new function,
`convolve_column_handler_fixed_point_direct_buffer_octo`, to `fixed_point_vertical.rs`: 8 separate
`[J; 8]` accumulators (`s0..s7`, each one AVX2 register's worth for `J = i32`) summed across the
whole tap loop, covering 64 columns per call. Wired into `column_handler_fixed_point`'s cascade
right before the existing 32-column "double" tier:

```rust
while cx + 64 <= total_width {
    convolve_column_handler_fixed_point_direct_buffer_octo::<T, J>(
        src, src_stride, dst, weight, bounds, bit_depth, cx,
    );
    cx += 64;
}
```

**Disassembly check (the verification this candidate specifically called for):** built
`cargo bench --bench imageops` in release mode and disassembled the `<u8, i32>` instantiation of
the new function with `objdump -d -C -M intel --no-show-raw-insn` (function kept `#[inline(never)]`
per the existing convention in this file, giving it a standalone symbol at `nm --demangle`). Result:
each accumulator is only 4 x i32 = 16 bytes (one **XMM**, not YMM - the crate does not use
256-bit AVX2 loads/stores anywhere in this file, `punpcklbw`/`pmaddwd`/`paddd` are all 128-bit SSE2
ops), so 8 accumulators need 8 XMM registers, plus 2-3 more temporaries (zero-vector, broadcast
weight, scratch load) per tap. The compiler could not keep all of them resident: the generated tap
loop spills **3 of the 8** accumulators to the stack every iteration
(`movdqa xmm13,[rsp]` / `... [rsp+0x10]` / `... [rsp+0x20]`, load-add-store each tap), while the
other 5 stay in XMM registers across the whole loop - i.e. a genuinely different, *partial* spill
pattern from candidate 039's single 512-element accumulator (which spilled the entire 2 KiB
buffer), but still a real spill, not the fully register-resident shape the candidate hoped to
confirm. Full disassembly excerpt in `raw/octo_disasm.txt`.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S: registry 0.1.11 (baseline) | 18.853 ms | 19.140 ms | Lanczos3, 4032x3024->400x300 |
| E: +64-col "octo" tier | 19.915 ms (**+5.6%**, 0/5 E-faster) | 20.143 ms (**+5.2%**, 0/5 E-faster) | Lanczos3, same scenario |
| S: registry 0.1.11 (baseline) | 7.369 ms | n/a | Triangle, same source/dest, single-core only |
| E: +64-col "octo" tier | 7.933 ms (**+7.6%**, 1/5 E-faster) | n/a | Triangle |
| S: registry 0.1.11 (baseline) | 195.6 us | n/a | Lanczos3, 400x300->32x24 (small image) |
| E: +64-col "octo" tier | 209.7 us (**+7.2%**, 0/5 E-faster) | n/a | Lanczos3, same small scenario |
| S vs S (same-binary control) | 18.853 -> 18.859 ms (+0.0%, 2/5 "wins") | n/a | noise floor for the Lanczos3 comparison above |

5 alternating paired runs per comparison via `raw/paired_pic_scale.sh` (adapted from candidate
033's script). `taskset -c 3` for single-core; unpinned for multi-core (24 logical cores, `uptime`
load 0.4-1.5 during this session). This code path runs single-threaded regardless of the crate's
own `rayon` feature (not forwarded by `image`'s `Cargo.toml`, per candidate 039's note), so
"multi-core" here means unpinned wall-clock only, not parallel execution of this function.

Raw logs under `raw/`: `singlecore_S_vs_E_Lanczos3.log`, `singlecore_S_vs_E_Triangle.log`,
`singlecore_S_vs_E_small_Lanczos3.log`, `multicore_S_vs_E_Lanczos3.log`,
`control_singlecore_S_vs_S_Lanczos3.log`, `octo_disasm.txt`.

## Result

**Regression, not a win**, in every scenario tried: +5.2% to +7.6% slower, 0-1 of 5 paired runs
faster in each comparison, well outside the +0.0% same-binary noise floor measured on the same
scenario. Smaller (+5.2-5.6%) than candidate 039's ~2x regression from the fully-spilled 512-wide
single accumulator, but still a real, reliably-measured regression rather than a wash.

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified via a temporary example (`examples/resize_hash_check.rs`, deleted
  before the revert, not part of `patch.diff`) hashing `DynamicImage::resize_exact` output
  (`to_rgb8().into_raw()`, `DefaultHasher`) across `Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/
  `Nearest`, on both a 4032x3024 source (destinations `400x300`/`64x48`/`67x53`/`1x1`/`512x512`)
  and a 400x300 source (destinations `32x24`/`800x600`/`1x1`) - 40 hashes total, byte-identical
  between the `S` and `E` builds (`raw/hashes_S.txt` vs `raw/hashes_E.txt`, `diff` clean). Expected:
  the new tier calls the same per-column accumulation arithmetic (same weights, same tap order),
  just partitioned into 8 separate 8-wide accumulators instead of the existing tiers' partitioning
  - summation order changes, but this crate's accumulator type (`J = i32`, fixed-point) has no
  floating-point rounding-order sensitivity, so bit-exactness is expected and confirmed.
- **Is this a breaking change?** No (not applicable - not a winning variant, and bit-exact output
  means no observable behavior change even if it were kept).
- Project test suite (`cargo test --release --no-fail-fast`): **622 passed, 1 failed, 4 ignored,
  identical before and after** (`raw/test_S_full.txt`, `raw/test_E_full.txt`). The one failure,
  `regression::resizing_with_catmul`, is the same pre-existing, unrelated bug already documented in
  `___OTHER___/003.RESIZE_EXACT_CATMULLROM_OFF_BY_ONE_ROUNDING` (confirmed there to reproduce on a
  clean checkout with zero `___OPTIMIZATIONS___` patches applied) - not introduced or affected by
  this candidate.

## Why the result looks the way it does

The candidate's premise - "keep several register-sized accumulators live, unlike the 039 tier's
one huge spilled accumulator" - assumed the *existing* smaller tiers already achieve full register
residency, and that the only obstacle to widening was picking a size that stays under the register
budget. The disassembly shows that assumption doesn't fully hold even at 64 columns: `J = i32`
accumulators in this file are 128-bit (XMM, 4 lanes), not 256-bit (YMM, 8 lanes) - this file never
emits an AVX2-width `movdqa`/`vmovdqa` with a `ymm` operand anywhere, only SSE2 `xmm` instructions -
so each "8-wide" accumulator the candidate description called "one AVX2 register's worth" is
actually *two* XMM registers' worth in the generated code, not one. 8 such accumulators need 16
XMM registers just for the accumulators, leaving none for the per-tap zero-vector, broadcast
weight, and scratch load also live in the loop body - so the compiler had to spill something, and
picked 3 of the 8 sub-accumulators (round-tripping them through the stack every tap) rather than
all of it. That's a smaller loss than candidate 039's full-buffer spill (3/8 of the working set
round-trips through L1 each tap instead of the full buffer every tap), which matches the smaller
regression magnitude observed (+5-8% here vs. +97-104% there), but it's still consistently worse
than the existing 32-column "double" tier shipped upstream, which uses the same `[J; 4]`-per-store
shape at half the total width (8 stores of 4 lanes each = 32 columns) and was not disassembled in
this candidate but, given it ships unmodified in upstream 0.1.11 and candidate 039's profiling
found it fast, is presumed not to spill at that width. In short: the crate's practical per-`J`-type
register budget for this loop shape caps out somewhere between the existing 32-column tier and the
64-column tier tried here, not at "8 accumulators, however wide" as the candidate description
assumed - the row-hop savings from processing 64 columns per row-visit instead of 32 are real but
too small to outweigh even a partial (3/8) spill.

## What's not done / open questions

- Whether an 8 x 4-wide (32-column) "octo-at-double-width" tier - i.e. the same multi-accumulator
  *shape* as this candidate but at the existing 32-column granularity instead of a new 64-column
  one - would fully avoid spilling and merely be a redundant reimplementation of the existing
  `..._double` tier, was not measured separately; the existing `double` tier already occupies that
  spot in the cascade, so this would only be interesting if it removed the 32-column tier's own
  (unverified) partial spill, if it has one.
- Whether `J = i64` (used for higher bit depths elsewhere in this crate, not reached by this
  candidate's `u8`/`i32` scenario) hits the register budget differently - not measured; a
  narrower-lane-per-register accumulator type would spill sooner, not later, so this is unlikely to
  flip the result but wasn't checked.
- No AVX2-explicit (`std::arch`/wide `ymm`) rewrite was attempted - this candidate stayed within
  the crate's existing "safe Rust, let LLVM auto-vectorize" style per its name (`pic-scale-safe`),
  matching the existing code's own approach; explicit intrinsics might pack 8 x i32 into a true
  256-bit YMM register and change this result, but that's a materially different, larger change
  than the one this candidate was scoped to test.
- No patch was kept applied anywhere - reverted from the main project's working tree per the
  workflow's standing rule, regardless of the (negative) result.
