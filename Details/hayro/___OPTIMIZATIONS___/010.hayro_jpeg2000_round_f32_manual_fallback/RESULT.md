# `math::round_f32`: use the manual (libm-free) fallback unless `sse4.1`/`neon` is enabled

`hayro-jpeg2000/src/math.rs::round_f32`'s `std` path called `x.round()` unconditionally whenever
the `std` feature was on, regardless of target CPU features. On `x86_64-unknown-linux-gnu`'s
default target-feature set (confirmed via `rustc --print cfg`: only `sse`/`sse2`/`fxsr`, no
`sse4.1`), `f32::round()` has no hardware instruction to lower to and LLVM emits a call to a
`roundf`-family libm routine instead. Candidate `008`'s callgrind profiling (same session's
IDWT investigation) attributed ~130-160M instructions (~12-15% of total decode) to `roundf`-family
symbols, traced to `interleave_and_convert`'s final `f32`->`u8` conversion in `lib.rs`. The crate
already ships a manual, branch-based fallback (used on `no_std` builds) that avoids the libm call
entirely; this candidate gates the `std` fast path on `sse4.1`/`neon` actually being present and
falls back to (a self-contained copy of) the manual implementation otherwise, matching the
existing pattern already used by `mul_add` in the same file for the analogous `fma` gate.

## What was tried

Bench: `hayro-bench`'s existing `jpx_decode_bench` binary (real JPX decode, no synthetic harness
needed), run against `hayro-tests/pdfs/load/issue391.pdf` (4 embedded JPX streams, largest is
1241x1752). Added a temporary FNV-1a hash printout of each decoded output buffer to the bench
(not part of `patch.diff` - reverted before finishing; trivial enough to not warrant a
`README.md` "instrumentation added" entry, unlike the larger diffs already listed there) to check
bit-exactness alongside timing. `taskset -c 2`, 200 iterations/round, 5 rounds/variant for
single-core, 3 rounds/variant for multi-core (unpinned) - each baseline and patched pair measured
twice (10 total single-core rounds, 6 total multi-core rounds per variant) per MEASUREMENT.md.

| Variant | single-core p50 (ms/iter, 10 rounds) | multi-core p50 (ms/iter, 6 rounds) | notes |
|---|---|---|---|
| baseline (`x.round()` unconditional) | 15.06 (range 14.74-15.40) | 15.10 (range 14.94-15.69) | |
| patched (manual fallback when no `sse4.1`/`neon`) | 14.57 (range 14.46-15.11) | 14.76 (range 14.71-15.23) | |

Per-image breakdown (one representative 200-iteration run each, `taskset -c 2`):

| Image | baseline ms/decode | patched ms/decode | delta |
|---|---|---|---|
| `000_13_0.jp2` (1241x1752, 1 channel - the JPEG2000-heavy one) | 9.813 | 9.723 | -0.9% |
| `001_14_0.jp2` (621x876, 1 channel) | 2.196 | 2.120 | -3.5% |
| `002_15_0.jp2` (188x212, 3 channels) | 1.206 | 1.198 | -0.7% |
| `003_16_0.jp2` (723x82, 3 channels) | 2.151 | 2.111 | -1.9% |

## Result

Small, consistent, reproducible win: **-3.2% single-core p50**, **-2.2% multi-core p50** for the
whole 4-image `jpx_decode_bench` run. Both variants' round-to-round spread within the same binary
(~1-3%) partially overlaps the delta, but every one of the 10 patched single-core rounds and 5 of
6 patched multi-core rounds beat every corresponding baseline round's median - the direction is
consistent even though the two distributions aren't perfectly separated. This is much smaller
than the ~12-15% instruction-count share candidate 008's callgrind run attributed to
`roundf`-family symbols; as with candidate 008's own IDWT finding, retired-instruction share is
not a 1:1 wall-clock proxy (branch-heavy manual fallback code vs. a single call instruction have
different real-hardware costs that `Ir`-only profiling doesn't capture).

## Does it change behavior/output?

- **Bit-exact?** yes. FNV-1a hash of each of the 4 decoded output buffers is identical between
  baseline and patched builds across every run this session (`000`: `0a437d805a8c66c2`, `001`:
  `ce7bf66b27dca6cc`, `002`: `65c50ef646ffbfe0`, `003`: `11e4af9812b2dbc1`) - expected, since
  "round half away from zero via floor(x+0.5)" and IEEE `roundf` agree for every finite value that
  isn't already exactly on a tie boundary representable differently in the two implementations,
  and none of this decoder's inputs hit that edge case.
- **Is this a breaking change?** no - `round_f32` is `pub(crate)`, not part of any public API.
- Project test suite: `cargo test -p hayro-jpeg2000 -p hayro-tests --release --test tests --
  stream_jpx` - **1 passed, 3 failed both before and after this patch, with identical pixel-diff
  counts** (`stream_jpx_3`: 65974, `stream_jpx_5`: 91173, `stream_jpx_6`: 29580) - these are the
  same pre-existing, unrelated failures candidate 008 already confirmed exist on a clean checkout
  in this environment.

## Why the result looks the way it does

The win is real but modest relative to the instruction-count share suggested by profiling,
because `round_f32` is only the final `f32`->`u8` conversion step, not the bulk of decode work
(IDWT filtering, entropy decoding, etc. dominate the rest of `000_13_0.jp2`'s 9.7ms). The
per-image deltas roughly track how much of each image's total time is that final conversion
relative to everything else - `001` and `003` (smaller/narrower images, proportionally more time
in "wrap-up" work relative to their filter passes) show a larger relative win than `000` (the
biggest image, dominated by IDWT).

## What's not done / open questions

- `hayro-syntax/src/filter/jpx.rs` has its own separate, near-identical `round_f32` (in
  `hayro-syntax/src/math.rs`) used for palette-based JPX color remapping - same `std`-path
  pattern, not touched by this candidate (out of scope: a different crate, a much smaller/less hot
  loop). Added to `IDEAS.md`.
- Not verified with `sse4.1` actually enabled (e.g. `-C target-feature=+sse4.1` or
  `target-cpu=native` on this AVX-512-capable Ryzen 9 7900) whether the gate correctly keeps using
  the hardware `x.round()` path there and whether that's still faster than the manual fallback in
  that case - the target this session cares about (default `x86_64-unknown-linux-gnu`) doesn't
  have it, so the `sse4.1` branch is untested this session, only compiled-and-trusted-by-pattern
  (it's unchanged from the original `std` branch's own logic, just newly gated).
