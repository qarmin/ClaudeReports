# JPEG2000 IDWT horizontal filter (`hayro-jpeg2000/src/j2c/idwt.rs`)

Part of stage 2 (interpret+decode, see `../README.md`) - the horizontal half of the wavelet
reconstruction filter during JPEG2000/JPX decode. `filter_horizontal`/`filter_step_horizontal` in
`idwt.rs` was scalar, next to a fully-SIMD `filter_vertical`, doing the same 5-3/9-7 lifting math.

**Files in this folder:**
- `patch.diff` (formerly `patch_attempt_REGRESSES_8x_DO_NOT_APPLY.diff`) - the complete,
  working, correctness-tested implementation (deinterleave-based SIMD rewrite + an exhaustive
  equivalence test module). Apply it with `git apply` to reproduce/investigate the regression.
- `bench/idwt/` - self-contained benchmark project (own `Cargo.toml`, real path dependency on the
  crate under test, embeds a real 1241x1752 JP2 codestream via `include_bytes!`). This is the tool
  that produced the "Performance" numbers below.
- `bench/transpose/` - self-contained project for the earlier, separate feasibility check
  described below.

## Reproduce with zero setup

```
cd bench/idwt
taskset -c 2 cargo run --release -- 100
```

Correctness is hash-checked every iteration (aborts if a patch silently changes decode output).
Apply `patch.diff` to the real `hayro-jpeg2000` checkout first to reproduce the regression, or
leave that tree clean to reproduce the baseline.

## What was tried: deinterleave-based SIMD rewrite

Deinterleave each row once into "evens" (`scanline[0,2,4,...]`) and "odds" (`scanline[1,3,5,...]`)
buffers. Within one lifting step, the array being updated only ever reads from the *other*,
unchanged array - after deinterleaving, that read is a plain contiguous, shifted-by-one slice (no
gather/shuffle needed), so the lifting step vectorizes directly on top of `f32x8` arithmetic
already used elsewhere in this file. Re-interleave once at the end.

## Result

Real, measured regression: `hayro-bench jpx_decode_bench`, real JPX decode, `taskset -c 2`,
100 iterations, `issue391.pdf`:

| | Horizontal filter time (100 iter) | Total decode (`000_13_0.jp2`) |
|---|---|---|
| Original (scalar) | 196.5 ms | 9.5-9.8 ms/decode |
| This patch | **1,697-1,830 ms** | **20.0-20.9 ms/decode** |
| Δ | **~8.6x slower** | **~2.1x slower overall** |

Fine-grained phase split of the new code's horizontal-filter time (same methodology, temporary
instrumentation, reverted after measuring):

| Sub-phase | Time (100 iter) |
|---|---|
| deinterleave | 94 ms |
| lifting steps (the actual SIMD arithmetic) | 1,535 ms |
| interleave | 128 ms |

The lifting-step arithmetic itself - not the deinterleave/interleave data-shuffling overhead - is
the dominant cost. One real inefficiency was found and fixed along the way (see below) with no
measurable effect on this number, meaning the dominant cost is still unexplained.

## Does it change behavior/output?

Correctness verified twice, independent of the performance result:

1. **Exhaustive synthetic test** (`horizontal_filter_equivalence_tests` in the patch): both
   transform types x widths 1-80 x both `x0` parities x 4 random seeds (2560 cases), comparing the
   new implementation against a preserved copy of the original scalar code. Required a tolerance
   (`1e-4` relative, `5e-4` absolute floor) rather than bit-exact equality.
2. **Real JPX render tests** (`stream_jpx_1/3/5/6`, exact pixel diff): 1 of 4 passes unchanged;
   3 of 4 show tiny pixel differences, measured with `../tools/analyze_diff/`:

   | Test | Image size | Pixels differing | Max channel deviation |
   |---|---|---|---|
   | `stream_jpx_1` | - | 0 | 0 |
   | `stream_jpx_3` | 595x842 | 3 / 500,990 (0.0006%) | 1 |
   | `stream_jpx_5` | 512x512 | 4 / 262,144 (0.0015%) | 1 |
   | `stream_jpx_6` | 540x720 | 8 / 388,800 (0.002%) | 1 |

- **Bit-exact?** no.
- What changes: a handful of pixels (<=8 out of hundreds of thousands per test image) differ by up
  to 1 LSB per channel. Visually indistinguishable (diff mask is solid black at normal viewing
  scale). Cause: the SIMD bulk path uses real hardware FMA (`fearless_simd` dispatches to it at
  runtime, independent of compile flags) while the scalar remainder/boundary positions use
  `crate::math::mul_add`, which falls back to unfused `a*b+c` unless the crate was compiled with
  `target-feature=fma` - the existing, shipped `filter_step_vertical` already has this exact same
  bulk/remainder precision split, so this is not a new category of risk, just newly exercised on
  the horizontal axis too.
- **Is this a breaking change?** no observable API/format change; a rendered-pixel difference of
  this magnitude on JPEG2000-heavy content only.
- Project test suite: `stream_jpx_1/3/5/6` (`cargo test -p hayro-tests --release --test tests --
  stream_jpx`) - 1/4 unchanged, 3/4 differ by the sub-pixel amounts above; no other regressions
  checked at this stage given the performance result below made further test-suite work moot.

## Why the result looks the way it does

No `perf` available this session (not installed, no root to install without asking) to get an
instruction-level profile of the lifting-step loop. Suspected but unconfirmed candidates, roughly
in order of suspicion:
- Poor inlining across the `filter_step_horizontal_dispatch` -> `filter_step_horizontal_simd` ->
  closure chain, possibly causing real function-call overhead per 8-element chunk instead of a
  fused vectorized loop (the original scalar code was one flat function, no generics/closures).
- The shifted/unaligned slice reads for `neighbor` (`neighbor[k-1..]` etc.) causing consistently
  unaligned SIMD loads, which can be markedly slower on some hardware/instruction-set combinations
  than the aligned, contiguous access `filter_vertical`'s SIMD path enjoys.
- Something about processing many separate, moderate-sized (~600-element) deinterleaved buffers
  per row, vs. `filter_vertical`'s SIMD which operates across many rows per call - i.e. the
  *problem shape* this rewrite creates may simply not amortize per-call/per-loop overhead as well,
  independent of any single fixable bug.

### A real, subtle bug found and fixed along the way (kept in the diff, did not fix the regression)

First attempt had `filter_step_horizontal_simd` always receiving `evens` as the "target" (updated)
buffer and `odds` as "neighbor" - but the original code's `first_even`/`first_odd` values are
`x0 % 2` and `1 - (x0 % 2)`, which can each be either 0 or 1 depending on the tile's pixel offset
parity. Hardcoding the buffer assignment caused out-of-bounds panics for odd image/tile offsets.
Fixed via `filter_step_horizontal_dispatch`, which picks `(target, neighbor) = (evens, odds)` or
`(odds, evens)` based on the actual `first` value at each call site. Caught immediately by the
exhaustive test.

### Investigated as a possible cause, ruled out

`num_terms` (size of the vectorizable bulk region) was originally computed via
`(range).step_by(2).count()`, a live iterator call on every single lifting-step invocation
(2-6 times per row x up to 1752 rows). Replaced with an O(1) closed-form
(`(width - 1 - middle_start).div_ceil(2)`, verified against the iterator version via the
exhaustive test). This produced no measurable performance change - the regression persisted
identically before and after, so this was a real but not dominant inefficiency.

## What's not done / open questions

- Not root-caused: needs `perf record`/`perf annotate` on the lifting loop to distinguish between
  the three suspected causes above before attempting a further fix.
- An earlier, separate feasibility check (transpose the buffer, reuse the already-fast vertical
  filter, transpose back) was checked and ruled out without implementation: pure transpose
  round-trip cost (~1.4-2.3ms for the largest sub-band) was already the same order of magnitude as
  the *entire* original horizontal filter budget (~2ms/iteration across all levels), so it could
  not have paid for itself. See `bench/transpose/` (`cd bench/transpose && cargo run --release`).
- `stream_jpx_1/3/5/6` in `hayro-tests` must stay passing, or any pixel difference must be
  characterized (magnitude, % of pixels, visual proof) the same way this session did, for whoever
  revisits this.
