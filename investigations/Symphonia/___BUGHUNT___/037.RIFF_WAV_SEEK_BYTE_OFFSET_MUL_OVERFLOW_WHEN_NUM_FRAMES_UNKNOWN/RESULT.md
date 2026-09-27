# RIFF_WAV_SEEK_BYTE_OFFSET_MUL_OVERFLOW_WHEN_NUM_FRAMES_UNKNOWN

`WavReader::seek` (`symphonia-format-riff/src/wave/mod.rs:295-297`) converts the requested frame
timestamp into an absolute byte offset with plain, unchecked `u64` arithmetic
(`self.data_start_pos + (actual_ts.get() as u64 * self.packet_info.block_size.get())`). Flagged by
the `HEURISTICS.md` pattern "arithmetic on offsets derived from input" combined with an `Option`
fallback path with no validation: the only upper bound on the requested timestamp is
`if let Some(num_frames) = track.num_frames` (mod.rs:279-283), and `DataChunk::parse`
(wave/chunks.rs:611-617) deliberately yields `len: None` when the `data` chunk length field is
`u32::MAX`, which is what makes `track.num_frames` stay `None`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the whole WAVE path feeding `seek` | `wave/mod.rs::try_new` -> `chunks.rs::DataChunk::parse`/`FactChunk`/`append_fact_params`, `common.rs::append_data_params`/`PacketInfo`/`get_actual_ts`, `symphonia-core/src/units.rs::TimeBase::calc_timestamp` | Confirmed the `num_frames == None` path is real and has no substitute bound: `append_data_params` (the only place that derives `num_frames` from the data length) is skipped when `data.len` is `None`, and the alternative source (`append_fact_params`, from an optional `fact` chunk) is absent in ffmpeg-produced streams. `get_actual_ts` only aligns the timestamp *down* to a multiple of `max_frames_per_packet` (1152), so a caller-supplied timestamp up to `i64::MAX` reaches the multiplication essentially unchanged, and `block_size` is 1..=65535 |
| Checked which real producer emits `data` length `u32::MAX` | `ffmpeg -f lavfi -i sine=... -c:a pcm_s16le -f wav pipe:1 > file.wav` (ffmpeg 8.0.1), hexdump of the result | ffmpeg writes `0xFFFFFFFF` for **both** the RIFF and the `data` chunk length fields when it cannot know the length ahead of time - exactly the convention symphonia's own comments (mod.rs:89-91, chunks.rs:613-614) describe. So the triggering file needs no hand-crafting at all; a plain piped ffmpeg WAVE is enough |
| Manual boundary-value construction + overflow-checks build | standalone `repro/`, `cargo run --release` with `overflow-checks = true`, 2 real ffmpeg-produced streaming WAVE files x (4 `SeekTo::Timestamp` + 2 `SeekTo::Time` targets), all through `symphonia::default::get_probe().probe()` + `FormatReader::seek` | Reproduced. 5 of the 12 seek calls panic: `attempt to multiply with overflow` at wave/mod.rs:297:35 (16-bit stereo, `block_size` 4, for `ts = i64::MAX`, `ts = 5e18`, `ts = 4_611_686_018_427_388_800`, and `SeekTo::Time { 1.5e14 s }` which `calc_timestamp` turns into 7.2e18) and `attempt to add with overflow` at wave/mod.rs:297:13 (16-bit mono, `block_size` 2, `data_start_pos` 1488, for `ts = i64::MAX`). The two ordinary in-range seeks in each file behave identically to before |
| Same repro with cargo's default release setting | `CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=false cargo run --release`, same inputs | No panic, but the wrapped offset produces a silently wrong seek: for `ts = 4_611_686_018_427_388_800` (stereo file) `actual_ts * 4` wraps to `3584`, so `seek` returns `Ok(actual_ts = 4611686018427388800)` while the stream is really positioned 3584 bytes into the data chunk - the very next packet has `pts = 896`. For the mono file at `ts = i64::MAX` the addition wraps *below* `data_start_pos` (to byte 464, inside the preceding LIST chunk), and the following `next_packet` reports `pts = 9223372036854775296` from a `pos - data_start_pos` that itself wrapped |
| Repeat-run check | 5 consecutive `cargo run --release` runs, same binary, same pristine worktree | 5/5 runs: 5 panics each, exit 1, identical panic sites/messages |
| Same-class sweep of the rest of the crate | grep for `data_start_pos +` / `block_size.get()` / `as u64 *` across `symphonia-format-riff`, `symphonia-format-caf`, `symphonia-format-isomp4`, `symphonia-format-mkv` | Only one twin: `aiff/mod.rs:440-441` is the identical expression, but `SoundChunk::parse`/`SoundChunk::empty` always produce `len: Some(_)` (aiff/chunks.rs:307 and :335), so `append_data_params` always runs for AIFF and `track.num_frames` is always known - the timestamp is therefore bounded there and the same arithmetic cannot overflow today. No other site in these crates computes a seek offset this way |

## Status

**Confirmed**

Reproduces 5/5 runs, deterministically, in a build with overflow checks enabled (the repro's own
`[profile.release] overflow-checks = true`; any debug build behaves the same), with all 5 panics
landing at the same two source locations every run - see `output.txt` (run 1 with a full
backtrace, runs 2-5 as panic-count + exit-status checks). In a build without overflow checks
(cargo's default for `--release`) it does not panic; the arithmetic wraps and `seek` reports
success at a position that has nothing to do with the requested timestamp - stated as a fact about
the two build configurations, not as a reason to discount either (`CONFIRMATION.md` section 2).

**Fully reproducible**: **Yes**. Both input files are produced by really running `ffmpeg ... -f wav
pipe:1 > file.wav` (re-generated by the repro at start-up when `ffmpeg` is on PATH, with the
byte-for-byte outputs of those commands shipped in `repro/assets/` as a fallback), and they are
opened through the ordinary public entry point an application uses -
`symphonia::default::get_probe().probe(&hint, MediaSourceStream::new(File...), ..)` - then seeked
through the public `FormatReader::seek` API. Nothing about the input is hand-authored to "look
like" what a producer would emit: ffmpeg genuinely emits the `0xFFFFFFFF` length fields that put
the reader into the unbounded state. The second half of the trigger is the seek target, which is an
argument the application itself passes to `FormatReader::seek` - not a second participant's output,
so there is nothing to fake there (`CONFIRMATION.md` section 11's single-external-input case).

## Class

crash

(Plus a wrong-position/wrong-`SeekedTo` logic manifestation of the same expression in builds
without overflow checks - see the fourth row of the table above and `output.txt`. Recorded as one
class, `crash`, because it is one arithmetic site with two build-dependent symptoms, not two bugs.)

## Root cause

`WavReader::seek` computes

```rust
let seek_pos =
    self.data_start_pos + (actual_ts.get() as u64 * self.packet_info.block_size.get());
```

with no bound on `actual_ts` and no checked arithmetic. The chain that leaves it unbounded:

1. A WAVE stream whose `data` chunk length field is `u32::MAX` is treated as "length unknown":
   `DataChunk::parse` returns `DataChunk { len: Some(len).filter(|&len| len != u32::MAX) }`, i.e.
   `None` (wave/chunks.rs:611-617). ffmpeg writes exactly this when it streams WAVE to a pipe, so
   the state is not exotic - `symphonia-check` itself is documented to exercise it.
2. With `data.len == None`, `try_new` skips `append_data_params` (mod.rs:163-165) - the only place
   that derives `track.num_frames` from the data length. Absent an optional `fact` chunk (ffmpeg
   does not write one), `track.num_frames` stays `None`.
3. `seek`'s range check is therefore skipped entirely: `if let Some(num_frames) = track.num_frames
   { if required_ts.get() as u64 > num_frames { return seek_error(OutOfRange); } }`
   (mod.rs:279-283) is the *only* upper bound in the function. The lower bound
   (`required_ts.is_negative()`) still applies.
4. `PacketInfo::get_actual_ts` only aligns the timestamp *towards zero* to a multiple of
   `max_frames_per_packet` (1152 for PCM), so `actual_ts` is still up to ~`i64::MAX`.
5. `block_size` is the frame length in bytes for PCM-like formats (4 for 16-bit stereo) and the
   `fmt` chunk's `nBlockAlign` for ADPCM (up to 65535). Any `block_size >= 3` overflows the
   multiplication for a large enough timestamp; `block_size == 2` overflows the *addition* instead
   whenever the data chunk starts more than 1023 bytes into the file (e.g. an ffmpeg stream with a
   large LIST/INFO chunk), because `(i64::MAX aligned down to 1152) * 2 = u64::MAX - 1023`.

So the documented contract ("out-of-range seek returns `SeekErrorKind::OutOfRange`") is only
honored while the frame count happens to be known; when it is not, the same request either panics
(overflow-checks builds) or wraps into a nonsense byte offset that `seek` reports as a success
(release builds). The wrap can also land `seek_pos` *below* `data_start_pos`, after which
`next_packet`'s own `pos - data_start_pos` (common.rs:415) underflows for the same reason - a
downstream consequence that disappears once `seek_pos` is computed with checked arithmetic against
`data_start_pos`.

## Introduced by

5a9bc17 "riff: Fix reading files with an unknown length." (2025-01-16) - the commit that made the
WAVE data-chunk length optional (`DataChunk::len: u32` -> `Option<u32>`, the `u32::MAX` filter, and
the `if let Some(data_len) = data.len { append_data_params(..) }` guard). Before it,
`append_data_params` ran unconditionally for every WAVE file, so `track.num_frames` was always
`Some(_)` and `seek`'s existing `if let Some(num_frames)` check always fired, bounding the
timestamp by the data chunk length (<= `u32::MAX` bytes) and keeping the same multiplication far
inside `u64` range. That commit removed the bound without adding an alternative one to `seek`.
Found with `git log -S "actual_ts.get() as u64 * self.packet_info.block_size"` (single hit,
666b213 "core (formats): Use signed timestamps.", a pure signed-timestamp migration of the same
expression) followed by `git log --follow` on `wave/mod.rs` and reading `git show 5a9bc17` plus
`git show 5a9bc17^:symphonia-format-riff/src/wave/mod.rs` to confirm the pre-change bound. No
bisect was needed.

## The fix

`patch.diff` (1 file, +11/-3, including 3 lines of comment) makes the offset calculation checked
and maps a failure to the error the function already uses for an out-of-range request:

```rust
let seek_pos = (actual_ts.get() as u64)
    .checked_mul(self.packet_info.block_size.get())
    .and_then(|offset| self.data_start_pos.checked_add(offset));

let Some(seek_pos) = seek_pos
else {
    return seek_error(SeekErrorKind::OutOfRange);
};
```

Both halves are covered deliberately, not just the multiplication that the candidate was filed
about: the repro shows the addition overflowing on its own for `block_size == 2`. It is
behaviour-preserving for every input that did not overflow (the value and all downstream uses are
unchanged), and it cannot itself overflow for any input. It does not attempt to also reject offsets
that are merely past the end of the stream - those already surface as the ordinary IO error the
unpatched code produced (`Invalid argument (os error 22)`), and tightening that would change
behaviour for streams whose real length genuinely is unknown.

Checked against more than the one triggering input (`SKILL.md` pitfall #3): two different
`block_size` values (4 and 2) with two different `data_start_pos` values (78 and 1488), four
`SeekTo::Timestamp` targets and two `SeekTo::Time` targets each, plus in-range control seeks in
both files, and both build configurations (overflow-checks on and off). Before: 5 panics, exit 1
(and, with checks off, one seek silently succeeding at the wrong position). After: no panic in
either configuration, every over-range seek returns `seek error: requested seek timestamp is
out-of-range for stream`, both control seeks still return the same `actual_ts` and the same
following packet `pts` as before the patch, exit 0. See `output.txt` and `just test`.

- Project test suite (`cargo test --workspace`, in this candidate's own git worktree): 106 passed
  / 0 failed before the patch -> 106 passed / 0 failed after. No new failures, none newly skipped.

## What's not done / open questions

- No regression test was added to the project's own test suite. `symphonia-format-riff` has no
  test harness that drives `WavReader` from a byte buffer, so a `seek`-level test would need one
  built first; the maintainers may prefer to add it alongside this fix.
- The AIFF twin (`aiff/mod.rs:440-441`) is the identical unchecked expression and was **not**
  patched here, because `SoundChunk` always reports a length so `track.num_frames` is always known
  and the arithmetic is currently bounded (see the last row of the table above). That safety is
  incidental, not enforced - the `TODO` at aiff/chunks.rs:333-334 explicitly contemplates
  supporting the streaming case where the sound chunk length is unknown, which would reintroduce
  exactly this bug there. Applying the same checked arithmetic to both sites would be a reasonable
  maintainer preference; this candidate keeps the patch to the site it actually reproduced.
- Whether `WavReader::seek` should also reject a `seek_pos` past the (known) end of the data chunk,
  rather than letting the underlying `MediaSource` fail, is a design decision left untouched.
- ADPCM streams reach the same line with `block_size` up to 65535 (`nBlockAlign` straight out of
  the `fmt ` chunk), which lowers the timestamp needed to overflow by ~4 orders of magnitude
  compared to the PCM files used here; not separately exercised, since the PCM path already
  reproduces it.
- A fuzzing campaign would not find this on its own: the trigger needs a large caller-supplied
  seek target, which none of the existing `symphonia/fuzz` targets provide (they only decode
  packets). A fuzz target that also issues `seek` calls with fuzzer-chosen timestamps would be a
  cheap addition and would likely find sibling issues in the other demuxers.
