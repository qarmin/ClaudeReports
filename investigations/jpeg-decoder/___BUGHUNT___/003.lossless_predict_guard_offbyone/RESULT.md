# lossless_predict_guard_offbyone

`src/decoder/lossless.rs`'s `predict()` function (~line 199-206), the H.1.2.1 default-prediction
branch used at the start of the first scan line and at the first sample after every restart
marker. Flagged because its underflow guard (`if input_precision > 1 + point_transform { ... }
else { 0 }`, added by commit 3179243 to stop a `u8` subtraction underflow) is stricter than the
invariant the parser now enforces (`point_transform < frame.precision`, added independently by
commit 95550d7 / #282), leaving one legal boundary value (`point_transform == precision - 1`)
mishandled.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading of `predict()` and `src/parser.rs::parse_sos`'s point-transform validation | current HEAD (eb2d7c0), plain reading, no build | Confirmed the exact code shape described: `predict()`'s guard is `input_precision > 1 + point_transform`; `parse_sos` rejects `point_transform >= frame.precision` (line 408), so by the time `predict()` runs, `input_precision - point_transform >= 1` always holds and the guard's original underflow concern is moot - but the guard's own threshold is one notch stricter than that invariant, so `input_precision == point_transform + 1` (e.g. precision=8, Pt=7) takes the `else` branch and returns 0 instead of the spec-correct `1 << 0 == 1`. |
| Manual boundary-value construction: a standalone synthetic 1x1-pixel lossless JPEG (SOF3, precision=8, predictor selection 0/NoPrediction to exercise the generic non-`Ra` branch, point_transform=7, one DHT symbol encoding DC difference 0) fed through the real `jpeg_decoder::Decoder::decode()` public API, release and debug builds | 4/4 runs (3x release, 1x debug), bit-for-bit identical output every time - fully deterministic, no layout/seed dependence | Decoded pixel = 0; ground truth (H.1.2.1: default prediction `1 << (precision-Pt-1)` = 1, then `<< Pt` = 128) = 128. Reproduces exactly as predicted from reading the code. |
| A standalone check sweeping every valid `(precision, point_transform)` pair with `0 <= point_transform < precision`, `precision` in 2..=16, comparing the old guard formula, the fixed guard formula, and the spec-correct value | plain `rustc -O`, no jpeg-decoder dependency, <1s | Old guard is wrong exactly at `point_transform == precision - 1` for every precision in range (15 mismatches, one per precision level 2..=16); fixed guard (`input_precision > point_transform`) matches the spec-correct value for every single pair in the swept range, confirming the fix's threshold change doesn't introduce any new mismatch. |

No sanitizer/miri/fuzzing was needed or attempted: this is a pure logic bug (wrong output on a
specific legal boundary value, not a crash/UB/overflow), and reading the two independently-added
guards side by side already pinpointed the exact off-by-one; a direct boundary-value repro against
the real public decode API gave a full, deterministic, repeatable confirmation on the first
attempt, and the boundary sweep above (HEURISTICS.md's "reasoning from a documented invariant and
constructing an input that should violate it") gave a second, independent signal ruling out any
other off-by-one direction.

## Status

**Confirmed.** Reproduces 4/4 runs (3x `cargo run --release`, 1x plain `cargo run`) against
base_commit eb2d7c0, byte-for-byte identical wrong output (decoded pixel 0 instead of 128) every
run - fully deterministic, not a layout/seed-dependent finding.

**Fully reproducible:** Yes. The repro feeds a hand-built but fully spec-valid lossless JPEG byte
stream (a single 1x1-pixel image, precision=8, point_transform=7, one DC Huffman symbol) through
`jpeg_decoder::Decoder`'s real, public `decode()` entry point - the actual affected code path end
to end. This is a single-participant case (`CONFIRMATION.md` section 11's "no second participant"
exception applies directly, same as candidates 001/002 in this workspace): the input is a single
external artifact (a JPEG file), the same kind of bytes a real caller/attacker would hand to
`Decoder::new(...).decode()`; there is no peer/caller whose reaction is being faked. The only
hand-authored part is the input file's bytes themselves, which is what a real fuzzer or a real
malformed/legal-edge-case file on disk would also just be, not a stand-in for another program's
behavior.

## Class

logic

## Root cause

`predict()`'s default-prediction branch (used at `(ix==0, iy==0)` - the very first pixel of the
scan - or at `restart` - the first sample right after any `RSTn` marker) computes, per ITU-T T.81
H.1.2.1, the fixed value `1 << (precision - Pt - 1)`. Because that subtraction is done in `u8`
arithmetic, commit 3179243 (2022-09-02, "Mitigate panic due to arithmetic overflow in lossless
module", fixing #250) wrapped it in a guard, `if input_precision > 1 + point_transform { ... }
else { 0 }`, to stop it from underflowing when `point_transform >= precision` (which was legal
input at the time - the parser didn't yet reject it).

Commit 95550d7 (2025-03-07, #282, "Fix panic when prediction calculation is invalid") later closed
that hole from the other end, independently, in `src/parser.rs::parse_sos`: it now hard-rejects
any scan with `point_transform >= frame.precision` before a `ScanInfo` is ever produced. From that
commit onward, every `predict()` call is guaranteed `input_precision - point_transform >= 1`, so
the underflow 3179243's guard was protecting against can no longer occur - but nobody revisited
the guard's own threshold when the invariant tightened. `input_precision > 1 + point_transform` is
equivalent to `input_precision - point_transform > 1`, i.e. `input_precision - point_transform >=
2` - one notch stricter than the now-guaranteed `>= 1`. The single legal value in between,
`input_precision - point_transform == 1` (`point_transform == precision - 1`, the parser's own
newly-legal boundary), takes the `else` branch and returns 0 instead of the correct `1 << 0 == 1`.

This silently mis-predicts the top-left pixel of the image and the first pixel of every MCU
following a restart marker for any lossless JPEG using `point_transform == precision - 1` on any
predictor selection except `Ra` (predictor selection 1 has its own separate, unconditional
top-left formula at line 111-112 - `1 << (frame.precision - scan.point_transform - 1) as i32` -
with no guard at all, so it already computes the correct value and was not affected). Every pixel
computed via that (correct-but-wrong-in-the-else-case) default prediction has a decoded value that
is silently off from the spec-correct one by a fixed offset - not a crash, a quiet data-corruption
bug.

## Introduced by

31792433d31ab654823503e0161c13108d0ab830 "Mitigate panic due to arithmetic overflow in lossless
module" (2022-09-02, fixes #250) - added the guard with the wrong threshold. The guard became
provably always-safe-to-relax only once commit 95550d7eec747b94b5f9beba3be2c6fcb1f4f70e (2025-03-07,
#282) independently tightened `parse_sos`'s own validation to `point_transform < frame.precision`;
neither commit's author revisited the other's code, which is how the off-by-one survived. Confirmed
via `git log -S "1 + point_transform" -- src/decoder/lossless.rs` (single match: 3179243) and
`git show 95550d7 -- src/parser.rs`.

## The fix

Tighten `predict()`'s guard from `input_precision > 1 + point_transform` to
`input_precision > point_transform` - the minimal change that exactly matches the invariant
`parse_sos` now enforces (`point_transform < frame.precision`, i.e. `precision > point_transform`),
with no other change to the function. See `patch.diff` (+1/-1 line, `src/decoder/lossless.rs` only).
The `else` branch (`0`) is now unreachable given `parse_sos`'s current validation, but is left in
place defensively rather than replaced with a panic/assert, since `predict()` has no way to itself
verify the invariant its one caller relies on.

Verified:
- Re-ran the standalone repro (precision=8, point_transform=7, 1x1 image) against the patched tree:
  decoded pixel is 128 (previously 0) - matches the spec-correct ground truth exactly.
- Re-ran the standalone sweep over every valid `(precision, point_transform)` pair (0 <=
  point_transform < precision, precision 2..=16) against both the old and new guard formulas: the
  new formula matches the spec-correct value for every single pair, including every one the old
  formula already got right (i.e. the fix changes behavior only at the one previously-wrong
  boundary, not anywhere else) - per `CONFIRMATION.md` section 7, this isn't a fix narrowly
  tailored to the one exact input that first triggered it.
- Project test suite (`cargo test --release`, toolchain override to 1.86.0 in this worktree only,
  purely to dodge an unrelated `unicode-ident`/MSRV lockfile mismatch against the repo's pinned
  1.61.0 toolchain that blocks `cargo test` entirely regardless of this patch - the pinned 1.61.0
  toolchain itself was still used for every build/run of the actual repro above): **23 passed, 0
  failed, 0 ignored** before the patch, and **23 passed, 0 failed, 0 ignored** after - identical
  counts, no regressions, no new failures. See `output.txt` for the full before/after breakdown.

## What's not done / open questions

- The same "guard threshold not revisited after a caller-side invariant tightened" bug class was
  checked only at this one call site - `predict()` has exactly one caller
  (`decode_scan_lossless`'s generic/non-`Ra` branch), confirmed by `grep -rn "predict("
  src/decoder/`, so there is no sibling call site to separately check.
- `tests/reftest/images/lossless/` has no test image using `point_transform == precision - 1` on
  any predictor selection - a maintainer taking this patch may want to add one as a regression
  test; this candidate's `repro/` demonstrates the bug and the fix but is not itself wired into the
  project's own test suite (per this skill's own scoping, this workspace's job is confirm+patch,
  not adding project-side regression tests).
- No broader fuzzing campaign was run against the lossless decode path in general (out of scope for
  this candidate - see `fuzz-bug-repro` for that, invoked separately; not needed here since manual
  boundary-value construction already gave a full deterministic confirmation on the first attempt,
  per `HEURISTICS.md`'s guidance for arithmetic-on-untrusted-values bug classes).
- This candidate's investigation independently confirms this workspace's existing candidates
  001/002 (both also in `decode_scan_lossless`, both from the same 770a5a2 restructuring commit,
  unrelated root causes) are not the same bug as this one: those affect restart-boundary tracking
  and the `Ra` fast path respectively, while this one is a pure guard-threshold off-by-one in the
  shared `predict()` helper the `Ra` fast path deliberately doesn't call.
