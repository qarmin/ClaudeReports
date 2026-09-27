# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results,
mixed results, and regressions) and the full methodology behind each number. **All numbers below
were ported from `image` repo's `___OPTIMIZATIONS___/` workspace and not re-measured against this
repo directly** - see `SUMMARY.md`'s own header and each candidate's `RESULT.md`.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 007 | Strong win | Vertical convolution tap loop widened to AVX2 via runtime multiversioning (changes `forbid(unsafe_code)` to `deny`) | +224/-8 | -27.0%/-26.9% single/multi-core on a large Lanczos3 resize |
| 008 | Strong win | Extends 007's AVX2 multiversioning to the horizontal + floating-point convolution paths | +657/-14 | -40.0%/-39.9% same scenario, up from 007's -27% |
| 001 | Strong win (vs. today's ungated rayon) | `resize_nearest` gains a `dst_height >= 32` gate before dispatching to rayon | +22/-12 | -91.8%/-95.9% single/multi-core vs. today's ungated rayon dispatch |
| 009 | Small win | Adds an AVX-512 dispatch tier on top of 007/008 for the vertical tap loop | +786/-14 | -2.2% to -3.5% further on top of the AVX2 win |
| 010 | Small win | Extends 009's AVX-512 tier to the horizontal/floating-point paths | +1045/-14 | -1.4% to -3.6% on the newly-tiered functions |
| 004 | Small win | Adds a low row-count floor to candidate 003's per-filter thread pool | +385/-151 | -6.5% to -18.8% at genuinely small output sizes, unchanged at the workspace's established scenario |
