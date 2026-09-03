# PACKETBUILDER_TRIMMED_DUR_STALE_FIELDS

`symphonia-core/src/packet.rs`'s `PacketBuilder::trimmed_dur` (~line 318-342) computes fresh,
correct local `trim_start`/`trim_end` values from the packet's PTS and the stream's end
timestamp, but then computes the built packet's `dur` field via
`block_dur.saturating_sub(self.trim_start).saturating_sub(self.trim_end)` using the stale,
still-default (`Duration::ZERO`) `self.trim_start`/`self.trim_end` fields left over from the
builder's earlier partial `let Self { track_id, pts, buf, dts, .. } = self;` destructure, instead
of the just-computed local `trim_start`/`trim_end` variables. This candidate was seeded from a
description matching upstream PR #547 (open, unmerged) - root cause was independently confirmed by
reading current `main` before any fix was looked at.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading of `PacketBuilder::trimmed_dur` and its only caller | current `main` @ ee35874, `symphonia-core/src/packet.rs` + `symphonia-bundle-mp3/src/demuxer.rs` | Confirmed by inspection: `self.trim_start`/`self.trim_end` are never assigned between `PacketBuilder::new()` (which sets them to `Duration::ZERO`) and the `let dur = ...` line; the freshly computed `trim_start`/`trim_end` locals are only used for the *returned* builder's fields, not for computing `dur` |
| Real end-to-end repro via `symphonia-bundle-mp3::MpaReader`'s public `FormatReader` API, fed a real MP3 file produced by `ffmpeg`/`libmp3lame` with a LAME/Xing gapless tag (nonzero encoder delay) | debug and release builds, 2 different real MP3 files (different sample rate/channel count/bitrate/duration) | Reproduces deterministically, 3/3 runs on the first file, 1/1 (repeat-invariant, purely deterministic logic bug) on the second - `packet.dur` for the first packet equals the full untrimmed per-frame duration (1152) instead of the correct trimmed value (47 = 1152 - trim_start of 1105) |

No sanitizer/miri/fuzzing was needed or attempted - this is a pure logic bug (stale field read),
not memory-unsafety or a crash, so a debug-assertions build plus a real end-to-end run through the
public demuxer API was sufficient to both find and confirm it deterministically. No 2nd/3rd
escalation tool was needed since the first approach (real end-to-end run) already gave a fully
deterministic, repeatable signal.

## Status

**Confirmed.** Reproduces deterministically, 3/3 runs (release build) against a real MP3 file
(`gapless.mp3`, produced by `ffmpeg -c:a libmp3lame`), and against a second, independently
generated real MP3 file (`gapless2.mp3`, different content/duration/channel count/bitrate) to
satisfy `CONFIRMATION.md` section 7's "vary the input within the same bug class" bar. This is a
pure logic bug with no layout/seed dependency - every run gives the identical result.

**Fully reproducible**: **Yes**. The repro runs the real, public
`symphonia_bundle_mp3::MpaReader::try_new` / `FormatReader::next_packet` entry point end-to-end on
a real, ordinary MP3 file produced by a real encoder (`ffmpeg`/`libmp3lame`) - no internal state or
wire bytes were hand-constructed. This is the "malformed/atypical file handed to a parser's real
public entry point" case `CONFIRMATION.md` section 11 says needs no second participant (there is
no peer/caller being impersonated here - the MP3 file itself, with its real LAME/Xing gapless
metadata, is the genuine external input a real caller would hand to the demuxer).

## Class

logic

## Root cause

`PacketBuilder::trimmed_dur` (`symphonia-core/src/packet.rs`, inside `mod builder`) does:

```rust
pub fn trimmed_dur(
    self,
    block_dur: Duration,
    end_pts: Option<Timestamp>,
) -> PacketBuilder<T, HasPts, HasDur, B> {
    let Self { track_id, pts, buf, dts, .. } = self;
    // ... computes local `trim_start` and `trim_end` correctly from `pts`/`block_dur`/`end_pts` ...
    let dur = block_dur.saturating_sub(self.trim_start).saturating_sub(self.trim_end);
    PacketBuilder { track_id, pts, dur: HasDur(dur), buf, dts, trim_start, trim_end }
}
```

The `let Self { track_id, pts, buf, dts, .. } = self;` destructure only moves out
`track_id`/`pts`/`buf`/`dts`; it leaves `self.dur`/`self.trim_start`/`self.trim_end` un-moved
(they're `Copy` `Duration` values), so `self.trim_start`/`self.trim_end` remain legally accessible
- but their value is whatever `PacketBuilder::new()` initialized them to, `Duration::ZERO`,
since nothing ever calls `.trim_start(...)`/`.trim_end(...)` on the builder before `trimmed_dur`
is invoked in the only real caller (`symphonia-bundle-mp3`'s `MpaReader::next_packet`). The `dur`
calculation therefore always computes `block_dur - 0 - 0 == block_dur`, i.e. the packet's `dur`
field is always the full, untrimmed block duration, silently ignoring whatever amount of
encoder-delay/padding trimming was just computed into the local `trim_start`/`trim_end` variables
(which are still returned correctly as the packet's own `trim_start`/`trim_end` fields - only the
`dur` field is wrong). Concretely: for the confirmed MP3 files, the first packet's `pts=-1105`
(encoder delay), correctly yielding `trim_start=1105`, but `dur` comes out as `1152` (the full
frame's raw sample count) instead of the correct `1152 - 1105 = 47`.

The only caller in this codebase today is `symphonia-bundle-mp3/src/demuxer.rs`'s
`MpaReader::next_packet` (MP3 LAME/Xing gapless trimming), so every MP3 file with nonzero
LAME-tag encoder delay/padding gets a first (and, when padding trimming applies, last) packet
whose reported `.dur` silently includes delay/padding frames that should have been excluded.

## Introduced by

8368547 "core (formats): Introduce a PacketBuilder." (2026-02-08) - this commit added the entire
`trimmed_dur` function (in `symphonia-core/src/formats/mod.rs` at the time; later moved verbatim
to `symphonia-core/src/packet.rs` by ede642b, "core (formats): Move Packet from formats module to
packet module.", a pure file move that carries the same bug forward unchanged). Confirmed via
`git log -S "let dur = block_dur.saturating_sub(self.trim_start)"` and reading `git show
8368547`'s diff of `trimmed_dur`, which already contains the exact buggy line at introduction.

## The fix

Use the freshly computed local `trim_start`/`trim_end` variables instead of the stale
`self.trim_start`/`self.trim_end` fields when computing `dur`:

```diff
-            let dur = block_dur.saturating_sub(self.trim_start).saturating_sub(self.trim_end);
+            let dur = block_dur.saturating_sub(trim_start).saturating_sub(trim_end);
```

This is a 1-line change (see `patch.diff`). Verified:

- Re-ran `repro/` against the patched tree: `packet[0].dur` is now `47` (correctly `block_dur -
  trim_start` = `1152 - 1105`) instead of the buggy `1152`, for both `gapless.mp3` and the
  independently-generated `gapless2.mp3` (per `CONFIRMATION.md` section 7 - checked against more
  than just the one file that first demonstrated the bug).
- Project test suite (`cargo test --workspace --release`, run in the same worktree, immediately
  before and after applying/reverting the 1-line patch): **106 passed, 0 failed, both before and
  after** - no regressions, no new failures.

## What's not done / open questions

- `trim_end` (the padding/end-trim side of the same stale-field bug) was not independently
  exercised with a real file that crosses the stream's known end timestamp mid-packet (this needs
  a file where `info_tag.num_frames` is known and the last decoded packet's PTS+block_dur exceeds
  it) - the fix is the same single line for both fields, and the root cause (never-assigned
  `self.trim_start`/`self.trim_end` on the builder) applies identically to both, so this wasn't
  treated as needing its own separate repro to reach the confirmation bar, but a maintainer wanting
  padding-side evidence specifically would need a different test file (or an artificially truncated
  `num_frames`).
- `PacketBuilder::trimmed_dur` has exactly one real caller in this codebase
  (`symphonia-bundle-mp3`); no other format reader currently uses it, so this bug's real-world
  impact today is scoped to MP3 gapless playback duration reporting specifically.
- Matches upstream PR #547 (https://github.com/pdeljanov/Symphonia/pull/547, open/unmerged at
  time of writing) exactly, including the same one-line fix - this candidate's confirmation and
  fix were derived independently from reading current `main`, not by inspecting that PR's diff
  first.
