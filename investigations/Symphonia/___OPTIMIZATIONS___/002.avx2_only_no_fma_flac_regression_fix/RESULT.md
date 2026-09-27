# mp3/ogg speedup without FLAC regression: `-C target-feature=+avx2` (no `+fma`)

Follow-up to candidate 001, which found `-C target-feature=+avx2,+fma` gives mp3 ~-16-17% and ogg
~-5-6% but flac ~+2.5% regression. This candidate checks whether `+avx2` alone (dropping `+fma`)
keeps the mp3/ogg wins while avoiding the flac regression - per candidate 001's own "What's not
done" backlog item.

## What was tried

Same harness/testdata as candidate 001 (`___OPTIMIZATIONS___/README.md`), same repeat-run
discipline (2 independent runs per format, `taskset -c 2`, 20 iterations each), comparing
`+avx2` (no fma) against the same baseline and `+avx2,+fma` numbers already recorded in
candidate 001.

| format | baseline p50 | `+avx2` only p50 | `+avx2,+fma` p50 (candidate 001) |
|---|---|---|---|
| mp3  | ~38.6-38.9ms | run1 32.70ms / run2 32.81ms (**~-15%**) | ~32.5-32.8ms (~-16-17%) |
| flac | ~21.3ms | run1 21.36ms(min)/26.83ms(p50, one slow outlier) / run2 21.68ms (**~flat, matches baseline**) | ~21.8ms (+2.5% regression) |
| ogg  | ~28.1-28.4ms | run1 27.01ms / run2 28.07ms (**mixed, run2 essentially flat**) | ~26.6-26.9ms (~-5-6%) |
| wav  | ~13.3-13.6ms | run1 13.58ms / run2 13.57ms (flat, noise-level) | ~13.4-13.9ms (flat) |

## Result

`+avx2` alone reproduces almost all of mp3's win (~-15% vs. baseline, vs. ~-16-17% with `+fma`
added) while flac's min/one of its two runs matches baseline almost exactly (21.36-21.68ms vs.
baseline ~21.3ms) - the regression seen with `+fma` added is gone or at worst within noise. Ogg's
result is less clean: one run showed a ~4% improvement, the other was flat within its own p95
spread - unlike mp3's clearly-outside-noise win, ogg's `+avx2`-only benefit is not confidently
distinguishable from run-to-run system noise on this machine, so it's reported as "mixed/unclear"
rather than claimed as a win. Only wall-clock time was measured.

## Does it change behavior/output?

- **Bit-exact?** yes - identical hash for every format, every run, matching candidate 001 and the
  baseline hash.
- **Is this a breaking change?** Same category as candidate 001: a build-flag-only change, not a
  source/API change. `+avx2` alone requires an AVX2-capable CPU (Haswell/Excavator, ~2013+) to run
  without `SIGILL` - a narrower requirement than `+avx2,+fma` needs (FMA3 support specifically,
  same rough hardware generation in practice on x86_64, so this doesn't meaningfully widen or
  narrow the deployment risk candidate 001 already flagged).
- Project test suite: not re-run for this candidate specifically (identical source to candidate
  001/baseline, only the RUSTFLAGS combination differs, and 001 already confirmed 106/106 passing
  under a target-feature build) - N/A, no source changed.

## Why the result looks the way it does

FMA fuses a multiply and add into one rounding step, which changes the *scheduling* LLVM's
auto-vectorizer chooses for a dependency chain, not just the raw instruction count - for FLAC's
smaller/less-uniform Rice-coding and fixed/LPC-prediction kernels (very different in shape from
MP3's dense, fixed-iteration DCT/IMDCT loops - see candidate 001's disassembly artifact), that
different scheduling apparently interacts badly with the surrounding integer-heavy bitstream code,
while plain wider-register AVX2 (no fused rounding step, same scheduling shape as the SSE2
baseline just wider) doesn't have the same effect. This is consistent with candidate 001's own
"why" section guess, now with a concrete two-way split to back it up: mp3 (dense float DSP)
benefits from both wider registers and FMA; flac (integer-heavy, less uniform) benefits from wider
registers alone and is hurt by FMA's scheduling change specifically.

## What's not done / open questions

- Ogg's own result here is inconclusive (noise-level, not a confirmed win) - a longer/paired run
  (e.g. `tools/paired.sh`-style alternating baseline/candidate runs, more repetitions) would be
  needed to say anything confident about ogg specifically under `+avx2`-only; not done this round
  given the time budget was better spent confirming the flac fix.
- Still no runtime-dispatch implementation (same open item as candidate 001) - this remains a
  build-flag-only investigation, not a patch. No `patch.diff` (N/A, no source changed - same as
  candidate 001).
- `+sse4.2` alone (an even narrower flag, older hardware) was not tried - left as a further backlog
  item if broader hardware compatibility than AVX2 is wanted while still avoiding a fully-generic
  baseline.
