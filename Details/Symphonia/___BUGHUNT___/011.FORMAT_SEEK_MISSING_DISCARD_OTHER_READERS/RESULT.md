# FORMAT_SEEK_MISSING_DISCARD_OTHER_READERS

Candidate 005 (`OGG_SEEK_CROSS_STREAM_INFINITE_LOOP`) confirmed that `symphonia-format-ogg`'s
`do_seek()` peeks the next logical packet without consuming it, and on a track mismatch does
`continue;` without ever calling `discard_logical_packet()` - re-peeking the same packet forever.
This candidate checks whether the other `FormatReader::seek()` implementations that consume
packets/frames in a loop while searching for a target timestamp
(`symphonia-format-mkv`, `symphonia-format-isomp4`, `symphonia-format-riff`) have the same
peek-then-maybe-skip-without-discard shape.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: grepped all three crates for `peek`/`.front()`/loop `continue` sites near their `seek`/`seek_track_by_*` implementations | `grep -rn "peek\|\.front()\|fn peek"` and `grep -n "continue"` across `symphonia-format-mkv/src`, `symphonia-format-isomp4/src`, `symphonia-format-riff/src` | `symphonia-format-riff` (WAVE and AIFF): no loop at all in `seek()` - both compute an absolute byte offset directly from the requested timestamp and either `Seek::seek` or `ignore_bytes` to it once; single-track formats only (`self.tracks[0]`), so there is no cross-track skip logic to have this bug class at all. `symphonia-format-isomp4`: `seek_track_by_ts`'s `'locate: loop` iterates `self.segs` with `.skip(seg_skip)`, and unconditionally sets `seg_skip = seg_idx + 1` for every segment it examines (whether or not it contained the target sample) before ever looping back - forward progress is unconditional, no peek/re-observe of the same segment is possible. `symphonia-format-mkv`: the only `.front()` (peek-like) use is `seek_track_by_ts_forward`'s `while let Some(frame) = self.frames.front()` loop; unlike Ogg's `continue` arm, its `else` branch (frame's `pts` not yet reached its target *or* wrong `track_num`) calls `self.frames.pop_front()` in both cases, i.e. it always discards the examined item before the next iteration. Two other `continue` sites in mkv (`demuxer.rs:118,182`) are unrelated finite `for` loops over already-collected `Vec`s during initial segment/seek-head scanning, not part of any packet-consumption loop. |
| Real end-to-end execution: standalone repro (see `repro/`) exercising the public `MkvReader`/`IsoMp4Reader` `FormatReader::seek()` API against real `ffmpeg`-produced two-track files (`assets/two_track.webm`, libvorbis; `assets/two_track.mp4`, AAC), seeking every track to 6 timestamps spread across the full duration (0.1s-7.9s of an 8s file), on both a seekable (`File`) and non-seekable (`ReadOnlySource`) source - 48 `seek()` calls total, each independently guarded by a 10s-timeout watchdog thread | Release build, real files (not hand-crafted bytes), public `FormatReader::seek(SeekMode::Accurate, SeekTo::Time{..})` entry point, run via `just test` against a fresh `git clone` of the repo at `base_commit` | All 48 calls returned well within the 10s timeout on every run (3/3 repeat runs, identical output each time - see `output.txt`). Some seekable-mode MKV seeks near the end of the file legitimately return `ok=false` (`SeekErrorKind::OutOfRange`, the requested 6.5s/7.9s timestamp falls past the last cue point in this short test file) - a correct error return, not a hang. No timeout was observed in any (format, source-seekability, track, timestamp) combination. |

Two tools were used (code reading + real end-to-end execution); both agreed, so the soft cap of 3
was not exercised further.

## Status

**Not reproducible.** Neither `symphonia-format-mkv` nor `symphonia-format-isomp4` nor
`symphonia-format-riff` has the peek-without-discard loop shape confirmed for
`symphonia-format-ogg` in candidate 005. `symphonia-format-riff` has no packet-skip loop of any
kind (single-track, direct byte-offset seek). `symphonia-format-isomp4`'s segment-search loop
always marks each examined segment as consumed (`seg_skip = seg_idx + 1`) regardless of outcome,
so it can never re-observe the same segment. `symphonia-format-mkv`'s frame-skip loop uses
`.front()` to peek but unconditionally `pop_front()`s in its `else` arm (covering both "wrong
track" and "right track, timestamp not yet reached"), so it always discards before re-looping.

**Fully reproducible**: N/A (Not reproducible - no reproduction to characterize).

## Class

N/A (Not reproducible).

## Root cause

Not applicable - each of the three implementations was found to already discard the examined
item before looping again, unlike the confirmed Ogg bug. See the "What was tried" table above for
the specific mechanism in each crate that prevents the same shape from occurring.

## Introduced by

N/A (Not reproducible - no bug to attribute).

## The fix

N/A (Not reproducible - no fix needed).

- Project test suite (`cargo test --workspace`, run in this session's worktree, no source changes
  made): **106 passed, 0 failed** (identical before and after, since no fix was applied - this
  candidate made no changes to the main project's source).

## What's not done / open questions

- Only `SeekTo::Time` was exercised for both mkv and isomp4 in the repro; `SeekTo::Timestamp`
  ultimately reaches the same `seek_track_by_ts`/`seek_track_by_ts_forward` functions for both
  crates (confirmed by code reading), so this is not expected to differ, but was not separately
  re-run.
- `symphonia-format-mkv`'s cue-point-guided fast path (`seek_track_by_ts`'s `if let Some(cues)`
  block) was exercised only for the seekable-source case where cues happened to be found before
  the target timestamp; the no-cues/cues-exhausted-before-target fallback straight into
  `seek_track_by_ts_forward` was exercised in both the seekable and non-seekable repro runs.
- `symphonia-format-isomp4`'s fragmented-stream path (`MoofSegment`, moof/moof atoms) was not
  exercised - the test file used is a plain non-fragmented `moov`-based MP4. The same
  `seg_skip`-based forward-progress guarantee in `seek_track_by_ts`'s `'locate: loop` applies
  identically regardless of which `StreamSegment` implementation (`MoovSegment` vs `MoofSegment`)
  is being iterated, since the loop only manipulates `seg_skip`/`self.segs` indices, not anything
  segment-implementation-specific, so this isn't expected to matter, but wasn't separately
  confirmed by running a fragmented-MP4 fixture.
- Only 2-track fixtures were used; a file with 3+ tracks interleaved was not tried, though nothing
  in either loop's forward-progress mechanism is track-count-dependent.
