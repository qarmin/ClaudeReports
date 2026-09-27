# ISOMP4_TRUN_TS_SAMPLE_DEFAULT_DUR_DIV_ZERO

`TrunAtom::ts_sample` (`symphonia-format-isomp4/src/atoms/trun.rs:235`) does
`ts_delta.checked_div(u64::from(default_dur)).unwrap_or(0)`, and `default_dur` is threaded in
unvalidated from `TfhdAtom::default_sample_duration`/`TrexAtom::default_sample_duration` - both
raw `u32`s read straight off the wire with no non-zero check anywhere in the read path
(`tfhd.rs:43-46`, `trex.rs:33`). The suspicion (from `IDEAS.md`): a crafted fragmented MP4/CMAF
with `default_sample_duration == 0` and a `trun` lacking per-sample durations would make every
timestamp-to-sample lookup in that run silently resolve to sample 0 via the real `seek()` path,
instead of surfacing an error.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro: `ffmpeg`-produced fragmented AAC-in-MP4 (`frag.mp4`, 9 fragments), binary-patched to zero every `tfhd.default_sample_duration` (`evil.mp4`), driven through the real public `IsoMp4Reader::try_new` + `FormatReader::seek` API | release build, 13 seek timestamps spanning the whole track (0.0s-1.0s), 3/3 repeat runs | `evil.mp4`: **every** seek fails with `Err(SeekError(OutOfRange))`, including t=0.0s. Baseline `frag.mp4` (same file, real non-zero durations): every seek succeeds with a correctly-resolved sample. Deterministic, byte-identical output across all 3 runs. |
| White-box unit test of `TrunAtom::ts_sample` called directly (bypassing the real caller) with `default_dur=0` | debug build (`overflow-checks=true`), temporary `#[cfg(test)]` code in the crate, reverted afterward | Confirms the checked_div(0) mechanism is real in isolation: `ts_sample(0, 0)`, `ts_sample(12345, 0)`, and `ts_sample(u64::MAX, 0)` all return `0` with no panic. |
| White-box test of the real caller, `MoofSegment::ts_sample` (the exact method `IsoMp4Reader::seek_track_by_ts` in demuxer.rs invokes), constructing a `MoofSegment`/`TrafAtom`/`TrunAtom` directly (10-sample run, `default_dur=0`, no per-sample durations) and sweeping `ts` from 0 to 19 | debug build, temporary test, reverted afterward | `Ok(None)` for every one of the 20 timestamps tried - `TrunAtom::ts_sample` (the function with the checked_div) is never actually invoked, because the caller's guard `if ts_accum + trun_dur > ts` requires `trun.total_duration(default_dur) > 0`, which is exactly 0 whenever `default_dur == 0` and no per-sample durations are present. |
| Investigated the only way to make `total_duration()` nonzero anyway despite `default_dur == 0` (the "first sample has an explicit duration" override, gated on `is_first_sample_duration_present()`) | debug build, temporary test, reverted afterward, 3/3 repeat runs | Panics unconditionally with "index out of bounds: the len is 0 but the index is 0" on `self.sample_duration[0]` (`trun.rs:92`) - `sample_duration` is only ever populated when the trun's own `SAMPLE_DURATION_PRESENT` flag is set, which is a separate, mutually-exclusive branch from this one. This panic fires regardless of `default_dur`'s value, so it can't be used as a path to reach the checked_div(0) branch with a nonzero `ts_delta` either - and it's an unrelated, likely separate, bug (see "What's not done" below). |

## Status

**Not reproducible.** The specific mechanism described (`checked_div(0).unwrap_or(0)` silently
resolving a timestamp to sample 0 via the real `seek()` path) does not occur: the isolated
function-level behavior is real (confirmed above), but the only real call site
(`MoofSegment::ts_sample` in `stream.rs`, called from `IsoMp4Reader::seek_track_by_ts` in
`demuxer.rs`) structurally guards against ever invoking it with a meaningful (nonzero) `ts_delta`
when `default_dur == 0`, because it never selects a `trun` whose `total_duration()` is 0 in the
first place (strict `>` comparison). The one path that could make `total_duration()` nonzero
despite `default_dur == 0` hits an unconditional, unrelated out-of-bounds panic first (a distinct
bug from this candidate). Verified via a real end-to-end file (a genuinely-produced fragmented
MP4, binary-patched to the exact crafted condition described) through the real public
`IsoMp4Reader`/`FormatReader::seek` API: 3/3 identical runs, every seek attempt across the whole
track's duration returns an honest `SeekError(OutOfRange)`, never a wrong-but-successful sample
resolution.

**Fully reproducible**: N/A (not applicable to a Not-reproducible result - no bug to reproduce).

## Class

N/A (Not reproducible).

## Root cause

Not applicable to the specific bug suspected here - the suspected code path turns out to already
be structurally unreachable with an interesting (nonzero) input in the way the candidate describes.
`TrunAtom::ts_sample`'s `checked_div(0).unwrap_or(0)` is real, dead-weight code (it never runs with
a `ts_delta` that would actually matter through any real caller in this crate - confirmed to be its
only call site), because its sole caller, `MoofSegment::ts_sample`, only ever calls it after first
confirming (via `trun.total_duration(default_dur) > 0`, checked one call up) that the run's total
duration is nonzero - which is false by construction whenever `default_dur == 0` and no per-sample
durations are present.

## Introduced by

N/A (Not reproducible - no bug to attribute).

## The fix

Not applicable - no bug found, no patch produced.

## What's not done / open questions

- **A new, distinct suspect was noticed while investigating this one** (recorded in `IDEAS.md`):
  `TrunAtom::is_first_sample_duration_present()` (`trun.rs:51`) tests
  `first_sample_flags & TrunAtom::FIRST_SAMPLE_FLAGS_PRESENT` (bit `0x4`) - the same constant used
  to decide whether `first_sample_flags` is present at all - instead of
  `TrunAtom::SAMPLE_DURATION_PRESENT` (bit `0x100`), which is what the analogous
  `is_first_sample_size_present()` correctly checks against (`SAMPLE_SIZE_PRESENT`, `0x200`) for
  the parallel "first sample size" case. Whenever a `trun`'s `first_sample_flags` value (an
  ordinary ISO/IEC 14496-12 sample-flags bitfield: is-leading/depends-on/is-depended-on/
  has-redundancy/is-non-sync/degradation-priority) happens to have bit `0x4` set - plausible for
  real encoder output, since that bit falls inside the `sample_depends_on` sub-field - and the
  trun's own `SAMPLE_DURATION_PRESENT` flag is *not* set, `total_duration()`/`sample_timing()`/
  `ts_sample()` all index `self.sample_duration[0]` on a vector that is guaranteed empty in that
  branch, causing an unconditional out-of-bounds panic - independent of `default_dur`, so
  reachable with an ordinary nonzero default duration too, not just the zero case investigated
  here. This looks like a copy-paste/constant-reuse bug distinct from the div-by-zero one this
  candidate was about, and was not chased further this session (out of scope) - it deserves its
  own candidate.
- `TrunAtom::sample_timing()` (used for normal, non-seek packet iteration via
  `MoofSegment::sample_timing`) is *not* guarded the same way `ts_sample()`'s caller is - it's
  called whenever `sample_num_rel < trun.sample_count`, regardless of `total_duration()`. With
  `default_dur == 0` (and no per-sample durations, no first-sample override), this multiplies
  `sample_num_rel * default_dur` (not a division, so no panic risk), giving every sample in the run
  timestamp `0` and duration `0`. This is a real, plausible-if-minor logic bug (timestamps/duration
  collapse for the whole run during ordinary decode, not just seeking) - a different code path and
  a different observable effect from the div-by-zero hazard this candidate specifically checked, so
  not confirmed/chased further here; noted in `IDEAS.md` as a related but distinct suspect.
- Only the fragmented-MP4 (`moof`/`traf`/`trun`) path was checked. The non-fragmented `stts`-based
  timing path (`SttsAtom::find_sample_for_timestamp`) has its own, already-confirmed, unrelated
  overflow bug (candidate 015) - not re-examined here.
