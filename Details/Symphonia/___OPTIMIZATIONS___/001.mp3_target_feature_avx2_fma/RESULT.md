# mp3/flac decode: build with `-C target-feature=+avx2,+fma` (or `target-cpu=native`)

Symphonia's default release build targets the generic `x86_64` baseline (SSE2 only) even though
the MP3 (`synthesis`, `imdct36`, `dct32`) and FLAC decode paths are dense scalar/SIMD-friendly
floating-point DSP kernels (32-point DCT, 18-point IMDCT, polyphase synthesis filterbank, FLAC
prediction). Profiling (`perf record -F 999 --call-graph dwarf`) showed the MP3 decode hot path
almost entirely inside `synthesis::synthesis` (35%), `imdct36` (17%), and `requantize::read_huffman_samples`
(16%) - i.e. the DSP core, not I/O or bitstream parsing. The disassembly of `synthesis()` at
baseline already uses `xmm`/`movups`/`mulps`/`addps` (128-bit SSE), so this isn't a
"missing SIMD entirely" case - it's "SIMD present but limited to 128-bit/no-FMA", the same shape
of finding as HEURISTICS.md's "scalar code where adjacent code already vectorizes" entry, just one
level up (SSE2-width vs. AVX2-width + fused multiply-add).

Nothing in the crate depends on scalar fallback logic guarded by `#[cfg(not(target_feature = ...))]`
- there is no explicit SIMD dispatch in `symphonia-bundle-mp3`/`symphonia-bundle-flac`/`symphonia-core`
at all (checked via grep for `target_feature`/`is_x86_feature_detected`), so this candidate measures
what the *existing* auto-vectorizer output does with a wider feature baseline, not a new SIMD
implementation.

## What was tried

Built `symphonia/examples/bench_decode` (this workspace's own harness, see `README.md`) with
different `RUSTFLAGS`, keeping everything else (source, Cargo.toml, profile) unchanged - this is a
build-flag-only change, no source touched.

Single-core (`taskset -c 2`), 20 iterations, two independent runs each (noise floor check),
p50 reported (min in parentheses):

| format | baseline (SSE2 default) | `-C target-feature=+avx2,+fma` | delta |
|---|---|---|---|
| mp3  | run1 38.55ms (38.21) / run2 38.92ms (38.64) | run1 32.45ms (31.98) / run2 32.80ms (32.44) | **~-16% to -17%** |
| flac | run1 21.28ms (21.09) / run2 21.27ms (21.12) | run1 21.82ms (21.74) / run2 21.81ms (21.72) | **+2.5% regression** |
| ogg  | run1 28.40ms (28.26) / run2 28.13ms (28.04) | run1 26.86ms (26.55) / run2 26.72ms (26.45) | ~-5.5% |
| wav  | (from earlier session runs) ~13.3ms | run1 13.36ms (13.21) / run2 13.27ms (13.17) | ~flat (noise-level) |

`-C target-cpu=native` (broader than just avx2/fma - includes whatever this specific CPU exposes,
e.g. AVX-512) was also tried as a first probe and gave similar mp3 numbers (~33.6-33.8ms p50) -
`+avx2,+fma` alone captures essentially all of the win without requiring AVX-512, so it's the
narrower, more portable flag combination reported as the main candidate.

## Result

MP3 decode: a real, repeatable ~16-17% wall-clock improvement (min and p50 both move together
across two independent runs each side, well outside the ~1ms run-to-run jitter visible within
either group) from a build-flag change alone, no source edit. Ogg/Vorbis: a smaller but still
real ~5-6% improvement (same repeatability check). FLAC: a small (~2.5%), consistent *regression*
across both repeat runs - not noise, but small enough that it doesn't offset the mp3/ogg gains if
this were applied crate-wide via `.cargo/config.toml`. WAV/PCM: flat, as expected (no DSP kernel in
that path). Only wall-clock time was measured this round; no allocation/RSS counters were relevant
here since this is a pure recompilation-target change, not a code-shape change.

## Does it change behavior/output?

- **Bit-exact?** yes - identical `DefaultHasher` hash over every decoded f32 sample for mp3, flac,
  ogg, and wav, compared against the baseline-build hash, for every iteration of every run above.
- **Is this a breaking change?** No source or public API changes at all - this is a `RUSTFLAGS`/
  `.cargo/config.toml` build setting. It does, however, produce a binary that will `SIGILL` on a
  CPU without AVX2+FMA (Intel pre-Haswell/AMD pre-Excavator, ~2013 and older, still realistically
  present) if applied unconditionally to a *shipped* binary - this is a real deployment
  consideration, not a code correctness one. It's safe for anyone building Symphonia themselves for
  their own known-modern hardware (a very common case for an audio decoder embedded in a desktop
  app), and is exactly what many end-users already do via `.cargo/config.toml` or
  `RUSTFLAGS`/`CARGO_BUILD_RUSTFLAGS` for their own builds - just not safe as the crate's own
  unconditional default.
- Project test suite: 106 passed / 0 failed both before and after (identical counts, same as the
  workspace's baseline - see `README.md`).

## Why the result looks the way it does

MP3's `synthesis`/`imdct36`/`dct32` kernels are exactly the dense multiply-accumulate loops that
benefit from FMA (fewer instructions, less rounding) and wider SIMD registers (AVX2 = 8×f32 per op
vs. SSE2's 4×f32) - `perf annotate` on `synthesis()` at baseline shows a long, straight-line chain
of `mulps`/`addps` pairs (see the annotated disassembly captured in `annotate_mp3_synthesis_baseline.txt`
in this folder) that is a textbook FMA-fusion + register-width win once the compiler is *allowed*
to target it; nothing in the source needs to change for LLVM's auto-vectorizer to use it, only the
target-feature baseline. FLAC's small regression is very plausibly a case of the auto-vectorizer's
own heuristics choosing a wider-but-differently-scheduled code shape for FLAC's smaller/less
uniform prediction kernels that doesn't pay off the same way (a known,
already-flagged-as-a-tradeoff way this class of build-flag change interacts with LLVM's own
heuristics, not a project-specific bug) - not chased further via a full disassembly diff this round,
since the point at this stage is establishing whether the crate-wide question ("is target-cpu
worth it") is answerable, not micro-explaining every per-codec delta.

## What's not done / open questions

- Not applied anywhere permanently - this candidate is a measurement, not a patch. There is
  nothing to `patch.diff`: no source file changed. (Per SKILL.md's "patch.diff is mandatory..."
  section, this is explicitly the N/A case - a build-flag-only investigation, not a change to the
  tested project's own source.)
- A maintainer wanting this permanently has two real options, neither exercised here: (a) document
  it as a recommended `RUSTFLAGS`/`.cargo/config.toml` setting for people building from source for
  their own hardware (safe, no compatibility risk to anyone else); (b) ship a runtime-dispatched
  SIMD path (`is_x86_feature_detected!("avx2")` + a `#[target_feature(enable = "avx2,fma")]`
  function, falling back to the current scalar/SSE2 path) for the hottest kernels specifically
  (`synthesis`, `imdct36`, `dct32`) - meaningfully more implementation work, not attempted this
  round, but the natural next step if the ~16% mp3 win is worth chasing without an AVX2 hardware
  requirement on every user.
- The FLAC regression wasn't investigated further (which specific FLAC kernel regressed, whether
  a narrower flag - `+avx2` without `+fma`, or `+sse4.2` alone - avoids it while still helping
  mp3/ogg). Left as a backlog item.
- AAC/ALAC/Opus/WavPack/ADPCM decode paths were not benchmarked this round (no test files
  generated for them) - plausible candidates for the same investigation, given they're also
  DSP-heavy (esp. AAC's MDCT, similar in shape to MP3's IMDCT).
