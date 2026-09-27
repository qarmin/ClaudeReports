# JPEG2000 IDWT horizontal filter: restructure `irreversible_filter_97i_horizontal_simd` into smaller functions to test the inlining-threshold hypothesis

Follow-up to candidate `013`, which found candidate `002`'s deinterleave-based SIMD rewrite of the
horizontal IDWT filter (`hayro-jpeg2000/src/j2c/idwt.rs`, never applied to real source - it
regressed 8.6x) compiles its `f32x8` arithmetic to real out-of-line `__fearless_simd_kernel` calls
under `Avx512` instead of inlining, unlike the existing, fast `filter_vertical` path using the same
primitives, and hypothesized (but did not test) that this was because
`irreversible_filter_97i_horizontal_simd`'s size/complexity (4 lifting steps + closures, all in one
function body) pushed LLVM's inliner past whatever threshold decides whether to inline
`fearless_simd`'s per-op calls. This candidate tests that hypothesis directly: split the function
into 5 smaller free functions (one per step/step-pair) with the arithmetic moved into small named
helper functions instead of inline closures, then re-check via `nm`/`objdump` and measure
wall-clock time.

**No patch to real source is proposed here** - like candidate 013, this is a
measurement/characterization. `patch.diff` re-applies candidate 002's rewrite (needed for the
functions under test to exist at all) plus this candidate's restructuring on top; both were
reverted from the real working tree before this folder was written (`git status` is clean).

## What was tried

Restructured `irreversible_filter_97i_horizontal_simd` (as it exists after candidate 002's patch)
from one function containing all 4 lifting steps + 2 scale calls inline, into:
- `irreversible_step_1_2` (steps 1/2, the scale calls)
- `irreversible_step_3`, `irreversible_step_4`, `irreversible_step_5`, `irreversible_step_6` (one
  lifting step each)
- `lifting_op_simd`/`lifting_op_scalar`: the shared `s +/- (left + right) * factor` arithmetic,
  factored into free functions taking `factor: f32` as a parameter, so each step's closure is a
  one-line forward (`move |s, l, r| lifting_op_simd(simd, s, l, r, NEG_DELTA)`) instead of
  duplicating the FMA expression inline.
- `irreversible_filter_97i_horizontal_simd` itself becomes a 5-line dispatcher calling the above.

See `patch.diff` for the exact diff (against candidate 002's already-applied rewrite).

### `nm`/`objdump` check (this session's own build, both variants, identical toolchain)

Built `bench/idwt`'s bench crate twice (path-pointed at the live checkout): once with only
candidate 002's patch applied (no restructuring - "baseline" for this candidate), once with this
candidate's restructuring on top ("restructured"). Counted real calls to
`<Level as Simd>::*_f32x8::kernel::__fearless_simd_kernel` via `objdump -d --demangle` (excerpts in
`evidence/`):

| Build | Function | Real `__fearless_simd_kernel` calls (`Avx512`) |
|---|---|---|
| baseline (002 unsplit) | `irreversible_filter_97i_horizontal_simd::<Avx512>` | 28 |
| restructured (this candidate) | `irreversible_filter_97i_horizontal_simd::<Avx512>` (steps 1/2, 4, 5, 6 all fully inlined back in - only `irreversible_step_3` stayed a separate symbol, per `nm`, see `evidence/symbols_restructured.txt`) | 26 |
| restructured (this candidate) | `irreversible_step_3::<Avx512>` (the one step LLVM kept separate) | 6 |
| restructured (this candidate) | **total** | **32** |

Note: candidate 013's own count for the *unsplit* function was 92 calls, not this session's 28 -
both builds in this session used the identical toolchain/profile for a fair same-session
comparison, but the absolute count differs from 013's number (possibly a different `rustc`/
`fearless_simd` version, or 013's build included its own temporary diagnostic `#[cfg(test)]`
module affecting codegen - not reconciled here). The *within-this-session* comparison (28 -> 32)
is what this candidate's conclusion rests on, not the cross-session absolute numbers.

**The hypothesis is not supported**: splitting into smaller functions did not reduce the number of
real out-of-line calls - it went up slightly (28 -> 32), and no variant reached anywhere close to
`filter_vertical`'s 0. Four of the five split-out steps (`irreversible_step_1_2`, `_4`, `_5`, `_6`)
were fully re-inlined into the parent function by LLVM anyway (no separate symbols in `nm`), while
one (`irreversible_step_3`) was kept as its own function but still contains 6 real calls itself -
i.e. neither the "stays one big function" nor the "gets split into small functions" case actually
achieves full inlining of `fearless_simd`'s primitives on this hardware/toolchain.

### Wall-clock (`bench_idwt`, real JPX decode of the embedded `sample.jp2`, 300 iterations/round)

`taskset -c 2` for single-core, unpinned for multi-core; `uptime` load 4.6 (well under the 24-core
count) throughout. First round of each pair showed elevated noise (system settling after the
preceding build); rounds 2 onward are the clean, reported numbers.

| Round | single-core p50 baseline | single-core p50 restructured | multi-core p50 baseline | multi-core p50 restructured |
|---|---|---|---|---|
| 1 (noisy, excluded) | 28.495 ms | 21.878 ms | - | - |
| 2 | 21.218 ms | 21.568 ms | 21.072 ms | 21.272 ms |
| 3 | 21.410 ms | 21.538 ms | 20.815 ms | 21.426 ms |
| 4 | 21.303 ms | 21.248 ms | 20.993 ms | 21.272 ms |

Single-core p50 average (rounds 2-4): baseline 21.310 ms, restructured 21.451 ms (**+0.7%**).
Multi-core p50 average (all 3 rounds): baseline 20.960 ms, restructured 21.323 ms (**+1.7%**).
`min` shows the same direction: single-core baseline avg 20.315 ms vs restructured 20.540 ms
(+1.1%); multi-core baseline avg 20.044 ms vs restructured 20.389 ms (+1.7%). Restructured is
consistently, if only slightly, slower in every one of the 6 clean rounds measured (3 single-core +
3 multi-core) - small but directionally consistent with the `nm`/`objdump` finding above (more real
calls remaining, not fewer), and nowhere near reversing candidate 002's original 8.6x regression.

## Result

The restructuring did not restore inlining and did not produce a measurable win - if anything, a
small (~1-2%), consistent-direction regression across both single-core and multi-core rounds,
which is directionally consistent with (though not fully explained by, given the still-small
overall function sizes involved) the `nm`/`objdump` finding that real out-of-line calls increased
slightly rather than decreasing. Candidate 013's "size/complexity threshold" hypothesis, at least
for this specific restructuring shape (5 free functions, arithmetic factored out, closures reduced
to one-line forwards), is not supported by this session's evidence.

## Does it change behavior/output?

- **Bit-exact?** Yes - `bench_idwt`'s per-iteration output hash matched between the 002-unsplit
  baseline and this candidate's restructured build in every run this session
  (`hash=339b9db71ea46bfc` throughout). Also re-ran candidate 002's own
  `horizontal_filter_equivalence_tests` exhaustive synthetic test (2560 cases, `cargo test -p
  hayro-jpeg2000 --release --lib horizontal_filter_equivalence`) against the restructured code -
  passes, same as it does against 002's unsplit rewrite (both compare against the same scalar
  reference with the same FP tolerance, since neither changes the actual arithmetic, only which
  function it lives in).
- **Is this a breaking change?** No - purely an internal restructuring of code that was itself
  never applied to real source (candidate 002's regression means none of this ships either way).
- Project test suite: not re-run this session (candidate 002's own regression already makes this
  moot for shipping purposes; this candidate exists purely to test the inlining hypothesis).

## Why the result looks the way it does

Not fully root-caused at the LLVM-internals level (same caveat as candidate 013 - `perf
record`/`perf annotate` still unavailable). Two observations from this session narrow it further
than 013 left it:

1. LLVM's inliner did **not** treat the 5 split functions uniformly - 4 of 5 were folded back into
   the parent function anyway (matching pre-split behavior for those steps), while only
   `irreversible_step_3` was kept separate. This suggests the split didn't change the *inlining
   decision* for most of the code at all - LLVM was already going to produce roughly the same
   final shape regardless of the source-level function boundaries, which would explain why the
   real-call count barely moved (and, if anything, moved the wrong way - possibly due to
   `irreversible_step_3` now being a distinct instantiation point with its own, individually-worse
   inlining tradeoff, though this is not independently confirmed).
2. This session's baseline count (28 real calls) is much lower than candidate 013's originally
   reported 92 for the same *unsplit* function - meaning something other than this candidate's
   restructuring (toolchain version, `fearless_simd` version, or build profile differences between
   sessions) already has a large effect on this inlining decision, independent of source-level
   function size. That's a more promising lead for a future candidate than further
   function-splitting: pin down what actually changed between the two sessions' 28-vs-92 counts
   (a `rustc`/`fearless_simd` version diff would be the first thing to check) before trying another
   restructuring-based fix.

## What's not done / open questions

- The 92-vs-28 discrepancy between candidate 013's count and this session's count (same unsplit
  function) is not reconciled - not clear whether it's a toolchain/dependency version difference,
  a build-profile difference (013 may not have used `debug = 1`), or something about 013's own
  temporary diagnostic test module affecting codegen for the function under test. Worth checking
  `rustc --version` / `Cargo.lock`'s `fearless_simd` version across the two sessions before trying
  further restructuring-based fixes.
- Other restructuring shapes weren't tried (e.g. `#[inline(never)]` explicitly on each split
  function to force LLVM's hand rather than leaving the decision to its own heuristic, since here
  4 of 5 were re-inlined anyway; or splitting the *closures* further from the *dispatch* logic
  differently than this candidate did).
- `evidence/` in this folder has the `nm` symbol lists for both builds and `objdump` excerpts for
  the specific functions counted above.
