# Hand-derived filter kernel formulas in src/math/*.rs

Backlog entry (`IDEAS.md`, added 2026-08-25, source: sibling-project `pic-scale-safe` cross-
reference): that related codebase had several hand-derived resampling-kernel formulas (Gaussian,
Bartlett/Bartlett-Hann, Kaiser, Welch, Jinc, etc.) found wrong by inspection during its own
bug-hunt session, so the same family of files here (`src/math/bartlett.rs`, `gaussian.rs`,
`hann.rs`, `kaiser.rs`, `welch.rs`, `jinc.rs`, `bohman.rs`, `blackman.rs`, `quadric.rs`,
`spline_n.rs`, `lanczos.rs`, `sinc.rs`, `cubic.rs`, `lagrange.rs`, `sphinx.rs`, `bc_spline.rs`) was
worth checking for the same or independently-introduced formula bugs.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read every file in `src/math/` and compare each closed-form formula against its known textbook/reference definition by hand | source at current HEAD (`7eec7cf`) | Found that `bartlett.rs`, `bc_spline.rs`, `blackman.rs`, `gaussian.rs`, `lanczos.rs`, `sphinx.rs`, `spline_n.rs` already carry an extensive `#[cfg(test)]` regression suite whose comments explicitly describe **the exact same bug classes** the sibling project found (e.g. gaussian.rs: "The pre-fix kernel used `exp(-x)` rather than `exp(-x*x)`"; lanczos.rs: "The pre-fix `x == 0` early-out made the kernel jump 1.0 -> 0.0"; spline_n.rs: "outside they used to keep extrapolating"). `git log` on those files shows two prior commits titled "Bugfixes" (`fdc2eca`, 2026-07-28; `2c632f3`, 2026-07-29) that added exactly this coverage, before this bug-hunt session started (today: 2026-08-25). |
| `cargo test --lib math::` (the pre-existing regression suite for the already-fixed files) | debug build, current HEAD | 32/32 (later 44/44 once this candidate's own additions are counted) passed - the specific bug classes named in the backlog entry are confirmed still fixed. |
| Manual boundary-value / independent-reference differential check, by hand-deriving the closed-form reference for the *remaining* files that had **no** existing test coverage (`hann.rs`'s `hann`/`hamming`/`hanning`, `kaiser.rs`, `welch.rs`, `quadric.rs`, `cubic.rs`'s `cubic_spline`/`bicubic_spline`, `lagrange.rs`), temporarily added as `#[cfg(test)] mod tmp_verify_tests` in each file (see `verification_tests.diff` in this folder), then `cargo test --lib tmp_verify` | debug build, current HEAD, reverted after capture | 12/12 passed after two rounds of correcting the *test harness's own* assumptions (see below) - no formula bug found in any of these previously-untested files either. |

### Two apparent failures that turned out to be test-harness mistakes, not source bugs

- `hamming`: initial reference used exact `0.46`/`0.54` f64 constants; the source builds them from
  f32 literals (`0.46f32.as_()`), so they're actually `0.46f32 as f64` / `0.54f32 as f64` once
  widened (same pattern gaussian.rs's own test already documents for its `SIGMA` constant).
  Widening the reference constants and loosening the tolerance to absorb the source's deliberate
  `x == 0 -> exactly 1.0` special case (vs. the f32-rounded formula's `1.0000000298023224`) made
  it match.
- `lagrange2`/`lagrange3`: an exact-polynomial-reconstruction check initially assumed the sample
  window around a phase `p` in `[0, 1)` is symmetric (`k` in `[-support, support)`). The actual
  call convention `weights.rs` always uses (`resampling_function(dx.abs() * filter_scale)`, i.e.
  the kernel is only ever invoked with a **non-negative** offset) combined with the kernel's own
  `n = floor(support + x)` cell-selection logic means the correct window is asymmetric:
  `(support - 1)` samples on one side, `support` on the other (`[-(support-1), support+1)`).
  Once the reconstruction check used that window and `.abs()` on the offset (matching `weights.rs`
  exactly), `lagrange2` reconstructed a degree-3 polynomial and `lagrange3` a degree-5 polynomial
  to within `1e-6` at every tested phase - i.e. the formula is a correct Lagrange-interpolation
  kernel for the way it's actually invoked.

## Status

**Not reproducible.** No hand-derived kernel formula bug was found anywhere in `src/math/*.rs` at
the current HEAD. Coverage: all 15 kernel-formula files were read and checked; 7 of them
(`bartlett`, `bc_spline`, `blackman`, `gaussian`, `lanczos`, `sphinx`, `spline_n`) already had a
dedicated regression suite (44 tests, all passing) put in place by two "Bugfixes" commits
(`fdc2eca`, `2c632f3`) shortly before this session started, explicitly covering the same bug
classes named in this backlog entry; the remaining files with no prior test coverage
(`hann`/`hamming`/`hanning`, `kaiser`, `welch`, `quadric`, `cubic_spline`/`bicubic_spline`,
`lagrange2`/`lagrange3`) were independently differential-tested against hand-derived reference
implementations this session and all matched (12/12). `jinc.rs` and `sinc.rs` are thin wrappers
around `pxfm`'s own (external, separately-maintained) `f_jincpi`/`f_sincpi` and were not
re-derived from first principles - see "What's not done" below.

## Class

N/A (Not reproducible).

## Root cause

Not applicable to the "no bug found" result. For *why the backlog entry's suspicion turned out
already resolved*: the exact bug classes the sibling project `pic-scale-safe` found by inspection
(wrong Gaussian exponent, a discontinuous jinc centre tap, an incorrectly-reparametrized
Bartlett-Hann/Blackman window, cubic splines that kept extrapolating past their support radius)
were already found and fixed in *this* project's own `src/math/` files by the same author, in two
commits dated 2026-07-28/29 - before the 2026-08-25 cross-reference note that seeded this backlog
entry was even written. The backlog entry's premise (a class of bug proven real in a sibling
codebase, worth checking here) was correct in spirit, but the check had already been done and
landed here independently.

For the two false leads (`hamming`, `lagrange`) hit while extending coverage to the previously-
untested files: both were artifacts of an initial mismatch between the *test's* assumptions
(exact f64 constants; a symmetric sample window) and the source's actual, valid design (f32-
widened constants; an asymmetric window driven by `weights.rs` always passing `dx.abs()` combined
with `lagrange`'s `floor(support + x)` cell selection) - not defects in the source.

## The fix

No fix - no bug confirmed. `verification_tests.diff` in this folder is **not** a source fix; it is
the temporary `#[cfg(test)]` coverage added and run this session (then reverted from the real
working tree per the workflow's "always leave the tree clean" rule), kept here as durable evidence
of what was checked and how.

- Project test suite: 78/78 passed both before and after this candidate (no source change was
  made; the temporary test-only additions were applied, verified, and reverted within this
  session - see `output.txt` for the actual captured runs, including the pre-existing `math::`
  suite, the new `tmp_verify` tests, and the full `cargo test --lib` run).

## What's not done / open questions

- `jinc.rs`/`sinc.rs` delegate to the external `pxfm` crate's `f_jincpi`/`f_sincpif`/`f_cospi`/etc.
  - those transcendental implementations were not independently re-derived or fuzzed here; this
    candidate only checked the pic-scale-side formulas built *on top of* them (e.g. `lanczos_jinc`'s
    windowing, already covered by the existing `lanczos.rs` test suite).
  - One latent (currently unreachable through the normal resize pipeline) formula gap noticed
    while writing the differential tests: `welch()` only checks `x >= 1` to clip to zero, with no
    symmetric `x <= -1` branch - calling it directly with e.g. `x = -2.0` returns a nonzero,
    incorrect value instead of `0.0`. This is not reachable via `ResamplingFunction::Welch` today
    because `weights.rs`'s `generate_weights` always calls every kernel with `dx.abs()` first, so
    `welch` (like every other `pub(crate)` kernel in this module) never actually receives a
    negative argument in the real pipeline - not filed as its own candidate since it isn't
    reachable, but noted here and in `IDEAS.md` in case a future refactor of `weights.rs` ever
    calls kernels with a signed offset directly.
  - Similarly, `lagrange`'s own bounds check (`if x > support { return 0 }`) only clips the
    positive side; a directly-supplied `x < -support` is not clipped either, for the same
    "callers always pass `dx.abs()`" reason - same unreachable-today caveat.
  - This candidate did not check the SIMD (`src/avx2`, `src/avx512`, `src/sse`, `src/neon`)
    re-implementations of any of these kernels for a similar hand-derivation mismatch against the
    scalar `src/math/` version - that's a distinct, not-yet-investigated suspect (see `IDEAS.md`).
