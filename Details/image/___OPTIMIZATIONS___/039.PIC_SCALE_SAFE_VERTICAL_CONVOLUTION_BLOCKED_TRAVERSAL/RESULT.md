# pic-scale-safe vertical convolution: wider column-block tier for cache locality

Candidate 015 in `IDEAS.md` (dated 2026-08-24): callgrind profiling of `DynamicImage::resize(400,
400, Lanczos3)` on a 4032x3024 source showed `fixed_point_vertical::column_handler_fixed_point`
at 62.6% of instructions vs. `fixed_point_horizontal::convolve_row_handler_fixed_point_4` at only
15.3% - a ~4x imbalance hypothesized to come from the vertical pass's strided memory-access
pattern (each destination row reads `bounds.size` source rows spaced `src_stride` bytes apart,
processed in small 8/16/32-element column chunks, so each source row is "hopped to" from cold
cache state `total_width / chunk_size` times per destination row instead of once). Tried a
blocked/tiled traversal: add a wider (512-element) column-block tier to
`column_handler_fixed_point`'s existing cascading buffer-size dispatch in
`fixed_point_vertical.rs`, mirroring candidate 007's blocked-transpose technique for
`rotate90_in`/`rotate270_in`.

## What was tried

Vendored `pic-scale-safe` 0.1.11 (following candidate 033's pattern: full `src/` copy into
`vendor/pic-scale-safe/`, wired in via `[patch.crates-io]`) and added one new tier to
`column_handler_fixed_point` (`fixed_point_vertical.rs`), reusing the existing generic
`convolve_column_handler_fixed_point_direct_buffer::<T, J, BUFFER_SIZE>` function unmodified with
`BUFFER_SIZE = 512`, tried before the existing 64(aarch64)/32/16/8/1 tiers:

```rust
while cx + 512 <= total_width {
    convolve_column_handler_fixed_point_direct_buffer::<T, J, 512>(
        src, src_stride, dst, weight, bounds, bit_depth, cx,
    );
    cx += 512;
}
```

Measured `DynamicImage::resize_exact(400, 300, Lanczos3)` and `Triangle` on a synthetic 4032x3024
`RgbImage` (`benches/imageops.rs`'s `bench_resize_dynamic`, reused from candidates 002/031-038;
this candidate's own bundled addition to that bench, `4032x3024->400x300`, was already present in
the pattern established by candidate 033). `S` = registry pic-scale-safe 0.1.11 unmodified. `E` =
vendored, with the 512-element tier added. Paired A/B via `___OPTIMIZATIONS___/tools`-style
`paired_pic_scale.sh` (reused from candidate 033's `raw/`), 5 alternating runs per comparison,
`taskset -c 3` for single-core, unpinned for multi-core (24 logical cores, `uptime` load 1.1-7.6
during this session, well under core count). This code path (`convolve_trampoline_fixed_point`'s
non-rayon branch) runs single-threaded regardless of the crate's own `rayon` feature, since
`pic-scale-safe/rayon` is not currently forwarded by `image`'s `Cargo.toml` (see the `TODO`
comment above the `rayon` feature line) - so "multi-core" here means unpinned wall-clock, not
parallel execution of this function itself.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S: registry 0.1.11 (baseline) | 18.786 ms | 18.698 ms | Lanczos3, 4032x3024->400x300 |
| E: +512-element vertical block tier | 37.083 ms (**+97.4%**, 0/5 E-faster) | 37.079 ms (**+98.3%**, 0/5 E-faster) | Lanczos3, same scenario |
| S: registry 0.1.11 (baseline) | 7.323 ms | n/a | Triangle, same source/dest, single-core only |
| E: +512-element vertical block tier | 14.937 ms (**+104.0%**, 0/5 E-faster) | n/a | Triangle |
| E vs E (same-binary control) | 37.633 -> 37.369 ms (-0.7%, 4/5 "wins") | n/a | noise floor for the Lanczos3 comparison above |

Raw logs under `raw/`: `singlecore_S_vs_E_Lanczos3.log`, `multicore_S_vs_E_Lanczos3.log`,
`singlecore_S_vs_E_Triangle.log`, `control_singlecore_E_vs_E_Lanczos3.log`.

## Result

**Decisive regression, not a win.** `E` is ~2x *slower* than `S` on both single-core and
multi-core, for both filters tried (Lanczos3 +97-98%, Triangle +104%), 0 of 5 paired runs faster
in every comparison. The same-binary control (`E` vs `E`) shows a -0.7% noise floor, so this is
far outside measurement noise - the patch is real and reliably worse, not a wash.

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified via a temporary example driver (`DynamicImage::resize_exact` on
  the same synthetic 4032x3024 source, at `400x300`/`64x48`/`8x6`, for
  `Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian`/`Nearest`) - all 15 output hashes identical between
  `S` and `E` (`raw/hashes_S.txt` vs `raw/hashes_E.txt`). Expected: the 512-element tier calls the
  exact same per-column accumulation function/weights/tap order as the smaller tiers, just with a
  different column-chunk boundary, which cannot change the arithmetic result.
- **Is this a breaking change?** No (not applicable - not the winning variant, and even if it
  were, bit-exact output means no observable behavior change).
- Project test suite (`cargo test --release --no-fail-fast`): **622 passed, 1 failed, 4 ignored,
  identical before and after.** The one failure, `regression::resizing_with_catmul`, is
  **pre-existing and unrelated to this patch** - reproduces identically on a fully clean checkout
  with zero `___OPTIMIZATIONS___` patches applied (confirmed via `git stash -u` before running
  `cargo test`). Filed as `___OTHER___/003.RESIZE_EXACT_CATMULLROM_OFF_BY_ONE_ROUNDING` (a ±1 LSB
  rounding drift on 0.15% of channels between `DynamicImage::resize_exact`/`CatmullRom`'s output
  and the crate's own committed reference PNG) since it's a genuine, currently-failing test on
  `main`, not something this candidate introduced or fixed.

## Why the result looks the way it does

The profiling-based hypothesis (cross-row cache-hopping dominates) undersold a countervailing
cost: `column_handler_fixed_point`'s existing small tiers (8/16/32, 64 on aarch64) are not
arbitrary - `BUFFER_SIZE = 8` for the `J = i32` accumulator is exactly one AVX2 register's worth
(8 x 32-bit lanes). For those sizes, LLVM can keep the whole `direct_store` accumulator resident
in vector registers for the entire tap loop (`bounds.size` taps), touching memory only once per
tap for the (small) source read and once at the very end to write the narrowed result - no
accumulator spill. Raising `BUFFER_SIZE` to 512 makes that impossible: 512 x i32 (2 KiB) cannot
live in registers, so `direct_store` spills to the stack/L1, and *every* tap's inner loop becomes
a full 512-element read-modify-write against that spilled array instead of a handful of
register-to-register FMAs. For a support width around 40-60 taps (Lanczos3/Triangle downscaling
3024->300, ~10x), that is 40-60x more L1 traffic against a 2 KiB working set, which comfortably
outweighs the row-hop savings the larger block was meant to buy - matching the observed ~2x
regression. The blocked-transpose technique that won for candidate 007's rotate90/270 doesn't
transfer directly here: that candidate's tiles held raw pixel values (no register-resident
accumulator to displace), while this one's inner loop is a reduction whose accumulator was already
sized to fit the hardware's vector registers.

## What's not done / open questions

- A genuine cache-locality win for this function likely needs to keep the *register-resident*
  accumulator width (8/16/32) while amortizing the row-hop cost some other way - e.g. processing
  several adjacent 8/16-wide sub-blocks per tap-visit (multiple small accumulators kept live
  together across one pass over the taps) rather than one giant accumulator. Not attempted here;
  register pressure (8 x 32-byte AVX2 accumulators already approaches the 16 YMM registers
  available) makes this fiddly enough to warrant a dedicated candidate with disassembly
  verification, not a quick follow-up within this one. Left in `IDEAS.md`.
- Smaller intermediate block sizes (e.g. 64, 128) between the existing 32-tier and the tried
  512-tier were not measured - the register-spill explanation predicts these would also regress
  (any size that can't fit in registers pays the same qualitative cost), but that wasn't verified
  empirically; noted as a cheap follow-up if someone wants the data point.
- No patch was kept applied anywhere - this candidate's change was reverted from the main
  project's working tree per the workflow's standing rule, regardless of the (negative) result.
