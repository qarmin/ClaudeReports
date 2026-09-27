# Frozen restart flag in lossless `predict()` branch (predictor selections 2-7)

`src/decoder/lossless.rs`, `Decoder::decode_scan_lossless`, the `else` branch (predictor
selections 2-7 / `NoPrediction`; selection 1, `Ra`, has its own separate code path). Picked up
from a prior perf-optimize session's writeup (`___OTHER___/001.frozen_restart_flag_lossless_predict/`,
same repo, base_commit eb2d7c0) for formal re-verification, root-cause confirmation, and a bounded
fix attempt in this bug-hunt workspace.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading of `decode_scan_lossless` against the prior report's claim | current HEAD (eb2d7c0), same as base_commit | Confirmed the exact code shape described still exists unchanged: `mcus_left_until_restart` is read once per pixel in the second (prediction) loop via `self.restart_interval > 0 && mcus_left_until_restart == self.restart_interval - 1`, but never mutated in that loop - so it is one frozen value from whenever the first (entropy-decode) loop finished, reused for every `predict()` call. |
| Boundary-value construction: standalone synthetic lossless JPEG (SOF3 + DHT + DRI + SOS, predictor selection 4, restart_interval=5, 4x4 image) hand-built from a known ground-truth pixel grid, fed through the real `jpeg_decoder::Decoder::decode()` public API, release build | 4/4 runs (3x release, 1x debug), bit-for-bit identical output every time - fully deterministic, no layout/seed dependence | 12 of 16 pixels wrong vs. ground truth - reproduces exactly as the prior report recorded |
| Same repro, varied to restart_interval=3 (same predictor selection 4, same 4x4 image) against pristine tree | release build | 10 of 16 pixels wrong - confirms the bug is not an artifact of the one specific restart_interval=5 parameter set, but of the general mechanism |

No sanitizer/miri/fuzzing was needed or attempted: this is a pure logic bug (wrong output, not a
crash/UB), and the code reading plus a direct boundary-value repro against the real public decode
API already gave a full, deterministic, repeatable confirmation on the first approach - per
`HEURISTICS.md`'s guidance for this bug class ("differential testing against a reference /
constructing an input that should violate a documented invariant"), which is exactly what both
repro variants above do (compare against a hand-computed spec-conformant ground truth).

## Status

**Confirmed.** Reproduces 4/4 runs against base_commit eb2d7c0 (3x `cargo run --release`, 1x plain
`cargo run`), byte-for-byte identical mismatched output every run - fully deterministic, not a
layout/seed-dependent finding. A second parameter variation (restart_interval=3 instead of 5, same
predictor selection) also reproduces (10/16 pixels wrong) against the pristine tree, confirming the
bug is general to the mechanism and not an artifact of one specific input.

**Fully reproducible:** Yes. The repro feeds a hand-built but fully spec-valid lossless JPEG byte
stream through `jpeg_decoder::Decoder`'s real, public `decode()` entry point - the actual affected
code path end to end. This is a single-participant case (CONFIRMATION.md section 11's "no second
participant" exception applies directly): the input is a single external artifact (a JPEG file), the
same kind of bytes a real caller/attacker would hand to `Decoder::new(...).decode()`; there is no
peer/caller whose reaction is being faked, so no second-participant repro is needed here. The only
hand-authored part is the input file's bytes themselves, which is the same thing a real fuzzer or a
real malformed file on disk would also just be, not a stand-in for another program's behavior.

## Class

logic

## Root cause

`decode_scan_lossless` (predictor selections 2-7 / `NoPrediction` branch) makes two sequential
passes over every pixel:

1. An entropy-decode loop that reads Huffman-coded DC differences for every MCU in raster order,
   and maintains `mcus_left_until_restart` - decrementing it each MCU, resetting it to
   `self.restart_interval` and consuming an `RSTn` marker whenever it hits 0.
2. A prediction loop that walks the same pixels again and computes each one's predicted value via
   `predict(..., restart)`, where (before this fix) `restart` was computed as
   `self.restart_interval > 0 && mcus_left_until_restart == self.restart_interval - 1`.

`mcus_left_until_restart` is never touched in the second loop - only read, at that one call site,
so it is whatever value the first loop happened to leave it at when it finished. `restart` was
therefore not a per-pixel "is this pixel right after a restart marker" check, despite `predict()`
using it exactly that way (`if (ix == 0 && iy == 0) || restart { /* use the default reset value */
}`); it was a single value, frozen at end-of-first-loop, reused for literally every `predict()` call
in the second loop. Depending on `width * height` and `restart_interval`, the frozen value can land
on either side of `restart_interval - 1`: if it happens to equal it, every pixel past (0,0) gets the
constant "reset" prediction (image decoded almost entirely wrong); otherwise no pixel ever gets the
reset prediction, so pixels that genuinely follow an `RSTn` marker use stale predictor state that
the format requires to be discarded at a restart (those specific pixels decode wrong, others happen
to be right). Both failure modes are silent wrong output, not a panic.

Reachability: `src/parser.rs`'s `parse_dri` (restart interval) and `parse_sos`'s
`predictor_selection` derivation are fully independent of each other, `Decoder::decode_internal`'s
marker dispatch places no restriction tying the two together, and ITU-T T.81 itself has no such
restriction either (restart markers, Annex B.2.4.4/B.2.5, are a general entropy-coding-layer
mechanism orthogonal to the lossless predictor selection, Annex H Table H.1) - any of predictor
selections 1-7 may legally combine with any restart interval. None of the 10 lossless images under
`tests/reftest/images/lossless/1/*.jpg` contain a `DRI` marker, so this combination has no existing
test coverage in this repo (a test-suite gap, not evidence the combination is unreachable).

## Introduced by

770a5a25 "performance regression" (2021-08-02) - this commit restructured
`decode_scan_lossless` from a single fused decode+predict pass into the current two-pass shape
(entropy-decode loop, then a separate prediction loop), and is where the `mcus_left_until_restart
== self.restart_interval - 1` expression (reading restart state after the loop that maintains it
has already finished) was introduced, per `git blame` on that line. Confirmed via `git log -S
"mcus_left_until_restart == self.restart_interval - 1"` (only match) and `git blame` on the
`predict()` call site.

## The fix

Record each MCU's actual restart-boundary status while the entropy-decode loop already knows it
(right where it currently resets `mcus_left_until_restart` and consumes the `RSTn` marker), into a
new `mcu_restarts: Vec<bool>` indexed by raster position, then index that per-pixel in the
prediction loop instead of recomputing (incorrectly) from the post-loop value of
`mcus_left_until_restart`. See `patch.diff` (+10/-4 lines, `src/decoder/lossless.rs` only).

Verified:
- Re-ran the standalone repro (restart_interval=5, predictor selection 4) against the patched tree:
  0 of 16 pixels mismatched (previously 12 of 16) - bug gone, not just changed shape.
- Re-ran with a second, different restart_interval=3 (same predictor selection) against the patched
  tree: 0 of 16 pixels mismatched (this input also failed - 10 of 16 wrong - against the pristine
  tree, confirming the fix isn't narrowly tailored to the one exact input that first triggered it,
  per CONFIRMATION.md section 7).
- Project test suite (`cargo test --release`, toolchain override to 1.86.0 in this worktree only,
  purely to dodge an unrelated `unicode-ident`/MSRV lockfile mismatch against the repo's pinned
  1.61.0 toolchain that blocks `cargo test` entirely regardless of this patch - the pinned 1.61.0
  toolchain itself was still used for every build/run of the actual repro above): **23 passed, 0
  failed, 0 ignored** before the patch, and **23 passed, 0 failed, 0 ignored** after - identical
  counts, no regressions, no new failures.

## What's not done / open questions

- The same "read restart state after the loop that maintains it has already finished" bug class was
  checked only in this one spot (`decode_scan_lossless`'s generic predictor branch). The `Ra`
  predictor-selection-1 branch just above it in the same function does not read
  `mcus_left_until_restart` at all (it has its own restart-agnostic column/row-wise reconstruction),
  so it was not affected and was not touched by this fix.
- No broader fuzzing campaign was run against the lossless decode path in general (out of scope for
  this candidate - see `fuzz-bug-repro` for that, invoked separately, not by this skill's own
  fuzz-harness step which was not needed here since the first approach already gave a deterministic
  confirmation).
- `tests/reftest/images/lossless/` still has no test image combining a restart interval with a
  non-`Ra` predictor selection - a maintainer taking this patch may want to add one (a real spec-
  valid lossless JPEG with `DRI` + predictor selection 2-7) as a regression test; this candidate's
  `repro/` demonstrates the bug and the fix but is not itself wired into the project's own test
  suite (per the skill's own scoping, this workspace's job is confirm+patch, not adding project-side
  regression tests).
