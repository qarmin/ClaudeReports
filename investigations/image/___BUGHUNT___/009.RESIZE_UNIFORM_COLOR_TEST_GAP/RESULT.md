# `image::imageops::resize`'s test suite has no uniform-color-preservation regression test

Backlog item from `IDEAS.md` (top row, sourced from candidate 003): candidate 003's confirmed
+1.0 accumulator-seed bias in `vertical_sample` passed the crate's own `cargo test --lib` suite
(331/331, 0 failed) on the buggy pristine tree - no existing test asserts that resizing a
uniform-color image preserves that color, so this exact bug class (a wrong constant folded into
every resized pixel) has no regression test guarding it. This candidate checks whether that gap is
real and closes it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Reran the crate's existing suite on the current (still-buggy) `main` working tree, in-process | `cargo test --lib`, debug build, no changes | 331 passed, 0 failed - confirms the gap is still live on current `main` (`44ce9226`), not just historically true as of candidate 003. |
| Added a candidate regression test (`test_resize_uniform_color_preserved`, resizes a uniform 6x6 `Rgb<u8>` image to 3x3 under 5 filter types and asserts every output pixel equals the source color) directly to `src/imageops/sample.rs`'s existing `mod tests`, ran it against the unmodified (buggy) source, in-process | `cargo test --lib test_resize_uniform_color_preserved`, debug build, 3 repeat runs | Deterministic failure, 3/3 runs: `left: Rgb([101, 101, 101])`, `right: Rgb([100, 100, 100])` for `FilterType::Nearest` (first filter checked) - the new test genuinely catches the known bug class. |
| Applied candidate 003's already-verified one-line accumulator fix (`Rgba([1.0; 4])` -> `Rgba([0.0; 4])`) on top of the new test, in-process | `cargo test --lib`, debug build, 3 repeat runs | 332 passed, 0 failed, 3/3 runs - the new test passes once the underlying bug is fixed, and no other test regresses. |
| Standalone `repro/` (own `Cargo.toml`, real public API `image::imageops::resize`, mirrors the added test's exact check) run via `just test` end-to-end against a pristine local clone pinned at `base_commit`, plus the clone's own `cargo test --lib` run before/after `patch.diff` | `just test` (full cycle: clone -> reproduce-before -> test-only-gap -> apply -> reproduce-after), release build | Matches the in-tree runs exactly: pristine clone -> repro prints `GAP DEMONSTRATED` and `cargo test --lib` still reports 331/331 green; test-only patch (no fix) -> new test fails 100,100,100 vs 101,101,101; full `patch.diff` (test + fix) applied -> repro prints `Gap closed` and `cargo test --lib` reports 332/332 green. See `output.txt`. |

This is a manual boundary-value/differential-invariant check (per `HEURISTICS.md`'s "Logic bugs"
guidance - no sanitizer/miri/fuzzing needed to confirm a missing assertion), not a source-code bug
hunt. Soft cap of 3 was not needed; the gap was evident from the first rerun of the existing suite.

## Status

**Confirmed** (as a test-suite gap, not a source bug - see the candidate's title and `IDEAS.md`'s
own framing). Deterministic: the gap (existing suite passes 331/331 despite the known bug) and the
fix (new test + candidate 003's accumulator fix together produce 332/332) both reproduce 3/3 direct
in-tree runs, plus a further 3/3 through the standalone `repro/`'s `just test` cycle (clone's own
`cargo test --lib`, matching in-tree results exactly) - 6/6 total for each side, byte-identical
test-result summaries every time.

**Fully reproducible: Yes.** `repro/`'s `main.rs` and the added `src/imageops/sample.rs` test both
call only `image::imageops::resize`, the crate's real public API, on a real `ImageBuffer` built
through its own public constructor - the same real entry point a genuine caller (or the crate's own
CI) exercises. There is no second participant/peer/wire input in this finding (a single-process
question of whether the test suite covers an invariant), so `CONFIRMATION.md` section 11's "real
second participant" requirement does not apply.

## Class

test-coverage

## Root cause

`src/imageops/sample.rs`'s existing `mod tests` (as of `base_commit`) checks `resize`'s behavior
only for: same-size no-op copy (`test_resize_same_size`), `sample_bilinear`/`sample_nearest`
bounds/`Option` behavior, per-pixel bilinear/nearest interpolation correctness against a small
hand-built 2x2/3x3 buffer (`test_sample_bilinear_correctness`, `test_sample_nearest_correctness`),
and a historical regression (`test_issue_186`). None of these assert an output value against the
documented invariant that `vertical_sample`/`horizontal_sample`'s filter weights are normalized to
sum to 1.0 (so a uniform-color source must resize to the exact same uniform color, for any filter).
Candidate 003's bug is a flat `+1.0`-per-channel accumulator-seed error that violates exactly that
invariant on every non-trivial resize - since nothing in the suite exercises that specific
invariant, the bug shipped past `cargo test --lib` undetected (331/331 unchanged before and after
candidate 003's fix, as recorded in that candidate's own `RESULT.md`).

## Introduced by

Not determined - this candidate's "root cause" is an absence (no test exercises the
normalized-weights invariant), not a buggy line with a single introducing commit. The source bug
this gap fails to catch is candidate 003's, introduced by 20a2308 "Remove Pixel channels4/from_channels" (2025-12-30).

## The fix

Added `test_resize_uniform_color_preserved` to `src/imageops/sample.rs`'s `mod tests`: builds a
uniform 6x6 `Rgb<u8>` source via `ImageBuffer::from_pixel`, resizes it to 3x3 under `Nearest`,
`Triangle`, `CatmullRom`, `Gaussian`, and `Lanczos3`, and asserts every output pixel equals the
source color exactly (no tolerance needed - a truly uniform image's weighted average, with
correctly-normalized weights, is exact for `u8` regardless of filter, so this is not a flaky
approximate check).

`patch.diff` bundles this new test together with candidate 003's already-independently-verified
one-line accumulator fix (`Rgba([1.0; 4])` -> `Rgba([0.0; 4])`), per `IDEAS.md`'s own suggestion
("worth adding one alongside candidate 003's fix, or independently") - applying `patch.diff` alone
to a pristine `base_commit` checkout produces a fully green 332-test suite. `test_only.diff` (also
in this folder) contains just the new test, without the accumulator fix, and is used by
`justfile`'s `test-only-gap` recipe purely as evidence that the added test is genuinely sensitive
to this bug class (it fails deterministically, 3/3, against the unmodified buggy source) - it is
not itself proposed as a shippable patch on its own, since shipping only a permanently-failing test
would leave `cargo test --lib` red.

- Project test suite (`cargo test --lib`, run in the main project's own working tree):
  - Before any change: **331 passed, 0 failed**.
  - With only the new test added (no accumulator fix): **331 passed, 1 failed**
    (`imageops::sample::tests::test_resize_uniform_color_preserved`) - reproduces 3/3 runs.
  - With `patch.diff` (new test + candidate 003's accumulator fix) applied: **332 passed, 0
    failed** - reproduces 3/3 runs, no other test regressed.

## What's not done / open questions

- This candidate deliberately re-bundles candidate 003's accumulator fix into `patch.diff` so the
  suite is green after applying it; candidate 003's own folder already records that exact fix
  independently (`003.VERTICAL_SAMPLE_PLUS_ONE_BIAS/patch.diff`) - the two are the same one-line
  source change, not two separate proposed fixes to the same line. A maintainer applying both
  candidates' patches together should apply only one copy of that hunk (they're identical).
- Not checked this session: whether any other resize-adjacent public API (`DynamicImage::resize`/
  `resize_exact`/`resize_to_fill`, `imageops::thumbnail`) has the same style of test-coverage gap
  for its own accumulator/averaging math - candidate 005's `RESULT.md` already establishes those
  use separate, independently-checked code paths that don't share `vertical_sample`'s bug, but
  their own test coverage for a "uniform input stays uniform" invariant wasn't audited here.
- The new test only checks `Rgb<u8>` uniform-color preservation; it does not add coverage for the
  `f32`/`Rgb32FImage` case that candidate 003's `RESULT.md` notes is affected *unconditionally and
  unclamped* by the same bug class (no `u8` saturation to mask it) - a future session could extend
  the test (or add a sibling one) for that pixel type if broader coverage is wanted.
