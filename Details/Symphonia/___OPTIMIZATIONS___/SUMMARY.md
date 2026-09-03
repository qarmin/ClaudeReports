# Optimization results

**Impact** (derived from the Result column, for scanning at a glance - not a recommendation, this
classifies the measured magnitude/direction only, it doesn't decide what to do about it): **Strong
win** (large, confirmed improvement) - **Small win** (real but modest improvement) - **No change**
(null result / within noise) - **Mixed** (improves one thing, regresses another) - **Small/Strong
regression** (would make things worse) - **N/A** (pure measurement/characterization, no patch
compared against a baseline).

Sort by impact (biggest real win first; noise/regressions at the bottom). A candidate that
combines several individually-tested fixes only appears here if it was explicitly requested and
measured as its own row (this skill tests one fix at a time by default - see `SKILL.md`).

**Candidate** and **Result** are two different lanes, not two phrasings of the same thing.
**Candidate**: the finding itself, in plain language - what's slow, or what the patch changes, and
why - written so a reader understands the problem with zero numbers. **Result**: the technical,
measured evidence - the actual numbers (timings, allocation counts, RSS, paired-run counts), what
was compared against what, and any measurement caveats.

**Fixed by**: blank/empty if the underlying inefficiency is still present in current `HEAD` - or
the short commit hash + subject of the commit that already addressed it, if a maintainer
independently applied the same (or an equivalent) fix upstream since this candidate was measured.

| # | Impact | Candidate | Result | Changes behavior? | Fixed by | Link |
|---|---|---|---|---|---|---|
| 001 | N/A (measurement only, no source patch) | Release builds target generic x86_64/SSE2 even though MP3/Vorbis decode's dense DSP kernels (32-point DCT, 18-point IMDCT, polyphase synthesis filterbank) are FMA/wide-SIMD-friendly | mp3 ~-16-17% wall time, ogg ~-5-6%, flac ~+2.5% regression, wav flat, with `RUSTFLAGS="-C target-feature=+avx2,+fma"` vs default build - repeated 2x per format, bit-exact hash both sides | no, bit-exact; not a breaking change to source/API, but a shipped AVX2+FMA-required binary would SIGILL on pre-Haswell/pre-Excavator CPUs | | [`001.mp3_target_feature_avx2_fma/RESULT.md`](001.mp3_target_feature_avx2_fma/RESULT.md) |
| 002 | N/A (measurement only, no source patch) | Follow-up to 001: does dropping `+fma` (keeping `+avx2`) avoid FLAC's regression while keeping mp3/ogg gains | mp3 ~-15-16%, flac ~flat/within noise (no longer regresses), ogg ~-6% to flat depending on run - `RUSTFLAGS="-C target-feature=+avx2"` (no fma) vs default build, bit-exact hash all formats | no, bit-exact; same build-flag-only caveat as 001 (AVX2-capable CPU required, ~Haswell/2013+) | | [`002.avx2_only_no_fma_flac_regression_fix/RESULT.md`](002.avx2_only_no_fma_flac_regression_fix/RESULT.md) |
| 003 | Small win | `copy_to_slice_interleaved` (shared by every codec's decode-to-interleaved-buffer path) uses a runtime-unknown interleave stride even for the common 2-channel (stereo) case, blocking vectorization of the interleaved write | wav ~-4-5% (4 paired rounds), flac ~-1-2%, ogg ~-1.5-3.5%, mp3 small win/flat (noise-level) - `perf annotate` share of WAV profile dropped 7.09%->3.33%; bit-exact hash for all formats plus synthetic mono/5-channel files (unaffected fallback path) | no, bit-exact; not a breaking change (internal fast path, same public API/semantics) | | [`003.stereo_copy_to_slice_interleaved_fast_path/RESULT.md`](003.stereo_copy_to_slice_interleaved_fast_path/RESULT.md) |
| 004 | Small win | `BitReaderLtr`/`BitReaderRtl::fetch_bits` (the bit-cache refill path, called once per Rice-coded residual sample in FLAC's hottest loop, and even more densely in ALAC's residual-reading loop) was missing `#[inline]` even though every sibling method on the same trait impl has one - a real un-inlined cross-crate call inside a hot per-sample loop | alac ~2.6-3.2% real win (paired 4/4, largest win in this workspace so far); flac ~0.5-1.5% real win (paired 8/8 across two sessions) - perf annotate showed the un-inlined `callq` in both profiles before the fix; ogg/wav/mp3 within noise (mp3 barely touches this call site) - bit-exact hash all 5 formats, full workspace test suite 0 failures before/after | no, bit-exact; not a breaking change (pure inline hint, no API/behavior change) | | [`004.bitreader_fetch_bits_missing_inline/RESULT.md`](004.bitreader_fetch_bits_missing_inline/RESULT.md) |
