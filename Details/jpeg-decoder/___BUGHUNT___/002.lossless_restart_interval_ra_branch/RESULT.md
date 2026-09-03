# lossless_restart_interval_ra_branch

Follow-up to candidate 001 (frozen restart flag in `decode_scan_lossless`'s generic `predict()`
branch, predictor selections 2-7). This candidate targeted the two other restart/predictor
interaction spots flagged as unchecked: the special-cased `Predictor::Ra` branch (selection 1)
and the restart-marker resync logic in the entropy-decode loop itself.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading: compared `decode_scan_lossless`'s `Predictor::Ra` branch (src/decoder/lossless.rs lines ~108-138) against its generic `predict()` branch and against baseline JPEG's `decode_scan` restart handling (src/decoder.rs ~915-955) | Static read, base_commit eb2d7c0 | Found the `Ra` branch never reads `mcus_left_until_restart`/any restart state at all - it always predicts from the previously *decoded* neighbour, with no reset at restart boundaries. Also confirmed the entropy-decode loop's own restart resync (RST-number matching, `huffman.reset()`, bit-reader realignment via `take_marker`) is structurally identical to baseline JPEG's own (proven) restart handling - no separate bug found there. |
| Hand-built synthetic lossless JPEG + real end-to-end decode via public API (`jpeg_decoder::Decoder`), boundary-value construction: 4x4 image, predictor selection 1 (Ra), restart_interval=5, ground truth chosen by hand from the correct spec algorithm | `repro/`, `cargo run --release` (also re-run under `+1.90.0` since this sandbox's stable rustc 1.61.0 can't build rayon-core's current lockfile version - unrelated to the bug) | Reproduced immediately: 6 of 16 pixels decoded wrong, deterministically (3/3 runs) |

Given the second approach reproduced cleanly and deterministically on the first real attempt,
the soft cap of 3 escalating tools was not exhausted - no sanitizer/fuzzing was needed for what
is a straightforward, fully deterministic logic bug (see `CONFIRMATION.md`: bit-for-bit
repeatable is the gold standard, already met here).

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs, release build (`+1.90.0`), same result
every time (no randomness/seed anywhere in the input construction or the decode path).

**Fully reproducible**: **Yes**. `repro/src/main.rs` hand-builds a complete, spec-valid lossless
JPEG byte stream (SOI/SOF3/DHT/DRI/SOS/entropy-coded data with real RST markers/EOI) and feeds
it through the real, public `jpeg_decoder::Decoder::decode()` end to end - there is no lossless
encoder available for this decode-only crate, so this is the standard way to drive it (same
approach candidate 001 used); this is not "faking a caller reacting to a peer's input" in the
CONFIRMATION.md section 11 sense - the finding is entirely about how one function reacts to a
byte stream, and the repro constructs and feeds that byte stream through the crate's own real
public parsing/decoding entry point, not a hand-built in-memory stand-in for some second
in-process participant.

## Class

logic (silent wrong pixel output, not a crash/panic)

## Root cause

`Decoder::decode_scan_lossless` (src/decoder/lossless.rs) decodes a lossless scan in two
sequential passes: an entropy-decode loop that reads per-pixel Huffman-coded DC differences
(and, in the same loop, does all of the restart-marker bookkeeping: matching `RSTn` numbers,
resetting the Huffman bit-reader), followed by a **separate** prediction/reconstruction pass
that turns those differences into actual pixel values using neighbouring already-reconstructed
pixels.

For predictor selection 1 (Ra), that second pass is a hand-unrolled fast path (lines ~108-138)
that:
1. computes pixel (0,0) using the frame's default prediction value,
2. walks down the left column using the *previous row's* pixel as predictor,
3. walks each row left-to-right using the *previous column's* pixel as predictor,

and never once consults any restart state. Per ITU-T T.81 section H.1.2.3, the predictor must
reset to the same default value used for pixel (0,0) at the first sample following *every*
restart marker, regardless of predictor selection or the pixel's row/column position - this
branch simply never implements that reset, so every pixel immediately following a restart
marker (and everything derived from it later in the same row/column, since the wrong value then
propagates forward as the "previous" pixel) is decoded from the wrong prediction base.

This is a different failure of the same underlying architectural issue candidate 001 already
found in the *generic* `predict()` branch (predictor selections 2-7): splitting reconstruction
into a second pass that runs after the entropy-decode loop's restart bookkeeping has already
been consumed, without carrying the per-pixel restart-boundary information forward into that
second pass. Candidate 001's branch at least attempts to pass a `restart` flag into `predict()`
(computed wrong - frozen at the loop's final value); this branch does not attempt it at all.

The entropy-decode loop's own restart resync (RST marker matching/`huffman.reset()`/expected
sequence number tracking, lines ~52-84) was checked directly against baseline JPEG's own
`decode_scan`'s equivalent logic (src/decoder.rs ~915-955) and found structurally identical -
same marker-matching, same decrement/reset bookkeeping, same use of `HuffmanDecoder::take_marker`
for bit-reader realignment at a marker. No separate bug found in that part; the difference
between lossless and baseline is that baseline additionally resets `dc_predictors`/`eob_run`
inline in that same loop (since it reconstructs coefficients immediately), while lossless defers
reconstruction to the second pass entirely - which is exactly where both this bug and candidate
001's live.

## Introduced by

770a5a2 "performance regression" (2021-08-02). This commit split `decode_scan_lossless` from a
single combined decode+reconstruct loop (which read `mcus_left_until_restart` live, per pixel,
correctly) into the current two-pass structure, and introduced the special-cased `Predictor::Ra`
fast path shown above - which never reads restart state at all. It also introduced candidate
001's frozen-restart-flag bug in the same commit, in the generic `predict()` branch it split out
alongside this one. Verified via `git log -p --follow -- src/decoder/lossless.rs`, confirmed with
`git show 770a5a25bfc77fc1d9c5d369ee440ce55c46e362 -- src/decoder/lossless.rs`.

## The fix

Record, during the entropy-decode loop (where the restart bookkeeping is already live), which
pixel index is the first one after a restart marker, in a new `restarts: Vec<bool>` alongside the
existing `differences` array. The `Predictor::Ra` branch's two carry-forward loops (leftmost
column, then each row) then check `restarts[idx]` per pixel and use the frame's default
prediction value there instead of carrying over the previous decoded pixel - mirroring exactly
the reset rule already used (correctly) for pixel (0,0).

This patch is scoped only to the `Predictor::Ra` branch (this candidate's own finding) - it adds
the `restarts` vector and populates it in the shared entropy-decode loop, but does not touch the
generic `predict()` branch's own (separately tracked, candidate 001's) frozen-flag bug, per
SKILL.md's "one bug per candidate" guidance. A real fix for both together would likely want to
thread the same `restarts` vector into the generic branch's `predict(..., restart)` call too,
replacing its `mcus_left_until_restart == self.restart_interval - 1` frozen check.

Verified: `repro/` (see `output.txt`) - 6/16 mismatched pixels before the patch, 0/16 after,
isolated via `git stash`/`git stash pop` around the exact same worktree checkout so the only
difference between the two runs is `patch.diff`'s own hunk (SKILL.md pitfall #6).

- Project test suite (`cargo +1.90.0 test --release`, all suites: unittests + tests/lib.rs +
  tests/rayon*.rs + doc-tests): 23 passed / 0 failed before the patch, 23 passed / 0 failed after
  - identical breakdown both times, no new failures, no regressions.

## What's not done / open questions

- The generic `predict()` branch's own frozen-restart-flag bug (candidate 001) is a separate,
  already-tracked finding - not touched by this patch. A combined fix threading one shared
  `restarts` vector into both branches would be a natural follow-up if both are ever fixed
  together.
- Only 8-bit precision / point_transform=0 was exercised in the repro (matching candidate 001's
  own repro parameters for consistency); the fix's `1 << (frame.precision - scan.point_transform
  - 1)` expression is copied verbatim from the branch's own existing pixel-(0,0) handling, so it
  inherits that same expression's pre-existing lack of the `input_precision > 1 + point_transform`
  guard that the generic `predict()` function has (relevant to the unrelated `#282` panic fix in
  95550d7) - not a new risk introduced by this patch, since pixel (0,0) already uses the exact
  same unguarded expression today, but worth a maintainer's attention if that guard is ever added
  to one call site and not the other.
- Multi-component (interleaved) lossless scans with predictor Ra + restart intervals were not
  separately exercised (the repro uses 1 component) - the restart bookkeeping in the entropy loop
  operates per-MCU (i.e. per whole set of components), not per-component, and the fix's
  `restarts` vector is indexed identically to `differences[i]`, so the same fix should apply
  uniformly per component, but this wasn't independently verified with a multi-component repro.
