# MKV_CUEPOINT_DROPS_ALL_BUT_LAST_TRACKPOSITIONS

`CuePointElement::read` (symphonia-format-mkv/src/segment.rs) stores only a single
`positions: CueTrackPositionsElement` for a `CuePoint`, overwriting it every time a
`CueTrackPositions` child is encountered. A real multi-track Matroska `CuePoint` commonly carries
one `CueTrackPositions` per indexed track, so for any file with more than one such child, all but
the last are silently discarded. `MkvReader::seek_track_by_ts` (symphonia-format-mkv/src/demuxer.rs)
then unconditionally uses that single retained `positions.cluster_pos`/`cluster_rel_pos` for
whichever track is being sought, without ever checking `positions.track` against the requested
track id. This looked suspicious purely from reading the code (candidate handed in with a specific
line reference), so this session went straight to constructing a real trigger file rather than
starting from a heuristic scan.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: crafted a real two-track (h264+aac) Matroska file via a real `ffmpeg` mux, then hand-built a `Cues` element with 2 `CueTrackPositions` per `CuePoint` (mirroring what a real multi-track-aware muxer such as mkvmerge commonly writes), fed through `MkvReader`'s real public `try_new`/`seek` entry points | debug build, `overflow-checks = true` | Reproduced deterministically: seeking track 1 (video) to ts=0 landed at actual_ts=979 instead of 0; a second, independently-crafted file showed seeking track 2 (audio) to ts=3000 fail outright with `SeekErrorKind::OutOfRange` instead of landing near 3000 |
| Repeat-run check | same binary, same 3 input files, 5 consecutive runs | Identical output byte-for-byte all 5 runs (deterministic, no reliance on timing/layout) |

A debug-assertions build was sufficient to observe and confirm the bug; miri/ASAN were not
applicable (no `unsafe` involved - this is a pure logic/data-loss bug in safe Rust) and a fuzz
harness was not needed once the mechanism was understood directly from the code and confirmed with
one deliberately-constructed file (per HEURISTICS.md, this is a logic-bug class best confirmed by
reasoning from the documented invariant + a targeted input, not fuzzing).

## Status

**Confirmed**, reproduces 5/5 runs (debug build, `overflow-checks = true`, deterministic - no
seed/layout dependence).

**Fully reproducible**: **Yes**. The repro opens each Matroska file through `MkvReader`'s real
public `try_new` and drives the real `FormatReader::seek` implementation end-to-end - there is only
one participant here (a file handed to a parser's public entry point), so there is no second
participant to fake; the file itself (with its `Cues` element) is the genuine external input a
real multi-track muxer would produce. All non-`Cues` bytes (EBML header, Tracks, Info, Tags, and
both clusters with every SimpleBlock) are 100% real bytes from an actual `ffmpeg -c:v libx264 -c:a
aac` mux; the `Cues` element replacing ffmpeg's own (which only ever wrote a single track's cue
positions) is the only hand-authored part, built to reflect the documented Matroska behavior of one
`CueTrackPositions` child per indexed track per `CuePoint`.

## Class

logic

## Root cause

Two compounding defects in symphonia-format-mkv:

1. `CuePointElement::read` (segment.rs) does `positions = Some(it.read_master_element()?);` inside
   the `MkvElement::CueTrackPositions` match arm, unconditionally overwriting any previously-parsed
   `CueTrackPositions` for the same `CuePoint`. For a `CuePoint` with N `CueTrackPositions`
   children, only the last one parsed survives.
2. `MkvReader::seek_track_by_ts` (demuxer.rs) uses `target_cue_point.positions.cluster_pos` /
   `.cluster_rel_pos` directly, with no check that `.track` matches the `id` (track number) being
   sought. Even if (1) were fixed to keep a `Vec`, the original code never picked the entry
   belonging to the requested track.

Combined effect: for a `CuePoint` whose retained `CueTrackPositions` happens to belong to a
different track than the one being sought, `seek_track_by_ts` seeks into that *other* track's
cluster/byte offset, then relies on the purely-forward `seek_track_by_ts_forward` scan to locate a
matching frame for the requested track from there. Since the scan can only move forward, being
redirected past the correct cluster produces either a materially wrong `actual_ts` (case A: video
seek to ts=0 landed at 979) or an outright `SeekErrorKind::OutOfRange` when no more matching frames
exist ahead of the wrong anchor within the stream (case B: audio seek to ts=3000 failed instead of
landing near 3019).

Secondary weakness (present but not the primary trigger for either demonstrated case; noted, not
separately exercised): the cue-point search (`for cue_point in &cues.points { if cue_point.time >
ts { break; } ... }`) assumes `cues.points` is sorted ascending by `time`, which the Matroska
specification does not guarantee. The bounded fix below also removes this assumption (full linear
scan for the maximal `time <= ts`) since it was a one-line-cost change alongside the primary fix,
but no crafted-unsorted-cues input was separately constructed to demonstrate it in isolation this
session.

## Introduced by

93d16eb0b138f0502229cf34688edefcb2224236 "mkv: Rewrite EBML iterator." (2025-01-14). Verified via
git show on the pre/post versions of `seek_track_by_ts`: before this commit, `seek_track_by_ts`
searched a fully-parsed `self.clusters` list and explicitly filtered blocks by
`block.track as u32 != track_id { continue; }`, so it never depended on `CueTrackPositions.track`
being correct for the requested track - it found the right block by scanning real blocks, not by
trusting the cue's per-track label. This commit replaced that with the current
`target_cue_point.positions.cluster_pos`/`cluster_rel_pos`-based seek and never carried the
track-matching check over. The single-position `CuePointElement.positions` field itself predates
this commit (present since the very first MKV-support commit, 9780210, which used a generic
`cues: Vec<Cue>` seek index instead of the per-track-positions field, so the field's single-value
design was harmless until 93d16eb made it load-bearing without the track check).

## The fix

Two changes in symphonia-format-mkv, applied and verified against the worktree's own copy of the
source (patch.diff):

1. `segment.rs`: `CuePointElement.positions` changed from a single `CueTrackPositionsElement` to
   `Box<[CueTrackPositionsElement]>`; `CuePointElement::read` now pushes every parsed
   `CueTrackPositions` child instead of overwriting, and validates the list is non-empty (same
   "at least one" invariant the old code enforced via `positions.ok_or(...)`, just applied to the
   collected list).
2. `demuxer.rs`: `seek_track_by_ts` now selects, from `target_cue_point.positions`, the entry whose
   `.track` matches the requested `id`; if none matches (e.g. a cue point that genuinely omits the
   sought track), it falls back to the entry with the smallest `cluster_pos` - since the seek scan
   is forward-only, seeking to the earliest available cluster position can never skip past the true
   target frame, only require scanning a bit further, so this fallback cannot introduce a new
   wrong-answer case. The cue-point search loop was also changed from an early `break` (assuming
   sorted `cues.points`) to a full scan tracking the maximal `time <= ts`.

Verified against **two independent boundary cases** (CONFIRMATION.md section 7 - not just the one
input that first triggered it):
  - Case A (`poisoned.mkv`): `CuePoint` has `[video(track 1), audio(track 2)]` -> audio's position
    was retained pre-fix. Before: `seek(track_id=1, ts=0)` -> `actual_ts=979`. After: `actual_ts=0`
    (matches the correctly-cued reference file).
  - Case B (`poisoned2.mkv`): `CuePoint` has `[audio(track 2), video(track 1)]` -> video's position
    was retained pre-fix, anchored inside cluster 2. Before: `seek(track_id=2, ts=3000)` ->
    `Err(SeekError(OutOfRange))`. After: `actual_ts=3019` (matches the reference file's `3019`).

- Project test suite: symphonia-format-mkv (`cargo test -p symphonia-format-mkv`): 3 passed / 0
  failed, both before and after the patch (no MKV-specific cue/seek tests exist in this package
  today - the 3 tests are unrelated EBML vint-reading unit tests). symphonia facade
  (`cargo test -p symphonia`): 0 tests either way. `cargo build --workspace` succeeds both before
  and after. `cargo clippy -p symphonia-format-mkv --all-targets` shows no new warnings introduced
  by the patch.

## What's not done / open questions

- The secondary "cues must be time-sorted" assumption was fixed as a low-risk one-line change
  alongside the primary fix, but no dedicated crafted-unsorted-`Cues` input was built to
  demonstrate it failing in isolation before the fix - it was folded in because it was cheap and
  directly named in the candidate description, not because it was independently confirmed as a
  standalone repro this session.
- The chosen fallback (smallest `cluster_pos` when no exact track match exists in a `CuePoint`) is
  a conservative choice reasoned to be safe given the forward-only scan design, but was not
  separately stress-tested with a `CuePoint` that omits the sought track's `CueTrackPositions`
  entirely (both crafted files always included an entry for every track present, just mislabeled
  which one was kept).
- Other Matroska cue/seek code paths were not audited for the same "trust a single retained
  value across multiple children without validating identity" pattern; this session focused solely
  on `CuePointElement`/`seek_track_by_ts`.
