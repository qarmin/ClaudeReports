# AVX2 horizontal convolution kernel for CbCr (2-channel) u8

`src/factory/cbcr8.rs`'s x86 horizontal-convolution dispatch topped out at SSE4.1
(`src/sse/cbcr8.rs`), even though the sibling planar (`plane_u8`, candidate 003) and RGB/RGBA u8
horizontal passes already have both SSE4.1 and AVX2 tiers - `src/avx2/mod.rs` had no cbcr-u8
module at all. This candidate adds `src/avx2/cbcr8.rs`, widening `src/sse/cbcr8.rs`'s existing
"reorder taps into per-channel groups via `shuffle_epi8`, then `madd_epi16`, fold once at the end"
trick from 128-bit (8 taps/iteration) to 256-bit (16 taps/iteration) using `_mm256_shuffle_epi8`
with the SSE kernel's 16-byte control masks replicated into both 128-bit lanes (`_mm256_shuffle_epi8`
shuffles each lane independently with the same indices, so this applies the identical grouping to
two independent 8-tap chunks - taps 0-7 in the low lane, 8-15 in the high lane - per instruction),
and wires it into `cbcr8.rs`'s dispatch as a new AVX2 tier above SSE4.1, mirroring the tiering
already used for RGB/RGBA/plane u8. This was the one remaining cell (CN=2, 8-bit) of the CN x
bit-depth grid candidates 002/003 already covered for the other three combinations - see
`___OPTIMIZATIONS___/IDEAS.md`'s now-removed backlog entry.

## Harness extension needed for this candidate

`app/benches/resize_cbcr/main.rs` had no criterion case at all for `u8`/`CN=2` (`ImageStore<u8,
2>` via `plan_cbcr_resampling`) - only `"Pic scale CbCr10: Lanczos 3"` and `"...CbCr16..."`, both
`u16`/`bit_depth<=12` or `>12`, neither of which reaches this patch's code path. A new
`"Pic scale CbCr8: Lanczos 3"` `bench_function` was added (same asset, same
Lanczos3/PreferSpeed/`ThreadingPolicy::Single` shape as the existing two cases, using
`img.to_luma_alpha8()` as a stand-in 2-channel u8 source). This is measurement-only instrumentation,
not part of `patch.diff` - see `bench_addition.diff` in this folder, applied by this candidate's own
`justfile` the same way the `app/Cargo.toml` avx/sse feature flip already is (see
`___OPTIMIZATIONS___/README.md`).

## What was tried

Only one variant: the AVX2 widening described above. Bench: the new `"Pic scale CbCr8: Lanczos 3"`
criterion case (`resize_cbcr`, `harness = false`), Lanczos3/`WorkloadStrategy::PreferSpeed`/
`ThreadingPolicy::Single`, 4928x3279 nasa asset resized to 1/4 (as luma-alpha8, used as a stand-in
2-channel u8 source). Two prebuilt binaries (`cargo bench -p app --bench resize_cbcr --no-run`,
baseline vs. patched source, `app/Cargo.toml` temporarily given `avx`/`sse` features) were compared
with `tools/paired.sh` (`RUNS=9`, alternating A/B).

`resize_cbcr` was already flagged in this workspace's `README.md`/`projects/pic-scale.md` as the
noisiest bench of the four (bimodal per-sample clustering observed for candidate 002 too), so 3
pinned repeats and 2 unpinned repeats were run rather than the usual 2/1.

| Variant | single-core (`taskset -c 3`) | multi-core (unpinned) | notes |
|---|---|---|---|
| baseline (SSE4.1 only) vs. AVX2 (this patch), repeat 1 | median 7.668 -> 8.092 ms (**+5.5%**, B *slower*), 7/9 pairs B faster | median 8.279 -> 7.732 ms (-6.6%), 8/9 pairs | pinned repeat 1: `paired.sh` flags "NOT consistent" (medians disagree with the 7/9 win count - noise, not a real regression, per the pattern below) |
| same, repeat 2 | median 8.335 -> 7.844 ms (**-5.9%**), 8/9 pairs, "consistent" | median 5.601 -> 5.348 ms (-4.5%), 9/9 pairs, "consistent" | |
| same, repeat 3 (pinned only) | median 5.835 -> 5.583 ms (**-4.3%**), 8/9 pairs, "consistent" | - | |

Raw pairs (ms), pinned:
- Repeat 1: `A: 6.3063 9.1428 9.2431 8.5376 6.7119 5.8229 5.7787 7.6685 8.3147` / `B: 8.1262 8.4375
  8.269 6.0006 5.5659 5.3602 5.3814 8.3892 8.0922`
- Repeat 2: `A: 8.5148 8.1864 8.202 6.0711 8.335 8.3601 8.4225 8.5217 5.7544` / `B: 7.8723 8.1798
  7.7757 7.2737 7.8993 7.9219 7.8441 6.1521 5.5042`
- Repeat 3: `A: 5.7176 5.8346 6.0249 5.7499 5.7589 5.8134 7.7842 8.5519 8.3087` / `B: 5.5149 5.5834
  5.4478 5.7085 5.5172 5.4739 7.7923 7.8612 7.8246`

Raw pairs (ms), unpinned:
- Repeat 1: `A: 7.3478 8.2789 8.4403 8.6134 8.5201 8.6082 5.6894 5.7509 5.6014` / `B: 5.7509 8.3578
  8.0586 8.3057 8.0819 7.7321 5.4578 5.4744 5.3192`
- Repeat 2: `A: 5.6961 5.6526 5.6662 5.623 5.5954 5.6009 5.5432 5.4705 5.5858` / `B: 5.3048 5.3983
  5.4277 5.4088 5.4545 5.3477 5.2244 5.2385 5.3208`

Each `A`/`B` sequence shows the same bimodal clustering (a ~5.5-6ms mode and a ~7.7-8.6ms mode)
within a single 9-run sequence, for both baseline and patched binaries alike - host-level noise
(matching the pattern already documented for `resize_cbcr` in this workspace's `README.md`/
`projects/pic-scale.md`), not something caused by the patch. `uptime`/`/proc/loadavg` stayed
between ~5.7 and ~15 during these runs on this 24-logical-core box.

## Result

A real but modest and noisier-than-usual improvement: **roughly -4% to -7% median wall-clock**
for the `"Pic scale CbCr8: Lanczos 3"` case, in both single-core (pinned) and unpinned
measurements, across 2 of 3 pinned repeats and both unpinned repeats (all "verdict: consistent"
per `paired.sh`'s own 8/9-or-better rule). One pinned repeat did not clear that bar (`+5.5%`
median with only 7/9 pairs in the patch's favor) - read as the same host-level noise visible in
that repeat's raw samples (see bimodal clustering above) rather than a real regression, given the
other 4 of 5 repeats agree in direction and rough magnitude. This is a smaller effect than
candidate 003's CN=1 plane widening (-14% to -20%) or candidate 002's from-scratch CbCr16 SSE4.1
kernel (-18% to -26%) - see "Why the result looks the way it does" below.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone `bit_exact_check/` crate (13 size/scale combinations x
  PreferSpeed/PreferQuality, sweeping tap counts through 1/2/4/8/16/17+ per output pixel via a mix
  of up/down scale ratios, plus a 1x1 case and heights/widths not a multiple of the batch sizes to
  exercise every remainder path) resamples a synthetic 2-channel u8 (CbCr-shaped) source and hashes
  the concatenated output with `DefaultHasher`. Built once against baseline source and once against
  patched source (both with `sse`+`avx` features enabled): both print the identical
  `FINAL_HASH=10798212667270458715` and identical per-case `first8=[...]` samples.
- **Is this a breaking change?** No - internal dispatch only, no public API change.
- Project test suite: not run separately - this candidate's own `bit_exact_check` (see above) is
  the correctness check for this code path; no `cargo test` run was done for the wider suite (no
  behavior change was expected or found outside `HorizontalFilterPass<u8, f32, 2>` on
  x86_64+avx2).

## Why the result looks the way it does

Same shape as candidates 002/003, but the smaller win relative to those two is expected: unlike
plane u8 (CN=1, candidate 003), CbCr u8 needs a real per-channel de-interleave (`shuffle_epi8`) on
every tap batch, not just a zero-extend - so widening to 256-bit doubles the shuffle/madd work per
instruction but the *relative* overhead per tap (loads, the shuffle itself, weight rearrangement)
doesn't shrink as much as it does for the shuffle-free CN=1 case. And unlike CbCr16 (candidate
002), this candidate is competing against an *existing* SSE4.1 kernel that was already reasonably
tight, not filling a scalar-only gap - closer in shape to candidate 003's "widen an already-decent
kernel" case than candidate 002's "add a kernel where there was none" case, which is consistent
with the smaller, noisier delta measured here.

## What's not done / open questions

- Only the "rows-4" and "single-row" u8 horizontal entry points were added; no AVX512 tier (out of
  scope - see candidate 001's regression finding for the RGBA u8 dot-product shape on this Zen4
  box before assuming a VNNI-based CbCr8 kernel would win here).
- Not re-measured on non-x86_64 or on hardware without AVX2 (dispatch already falls back to
  SSE4.1/scalar correctly there; no behavior change expected, not specifically re-verified this
  session).
- `cargo test`/snapshot suite for the wider crate wasn't run before/after - only the
  `bit_exact_check` hash comparison targeting this exact code path.
- `resize_cbcr`'s host-level noise (see raw pairs above) made this candidate's measurement
  noticeably less clean than candidates 001/003's plane/RGB benches - a longer quiet window or
  more repeats would tighten the confidence interval on the exact percentage, though the direction
  (a real, if modest, win) held in 4 of 5 repeats.
