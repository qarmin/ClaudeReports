# lossless_restart_boundary_pr287_cross_check

Diff-review of open upstream PR image-rs/jpeg-decoder#287 ("Fix lossless restart boundary handling
in reconstruction", closes #286), which reworks `decode_scan_lossless`'s restart-boundary
determination (both the generic `predict()` branch and the special-cased `Predictor::Ra` fast
path) in `src/decoder/lossless.rs` to compute `is_restart_boundary(pixel_index) = restart_interval
> 0 && pixel_index > 0 && pixel_index % restart_interval == 0` instead of the frozen/absent state
this workspace's own already-Confirmed candidates 001 (generic branch) and 002 (Ra branch) found
and fixed. Task: does PR #287's formula correctly handle multi-component lossless scans and fully
match ITU-T T.81 H.1.2.3, or does it leave a residual edge case 001/002's own fixes might also
miss?

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual diff review: PR #287's `src/decoder/lossless.rs` hunk (`gh pr diff 287`) read side by side with candidates 001/002's own patch.diff and RESULT.md | Static read, base_commit eb2d7c0 | `is_restart_boundary`'s arithmetic is mathematically equivalent to the actual restart-marker positions the entropy-decode loop consumes (both derive from the same `MCU index is a nonzero multiple of restart_interval` condition; the entropy loop's own `mcus_left_until_restart == 0` check fires at exactly those same positions) - no obvious flaw in the restart-boundary formula itself, for either branch, including multi-component (the formula is per-MCU/`pixel_index`, independent of `ncomp`, applied identically to each component). PR #287 also newly guards its shared `prediction_baseline` with `frame.precision > 1 + point_transform` - the exact same too-strict threshold already flagged in this workspace's candidate 003 for the pre-existing generic `predict()` function - but the *pre-PR287* `Predictor::Ra` branch computes this value unconditionally (no guard at all), so this looked like a possible newly-introduced regression specific to PR #287, worth verifying empirically. |
| Manual boundary-value construction: a standalone multi-scenario Rust repro (`repro/src/main.rs`, scenarios A-J) hand-building spec-valid lossless JPEG bitstreams - single- and multi-component (up to 3, interleaved), various predictor selections (1/Ra and 4/generic), restart intervals that do and do not evenly divide the image width (mid-row restarts), and a precision/point_transform boundary (`point_transform == precision - 1`) isolated to a single pixel - each with a hand-computed ground truth per ITU-T T.81 H.1.2.1/H.1.2.3, fed through the real `jpeg_decoder::Decoder::decode()` public API | Built and run against three real trees: pristine base_commit, base_commit + PR #287's `src/decoder/lossless.rs` hunk only, and base_commit + PR #287 + this candidate's own additional guard fix (`patch.diff`). Release build (`+1.90.0` toolchain override for the repro binary only - this sandbox's pinned 1.61.0 stable can't build the current rayon-core lockfile version, same MSRV/lockfile mismatch candidates 001/002 already noted; unrelated to anything under test), 3 full runs each of the full scenario suite, bit-for-bit identical output every run | PR #287 correctly fixes the original restart-boundary bugs (scenarios A/B, matching 001/002's own repro shape) **and** correctly extends the fix to multi-component interleaved scans with a restart interval that doesn't evenly divide the image width, for the generic branch (scenario D: 0/75 mismatched). It also confirmed the suspected regression: scenario E (`Predictor::Ra`, precision=4, point_transform=3, single pixel, no restart) matches against pristine HEAD (0/1 mismatched - current Ra branch has no guard bug) but **mismatches against PR #287 applied** (1/1 mismatched, expected 8 got 0) - PR #287 imports candidate 003's already-known off-by-one guard bug into a branch that currently doesn't have it. Applying this candidate's own additional fix (loosen the shared `prediction_baseline` computation, matching candidate 003's own fix direction) on top of PR #287 makes scenario E match again (0/1) without disturbing anything else PR #287 fixed. |
| Follow-up boundary-value construction to root-cause an unexpected residual mismatch found in scenario C (multi-component `Predictor::Ra`, restart_interval=7, width=6 so restarts land mid-row): scenarios G (isolate component c=2's own value formula as a single-component scan), H (same 3-component grid as C but restart_interval=0), I (same as C with component order reversed), J (same grid/restart as C but generic branch/predictor 4 instead of Ra) | Same repro/build, same three trees | This residual mismatch (10/90 in scenario C, identically 10/90 against pristine, PR #287-only, *and* PR #287 + this candidate's fix - i.e. **unrelated to and not affected by either the restart-boundary formula or the guard question**) turned out to require *both* multi-component (G alone: 0/30 mismatched, H alone: 0/90 mismatched) *and* a restart interval that doesn't evenly divide the width, and follows the *array position* of the last scan component specifically (I: reversing component order moves the mismatch to whichever value formula is now at index `ncomp-1`, not a particular value range) - it also affects the generic branch under the same grid/restart combination (J: 4/90 mismatched), just at fewer positions. This is a distinct, apparently pre-existing anomaly, out of scope for this candidate's own question (which is specifically about the restart-boundary/guard formulas PR #287 changes) - not root-caused further here; see "New suspects noticed" below. |

Given the second approach reproduced the suspected regression cleanly and deterministically, and
the third approach both confirmed the residual mismatch it turned up is unrelated to what this
candidate was asked to check (present identically regardless of which of the three trees was
used) and pinned down enough of its shape to write a useful backlog note, the soft cap of 3
escalating approaches was reached without a live, still-open thread specific to this candidate's
own question - no sanitizer/miri/fuzzing was needed for what is a deterministic logic-bug
diff-review.

## Status

**Confirmed.** PR #287, as currently posted, would introduce a real regression relative to current
HEAD: applying only its `src/decoder/lossless.rs` hunk to base_commit eb2d7c0 makes the
`Predictor::Ra` branch inherit candidate 003's already-known off-by-one default-prediction guard
bug (previously specific to the generic `predict()` branch), because PR #287 unifies both
branches' default-prediction computation into one shared `prediction_baseline` expression that
carries over the same too-strict `frame.precision > 1 + point_transform` guard. Reproduces
deterministically, 3/3 runs, release build, byte-for-byte identical mismatch every run (scenario E:
expected 8, got 0) - not layout/seed-dependent. Separately, PR #287's actual restart-boundary
fix itself (the thing candidates 001/002 and issue #286 are about) is correct, including for
multi-component interleaved scans with a restart interval that does not evenly divide the image
width - no residual edge case found there.

**Fully reproducible:** Yes. The repro feeds hand-built but fully spec-valid lossless JPEG byte
streams through `jpeg_decoder::Decoder`'s real, public `decode()` entry point, against a real
build of the actual tree under review (base_commit + PR #287's own diff, applied via `git apply`
from a file bundled in this candidate folder, `pr287_under_review.diff`) - the same class of
single-participant repro as candidates 001/002/003 in this workspace (`CONFIRMATION.md` section
11's "no second participant" exception: the input is a single external artifact, a JPEG byte
stream, with no peer/caller whose reaction is being faked).

## Class

logic

## Root cause

PR #287 replaces both `decode_scan_lossless` reconstruction branches' restart-boundary
determination with a single closure, `is_restart_boundary`, and (correctly) fixes candidates
001/002's own bug: the closure computes each pixel's restart-boundary status from
`pixel_index % restart_interval`, an arithmetic recomputation that is mathematically equivalent
to (and confirmed empirically to match) the actual restart-marker positions consumed by the
shared entropy-decode loop, for both branches and for multi-component interleaved scans (the
closure operates on `pixel_index = mcu_y * width + mcu_x`, the same value regardless of `ncomp`,
applied identically per component).

Separately, and not required by that fix, PR #287 also *refactors* the previously hand-unrolled
`Predictor::Ra` branch into a loop shape that shares one `prediction_baseline` value with the
generic branch's default-prediction case:

```rust
let prediction_baseline = if frame.precision > 1 + point_transform {
    1i32 << (frame.precision - point_transform - 1)
} else {
    0
};
```

This is the exact guard local candidate 003 already flagged as one notch stricter than the
invariant `src/parser.rs::parse_sos` enforces at parse time (`point_transform < frame.precision`,
added independently by commit 95550d7 / #282) - the guard's own underflow concern is moot given
that invariant (`frame.precision - point_transform - 1` can never underflow), but its threshold is
still wrong by one: at `point_transform == precision - 1`, `frame.precision > 1 + point_transform`
is false, so this returns 0 instead of the spec-correct `1 << (precision - point_transform - 1)`
(which, at that exact boundary, is `1 << 0 == 1`). Candidate 003 already tracks this bug for the
*pre-existing* generic `predict()` function (which has always had this exact guard). What's new in
PR #287: the *pre-PR287* `Predictor::Ra` branch computes this value completely unconditionally
(`1 << (frame.precision - scan.point_transform - 1) as i32`, no guard at all) - so it currently
does *not* have candidate 003's bug, only the generic branch does. By routing both branches through
one shared, guarded `prediction_baseline`, PR #287 would import candidate 003's bug into the Ra
branch too, where it doesn't currently exist. This is a genuine (if narrow) regression PR #287
would introduce if merged as posted - not a residual case of 001/002's own bug (which is entirely
about restart-boundary determination, not the default-prediction guard), and not something 001/002's
own local fixes in this workspace would have introduced either (neither patch touches this guard).

## Introduced by

Not applicable in the usual sense - this is a regression in an *unmerged, open* PR
(image-rs/jpeg-decoder#287, as of the date this candidate was investigated), not in code present
in any commit of the tracked history. The regression would be introduced by PR #287 itself, in the
same commit(s)/diff that unifies the two branches' default-prediction computation into the shared
`prediction_baseline` closure (visible directly in `gh pr diff 287`'s `src/decoder/lossless.rs`
hunk, the same hunk analyzed above).

## The fix

Loosen (or remove, since it can never fire) `prediction_baseline`'s guard, mirroring candidate
003's own fix direction for the generic branch's `predict()` function - since
`src/parser.rs::parse_sos` already guarantees `point_transform < frame.precision` before
`decode_scan_lossless` ever runs, `frame.precision - point_transform - 1` cannot underflow, so the
guard can simply be dropped:

```rust
let prediction_baseline = 1i32 << (frame.precision - point_transform - 1);
```

See `patch.diff` (+7/-5 lines against a tree with PR #287's own `src/decoder/lossless.rs` hunk
already applied - see `pr287_under_review.diff`, also bundled in this folder, and this candidate's
`justfile` for the full apply order). This patch is intentionally scoped only to the regression
this candidate found (the Ra-branch guard) - it does not touch candidate 003's own already-tracked
bug in the generic `predict()` function, which is a separate, already-Confirmed candidate in this
workspace and remains present (in both the pristine tree and PR #287 alike) either way; a
maintainer applying both PR #287 and candidate 003's own fix together would resolve both at once,
since they're the same underlying too-strict guard duplicated in two places once PR #287 lands.

Verified:
- Re-ran the standalone repro against a tree with PR #287's own diff applied but not yet this
  candidate's fix: scenario E mismatches (1/1, expected 8 got 0) - confirms the regression exists
  in PR #287 as posted.
- Re-ran the same repro against PR #287 + this candidate's `patch.diff`: scenario E now matches
  (0/1) - regression fixed. Scenarios A/B/C/D/F/G/H/I/J are unaffected by this patch either way
  (F, candidate 003's own separate bug in the generic branch, remains mismatched both before and
  after, exactly as expected since this patch doesn't touch that code path; C/G/H/I/J, the
  separate multi-component+restart anomaly noted above, also remain unaffected both before and
  after, confirming it's unrelated to this patch).
- Project test suite (`cargo +1.90.0 test --release`, same toolchain-override reasoning as
  candidates 001/002 - unrelated to anything under test, only needed because this sandbox's pinned
  1.61.0 stable can't build the current rayon-core lockfile version): **23 passed, 0 failed, 0
  ignored** against PR #287's own diff alone, and **23 passed, 0 failed, 0 ignored** against PR
  #287 + this candidate's `patch.diff` - identical counts, no regressions, no new failures. (PR
  #287's own new reftest fixture, `tests/reftest/images/lossless/3/`, was not applied in either
  tree tested here - only the `src/decoder/lossless.rs` code hunk was, since the binary fixture
  files don't apply via plain `git apply`; this doesn't affect the counts above since that fixture
  wasn't present in either tree compared.)

## What's not done / open questions

- **New suspect, not investigated further here** (out of scope for this candidate's own question,
  and not affected by its patch either way): scenarios C/G/H/I/J above establish a distinct,
  apparently pre-existing anomaly - present identically in the pristine tree, PR #287 alone, and PR
  #287 + this candidate's fix - where a multi-component (`ncomp > 1`) interleaved lossless scan
  combined with a restart interval that does not evenly divide the image width produces an
  off-by-one value at each row's `x=0` position (between restarts) for the *last* scan component
  specifically (confirmed to follow array position via scenario I, not a particular value range),
  affecting both the `Predictor::Ra` branch (scenario C, 10/90 mismatched) and the generic branch
  under the same grid/restart combination (scenario J, 4/90 mismatched) - but *not* present without
  a restart interval (scenario H, 0/90) or without multi-component (scenario G, 0/30). Root cause
  not identified - candidate reproductions and parameters are preserved in `repro/src/main.rs`
  (scenarios C/G/H/I/J) for a future session to pick up; worth checking first whether it's an
  artifact of this candidate's own hand-rolled `BitWriter`/restart-alignment encoder logic (which
  has not been independently validated against a real third-party lossless encoder for the
  multi-component case) rather than a genuine decoder bug, before assuming the latter.
- This candidate did not check whether PR #287's restructuring of the `Predictor::Ra` branch (from
  a hand-unrolled two-pass loop into a single `mcu_y`/`mcu_x` loop matching the generic branch's
  shape) changes performance characteristics - out of scope (see `perf-optimize` for that).
- Only 8-bit and 4-bit (for the boundary scenarios) precision were exercised; PR #287's own bundled
  test fixture (`tests/reftest/images/lossless/3/restart_psv1_dri3.jpg`, 16-bit, real `cjpeg
  -lossless` output) was not independently re-verified in this session (its binary fixture files
  don't apply via plain `git apply` without the rest of the PR's commit metadata) - PR #287's PR
  description implies it was checked against a real third-party encoder's output, which this
  candidate's own hand-built repro cannot substitute for as *additional* evidence.
