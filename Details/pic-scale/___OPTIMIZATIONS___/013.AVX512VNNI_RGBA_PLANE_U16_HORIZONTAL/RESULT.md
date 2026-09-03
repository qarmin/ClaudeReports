# AVX-512VNNI dispatch branch for RGBA u16 (CN=4) and Plane u16 (CN=1) horizontal convolution (bit_depth<=12)

Follow-up to candidate 012, which wired up and measured the `avx512vnni`+`avx512vl` (EVEX
`vpdpwssd`) counterpart of the existing `avxvnni`-gated horizontal dot-product kernel for RGB u16
(CN=3) only, and found no separable effect on this Zen4 dev box. `src/factory/rgba_u16.rs`
(CN=4) and `src/factory/plane_u16.rs` (CN=1) have the identically-shaped gate
(`is_x86_feature_detected!("avxvnni")` guarding `src/avx2/rgba_u16_lb.rs`/`plane_u16_lb.rs`,
never firing on this box as shipped since it has real AVX-512, not narrow `avxvnni`) - this
candidate measures both of those, closing the gap candidate 012 explicitly left open.

Reading both kernels first (same discipline candidate 012 used) confirmed the same "identical
surrounding shuffle/load code, only the final accumulate instruction differs" shape:

- `rgba_u16_lb.rs`'s row4/row1 kernels already use a `const D: bool` generic exactly like
  `rgb_u16_lb.rs` did, selecting `_mm256_dot16_avx_epi32::<D>`/`_mm_dot16_avx_epi32::<D>` (VEX
  `vpdpwssd_avx` fused-multiply-add vs. `vpmaddwd`+`vpaddd`) - same shuffle-heavy CN=4 shape as
  RGB's CN=3, just a 4-lane-per-pixel shuffle table instead of 3.
- `plane_u16_lb.rs` (CN=1) is actually simpler than either: since a whole tap fits one `u16`, the
  kernel does **no shuffle at all** - `acc_1/2/4/8_dot` just zero-extend-load 1/2/4/8 contiguous
  `u16`s directly into `_mm_dot16_avx_epi32::<D>`. All four load widths stay within a single
  128-bit XMM register (no `__m256i` used anywhere in this file, despite the enclosing function
  being gated on `avx2` for tier-selection purposes only).

## What was tried

Added `avx512vnni`+`avx512vl`-gated counterparts of both existing `avxvnni` row4/row1 kernel
pairs, following candidate 012's approach exactly: new `_mm256_dot16_avx512_epi32`/
`_mm_dot16_avx512_epi32` helpers in `src/avx2/utils.rs` (EVEX `_mm256_dpwssd_epi32`/
`_mm_dpwssd_epi32`, `#[cfg(feature = "avx512")]`, byte-identical to candidate 012's own addition
of the same name - not touching the existing generic `_mm_dot16_avx_epi32`/
`_mm256_dot16_avx_epi32` shared by dozens of other call sites), plus fully separate
`acc_*_dot_512` free functions and `Row4ExecutionHandler512`/`OneRowExecutionUnit512` structs in
`rgba_u16_lb.rs` and `plane_u16_lb.rs` (byte-for-byte the same load/shuffle logic as the existing
`avxvnni`-gated ones, calling the EVEX helper unconditionally). Wired into
`src/factory/rgba_u16.rs`'s and `src/factory/plane_u16.rs`'s `HorizontalFilterPass::horizontal_plan`,
checked before the existing `avxvnni` branch in each, same order/pattern as candidate 012.

Not touched: `plane_u16.rs`'s *second* `avxvnni` gate (`convolve_column_lb_avx2_u16_vnni`, the
vertical/column path) - flagged separately in `IDEAS.md` as a structurally different kernel shape
that would need its own read before assuming this result transfers; still untouched here.

## Bench

`app/benches/resize_rgba/main.rs`, `"Pic scale RGBA10 without alpha: Lanczos 3"` case
(`ResamplingFunction::Lanczos3`, `ThreadingPolicy::Single`, 4928x3279 -> 1232x819, bit_depth=10,
`../assets/nasa-4928x3279-rgba.png`, `plan_rgba_resampling16(..., false, 10)`), and
`app/benches/resize_plane/main.rs`, `"Pic scale Plane10: Lanczos 3"` case (same asset via
`to_luma16()`, `WorkloadStrategy::PreferSpeed`, `plan_planar_resampling16`). Both already exist in
the harness (no bench-file changes needed) and reach both the row4 and row1/remainder kernels this
patch touches. `app/Cargo.toml` needed `avx`/`sse`/`avx512` added to the `pic-scale`
path-dependency's feature list to reach any x86 SIMD tier at all (same instrumentation as
candidates 001/012) - included in `patch.diff`. Single-core = `taskset -c 3`-pinned; multi-core =
same unpinned.

Paired via `___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B, 9 pairs each; run from `app/`
so the bench's relative asset path resolves), point estimate = criterion's reported mean (middle
of `[low mean high]`).

### RGBA10 without alpha

| Variant | pinned (`taskset -c 3`) | unpinned |
|---|---|---|
| baseline (AVX2 fallback, madd+add; `avxvnni` gate never fires) | run1: 18.635 ms; run2: 16.282 ms | run1: 16.283 ms; run2: 15.881 ms |
| patched (new `avx512vnni`+`avx512vl` branch, EVEX `vpdpwssd`) | run1: 18.366 ms (-1.4%, B faster 6/9); run2: 16.221 ms (-0.4%, B faster 5/9) | run1: 16.494 ms (+1.3%, B faster 5/9); run2: 15.629 ms (-1.6%, B faster 9/9) |

3 of 4 repeats landed outside `paired.sh`'s consistency bar (6/9, 5/9, 5/9); one unpinned repeat
hit 9/9 (`consistent`) at -1.6%. Raw pairs:

- Pinned run1: `A: 18.36 18.493 19.024 16.551 18.806 18.635 19.281 16.353 27.33` /
  `B: 19.158 18.626 18.084 16.118 18.942 18.474 18.252 16.023 18.366`
- Pinned run2: `A: 16.254 16.187 16.282 16.561 16.183 16.499 16.247 16.331 16.882` /
  `B: 15.976 15.985 17.055 16.089 16.954 16.221 16.168 16.393 17.077`
- Unpinned run1: `A: 19.413 16.283 16.265 27.778 16.026 16.329 16 16.268 17.488` /
  `B: 16.494 16.07 16.326 22.884 17.054 17.163 15.762 17.224 15.913`
- Unpinned run2: `A: 15.877 16.47 16.363 16.132 15.965 15.838 15.881 15.851 15.684` /
  `B: 15.831 15.949 16.17 15.898 15.433 15.574 15.629 15.443 15.534`

### Plane10

| Variant | pinned (`taskset -c 3`) | unpinned |
|---|---|---|
| baseline | run1: 3.998 ms; run2: 10.088 ms | run1: 3.803 ms; run2: 3.830 ms |
| patched | run1: 3.932 ms (-1.7%, B faster 4/9); run2: 9.135 ms (-9.4%, B faster 6/9) | run1: 3.732 ms (-1.9%, B faster 9/9); run2: 3.774 ms (-1.5%, B faster 7/9) |

3 of 4 repeats landed outside the consistency bar; one unpinned repeat hit 9/9 at -1.9%. Pinned
run2 shows the same bimodal host clustering seen elsewhere in this workspace (`resize_cbcr`,
`resize_plane`'s wide-filter case) - values ranged 7.2-11.7ms across the run vs. run1's 3.7-6.7ms,
for both binaries alike. Raw pairs:

- Pinned run1: `A: 3.6997 3.8036 3.6976 3.9985 3.8297 4.5123 6.6851 5.107 6.5778` /
  `B: 3.6592 3.7041 3.8781 4.1017 4.0235 6.5118 3.9318 6.534 3.758`
- Pinned run2: `A: 11.542 11.687 10.985 10.088 8.6857 9.7308 8.1653 9.4147 10.102` /
  `B: 8.7398 9.1135 9.2795 9.1354 9.1631 8.276 10.362 7.2151 11.182`
- Unpinned run1: `A: 3.8311 3.7288 3.7475 3.7606 3.8034 3.8245 3.8171 3.7923 3.8157` /
  `B: 3.7683 3.6827 3.665 3.7092 3.7325 3.7095 3.7582 3.755 3.7731`
- Unpinned run2: `A: 3.8237 3.8352 3.828 3.7795 3.8342 3.8296 3.7088 5.0228 4.1441` /
  `B: 3.7355 3.7383 3.762 3.8012 3.7756 3.7739 4.5973 4.4802 3.7573`

## Result

For both CN=4 (RGBA) and CN=1 (Plane), 3 of 4 repeats each (2 pinned, 2 unpinned) landed outside
`paired.sh`'s consistency bar, with the point estimate swinging in a narrow band (RGBA: -1.6% to
+1.3%; Plane: -9.4% to -1.7%, the one large-magnitude reading confounded by visible bimodal host
noise present in both binaries). Exactly one repeat per case (both happened to be the second
unpinned repeat) reached the 9/9 "consistent" bar, both showing a small (-1.6%/-1.9%) win for the
EVEX path. This is the same overall shape candidate 012 found for RGB u16 (CN=3): no reliably
separable effect either direction on this host for this kernel/input, with the surrounding
shuffle/load code identical between baseline and patched either way - consistent with the prior
that this kernel-shape's null result generalizes across CN, not just for RGB.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone correctness-check crate (`bit_exact_check/` in this folder)
  resizes the same `nasa-4928x3279-rgba.png` asset with the exact transforms used by both bench
  cases (`Lanczos3`, bit_depth=10, RGBA u16 CN=4 without-alpha and Plane u16 CN=1), built once
  against baseline source and once against patched source: both produce `rgba16: len=4036032
  hash=d9c2523d1b23e3d0` and `plane16: len=1009008 hash=0ee2599b9387b63f`.
- **Is this a breaking change?** No public API change either way.
- Project test suite (`cargo test --release`, no avx512 feature): baseline 66 passed / 0 failed
  (lib) + 0 passed / 20 ignored (doctests); patched: identical, 66 passed / 0 failed + 0 passed /
  20 ignored.
- Project test suite (`cargo test --release --features avx512`, exercises the new cfg-gated
  code): baseline 68 passed / 0 failed (lib) + 0 passed / 20 ignored (doctests); patched:
  identical, 68 passed / 0 failed + 0 passed / 20 ignored.

## Why the result looks the way it does

Same explanation as candidate 012, now confirmed to hold across both other CN values sharing this
kernel shape: the shuffle/load reformatting dominates the loop body and is byte-identical between
baseline and patched paths (for Plane u16 there isn't even a shuffle - just a zero-extend load -
making this the cleanest possible "swap one instruction only" comparison in this workspace, and it
still didn't produce a reliable signal). Saving one instruction per tap (`vpdpwssd` EVEX fused
multiply-add vs. `vpmaddwd`+`vpaddd`) inside an already load/shuffle-dominated loop isn't large
enough, relative to this host's own measurement noise for these bench cases, to separate from the
noise floor across 4 repeats each.

## What's not done / open questions

- `plane_u16.rs`'s vertical/column `avxvnni` gate (`convolve_column_lb_avx2_u16_vnni`) is a
  different kernel shape (accumulates down a column of taps, not across shuffled-lane pixels) and
  still untouched/unmeasured - flagged again in `IDEAS.md`.
- Not tested on real `avxvnni` hardware (Alder Lake+/Sierra Forest) - same caveat as candidates
  001/012, this only establishes the EVEX/`avx512vnni` path's behavior on Zen4.
- Given the consistently noisy, inconclusive results across both candidates 012 and 013 (6 of 8
  total repeats across all three CN values landing outside the consistency bar, all in a tight
  band under ~2% except for one confounded-by-noise Plane pinned repeat), a `perf stat`
  instruction-count comparison instead of wall-clock could more directly test the "one instruction
  saved per tap doesn't matter" hypothesis - not done here, per SKILL.md's bounded-investigation
  guidance (pitfall #10), since the kernel-shape question this candidate exists to answer was
  already settled by reading the code before measuring.
