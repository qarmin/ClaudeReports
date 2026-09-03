# lossless_precision_point_transform_subtract_overflow

`src/decoder/lossless.rs:112` (`Decoder::decode_scan_lossless`'s `Predictor::Ra` top-left-pixel
branch) computes `1 << (frame.precision - scan.point_transform - 1) as i32` with unchecked `u8`
subtraction. GitHub issue #277 (open, filed against jpeg-decoder 0.3.1) reports a live
`attempt to subtract with overflow` panic at exactly this line/column on a crafted JPEG passed to
`Decoder::decode`. Looked suspicious because it's the sibling expression to candidate 003's
`predict()`-branch bug, and because #132 (2020, open) reported the same *class* of panic message,
suggesting a recurring pattern across refactors.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro with the actual issue #277 PoC attachment | Downloaded `jpeg-decode-crash.xx.zip` from the issue's own GitHub attachment URL, unzipped to `jpeg-decode-crash.xx` (508 bytes, sha256 `21ea1ac226d65e2e96aa5473063a9095b3ec4044bb43ef2f943c8c1165f0527f`). Ran it through the real public `jpeg_decoder::Decoder::decode()` API two ways: (a) `jpeg-decoder = "=0.3.1"` pulled straight from crates.io (the version the issue was filed against), dev profile (debug-assertions on by default); (b) the project's own `tests/crashtest` harness plus a small `examples/decode277.rs`, both run against a worktree of current HEAD (`eb2d7c0`), dev profile. | (a) panics, 3/3 runs, identical `attempt to subtract with overflow` at `lossless.rs:112:40`, matching the issue's own backtrace exactly. (b) does **not** panic, 3/3 runs, identical `Err(Format("invalid point transform, must be less than the frame precision"))` every time - the same input that crashes 0.3.1 is cleanly rejected on current HEAD. |
| Commit-range inspection (`git log -S`/diff between the panicking and non-panicking versions) | `git log --oneline c1a1fe0..95550d7 -- src/` (0.3.1's release commit .. the commit right before 0.3.2's release commit) | Exactly one `src/` commit in that range: 95550d7 "Fix panic when prediction calculation is invalid (#282)", which adds `if point_transform >= frame.precision { return Err(...) }` to `parse_sos` in `src/parser.rs`. Combined with the pre-existing SOF-time guard in `src/decoder.rs` (`if !(2..=16).contains(&frame.precision) { return Err(...) }`), this algebraically guarantees `frame.precision - scan.point_transform - 1 >= 0` in `u8` arithmetic by the time line 112 runs - see reasoning in `output.txt`. |
| Exhaustive boundary-value sweep against current HEAD, debug-assertions/overflow-checks explicitly on | Hand-built minimal SOF3 (lossless) JPEGs (reusing the byte-level builder pattern from candidate 002's repro) sweeping ALL `u8` precision values (0..=255) x all 4-bit `point_transform` values (0..=15) x all 7 lossless predictor selections, 28,672 total, each run through the real `Decoder::decode()` under `catch_unwind`. `overflow-checks = true` / `debug-assertions = true` set explicitly even in the `release` profile used for speed. | `oks=945 errs=27727 panics=0` - zero panics across the full parameter space; every combination either decodes successfully (precision in [2,16], point_transform < precision) or is cleanly rejected with `Error::Format`/`Error::Unsupported`. |

## Status

**Not reproducible** (on current HEAD, `eb2d7c0`). It reproduces reliably (3/3, deterministic) on
jpeg-decoder 0.3.1 (the version issue #277 was filed against) but was already fixed by commit
95550d7 ("Fix panic when prediction calculation is invalid (#282)"), which shipped in the 0.3.2
release (current HEAD). The exhaustive boundary-value sweep found no residual gap in the fix
across the full precision x point_transform x predictor-selection space.

Issue #132 (2020, also open, also "attempt to subtract with overflow") is a **different bug**:
its own attached backtrace points at `decoder.rs:760` inside `refine_non_zeroes` /
`decode_block_successive_approximation` (progressive DCT successive-approximation refinement),
not `lossless.rs`. Same panic *message* by coincidence of both being unchecked `u8`/integer
subtraction, unrelated code path. Not investigated further as part of this candidate; left as a
separate potential lead (see "What's not done" below) rather than folded into this result.

**Fully reproducible**: N/A (Not reproducible - no bug survives on current HEAD to be repro'd).
The pre-fix reproduction (0.3.1) itself was Yes-quality: it ran the real, unmodified `Decoder::decode()`
public API against the exact bytes attached to the real upstream issue report, no hand-waving.

## Class

N/A (Not reproducible).

## Root cause

The suspect line is safe on current HEAD because two independent, already-present guards close
every input path that could reach it with an invalid `precision`/`point_transform` combination:

1. `src/decoder.rs`'s `SOF` handling rejects any frame whose `precision` is outside `[2, 16]`
   before `self.frame` is even set (`if !(2..=16).contains(&frame.precision) { ... }`), so every
   `FrameInfo` that ever reaches `decode_scan_lossless` has `precision >= 2`.
2. `src/parser.rs`'s `parse_sos` (added by 95550d7, commit for GitHub #282) rejects any scan whose
   `point_transform` (`Al`, a 4-bit field, so `0..=15`) is `>= frame.precision`, so every
   `ScanInfo` that reaches `decode_scan_lossless` has `point_transform < precision`.

Together: `precision - point_transform >= 1`, hence `precision - point_transform - 1 >= 0` always,
in `u8` arithmetic, so the subtraction at `lossless.rs:112` can never underflow. Issue #277's own
PoC was filed against 0.3.1, before guard 2 existed; that guard is exactly what makes the same
bytes fail cleanly on current HEAD instead of panicking.

## Introduced by

N/A (Not reproducible on current HEAD - this section describes the *fix*, not an unfixed bug). The
originally-panicking behavior traces to whatever commit first wrote the unguarded
`frame.precision - scan.point_transform - 1` expression (not investigated further, since the bug
is already closed); the fix is 95550d7 "Fix panic when prediction calculation is invalid (#282)"
(2025-03-07), already present in current HEAD.

## The fix

No fix needed - already fixed upstream by 95550d7, present in current HEAD/the shipped 0.3.2
release. No `patch.diff` in this candidate folder.

- Project test suite: N/A, not run as part of this candidate (no source change was made - nothing
  to verify before/after). The project's own `tests/crashtest` suite was used as one of the three
  verification approaches above and passed (1/1, `crashtest::crashtest ... ok`) with issue #277's
  PoC dropped into `tests/crashtest/images/`.

## What's not done / open questions

- Issue #277 itself is still open on GitHub; based on this candidate's findings it looks like a
  stale report that 95550d7/the 0.3.2 release already resolved, worth closing (or the maintainers
  linking it to #282) rather than re-investigating further.
- Issue #132 (`decoder.rs:760`, `refine_non_zeroes`/progressive successive-approximation
  refinement) reports the same panic *message* on unrelated code and was **not** investigated by
  this candidate - it's a legitimate separate lead for a future candidate (progressive DCT
  successive-approximation refinement arithmetic), not covered by 95550d7's fix at all since that
  commit only touches lossless-scan parsing.
- The boundary-value sweep only covered the `Predictor::Ra` fast path and the generic `predict()`
  branch's shared guard (both branches read the same `scan.point_transform`/`frame.precision`
  pair checked once in `parse_sos`), and only single-scan lossless frames (hierarchical mode is
  unsupported/rejected elsewhere in the decoder, so multi-frame precision reuse wasn't a candidate
  worth testing).
