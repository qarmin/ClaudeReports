# FUZZ_AUDIO_DECODER_ZERO_BLOCK_DUR

`symphonia/fuzz/src/lib.rs::fuzz_audio_decoder` - the shared helper used by the `decode_flac`,
`decode_mpa`, `decode_adpcm`, `decode_pcm`, `decode_alac`, `decode_aac`, and `decode_vorbis` fuzz
targets - always constructs `Packet::new(0, Timestamp::ZERO, Duration::ZERO, packet_data)`.
`Packet::block_dur()` returns `dur + trim_start + trim_end`, all zero here, so any decoder that
derives its per-packet work amount from `block_dur()` never actually runs its block-decoding code,
no matter what `packet_data` the fuzzer generates. This looked worth checking after candidate
002's `RESULT.md` noted it in passing while investigating a different, already-confirmed ADPCM bug.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `Packet::block_dur()` (`symphonia-core/src/packet.rs`) and grepped every codec crate reachable from the fuzz targets (`symphonia-codec-adpcm`, `symphonia-codec-pcm`, `symphonia-bundle-flac`, `symphonia-codec-alac`, `symphonia-codec-aac`, `symphonia-codec-vorbis`, `symphonia-bundle-mp3`) for `.block_dur()` / `packet.dur` usage inside their `decode`/`decode_ref`/`decode_inner` paths | manual, static read | Only `symphonia-codec-adpcm/src/lib.rs:130` (`block_count = packet.block_dur().get() as usize / frames_per_block`) uses it. PCM derives frame count from `packet.data.len()`; FLAC/ALAC/AAC/Vorbis/MP3 all parse their own frame/block length out of the bitstream itself (`packet.data`/`packet.as_buf_reader()`), never touching `packet.dur`/`block_dur()`. So the bug as stated is real, but its silent-coverage-loss effect is currently confined to the ADPCM fuzz target, not FLAC/PCM/ALAC/AAC/Vorbis/MP3. |
| Standalone repro calling the real, unmodified `symphonia_fuzz::fuzz_audio_decoder::<D>` (via a thin observing wrapper decoder around the real, public `AdpcmDecoder`) with the exact `AudioCodecParameters` `decode_adpcm.rs` uses (ADPCM MS, frames_per_block=1024, max_frames_per_packet=1024) and a deliberately truncated 3-byte `packet_data` | release build, 3 runs | Deterministic 3/3: `block-decode loop entered = false, output frames = 0, decode err = None` - `decode_mono`/`decode_stereo` never ran, and the 3 garbage bytes were never even read, for any `packet_data`. |

Two approaches were enough here: the first (static read across every affected decoder crate)
already gave a definitive, code-level answer to the "does this affect FLAC/PCM/ALAC/AAC/Vorbis/MP3
too" question the candidate description asked about, and the second produced a deterministic,
real-entry-point repro on the first try with no partial/ambiguous signal - no third tool was
needed.

## Status

**Confirmed**, 3/3 runs, release build, deterministic (both the "before" reproduction and the
"after" fix-verification runs).

**Fully reproducible**: **Yes**. `repro/main.rs` calls the real, unmodified
`symphonia_fuzz::fuzz_audio_decoder::<D>` function (the exact same function every `decode_*` fuzz
target in this repo calls, unmodified), with the exact `AudioCodecParameters` setup
`decode_adpcm.rs` uses. The only repro-local code is a thin observer decoder wrapper
(`ObservingAdpcmDecoder`) that forwards every trait method unchanged to the real, public
`AdpcmDecoder` and records the outcome via process-wide statics (needed because
`fuzz_audio_decoder` owns the decoder internally and never returns it, so there's no other way for
the calling process to observe what happened inside). There is no second participant to fake here -
this is a single caller driving a real public API with a real artifact (packet bytes), the case
CONFIRMATION.md section 11 says stays a plain single-process repro.

## Class

logic (test-coverage gap in the shared fuzz harness, not a runtime-reachable decoder bug by
itself).

## Root cause

`AdpcmDecoder::decode_inner` (`symphonia-codec-adpcm/src/lib.rs:130`) computes
`block_count = packet.block_dur().get() as usize / frames_per_block`, then loops
`for block_id in 0..block_count` to call `decode_mono`/`decode_stereo`. `fuzz_audio_decoder`
(`symphonia/fuzz/src/lib.rs`) always builds `Packet::new(0, Timestamp::ZERO, Duration::ZERO,
packet_data)`. `Packet::block_dur()` (`symphonia-core/src/packet.rs`) is
`dur.saturating_add(trim_start).saturating_add(trim_end)`, all three fields zero on every packet
this harness ever constructs, so `block_count` is always `0` regardless of `packet_data`'s content
or length. The `0..0` loop never runs, so `codec_ms`/`codec_ima_wav`/`codec_ima_qt`'s
`decode_mono`/`decode_stereo` - the actual bitstream-parsing logic these fuzz targets exist to
exercise - are unreachable from `decode_adpcm`'s fuzz corpus no matter what the fuzzer generates.
`decode_ref` returns `Ok` with a 0-frame buffer in this case, so libFuzzer sees a normal,
uninteresting return on every single input - there is no crash, no distinguishing coverage signal,
nothing that would make a coverage-guided fuzzer treat "produce a packet_data that reaches
`decode_mono`" as interesting, because from the fuzzer's point of view that code is simply never
instrumented-reachable through this target.

Every other decoder reachable through this same helper (PCM, FLAC, ALAC, AAC, Vorbis, MP3) derives
its own frame/block boundaries from `packet.data` itself (their own bitstream/header parsing) and
never reads `packet.dur`/`block_dur()`, so they are unaffected by this specific harness defect -
see "What was tried" above for the exact call sites checked.

## Introduced by

93b20ff "fuzz: Expand fuzzing coverage and fix FLAC & ALAC OOM/panic bugs. (#442)" (2026-02-12) -
this commit added `symphonia/fuzz/src/lib.rs::fuzz_audio_decoder` (and the `decode_adpcm`/
`decode_pcm`/`decode_flac`/`decode_alac`/`decode_aac`/`decode_mpa`/`decode_vorbis` fuzz targets
that call it) from scratch, already using `Duration::ZERO` in the `Packet::new(...)` call
(confirmed via `git log -p --follow` on the file).

## The fix

Changed `fuzz_audio_decoder` to compute a plausible non-zero packet duration instead of always
using `Duration::ZERO`: `let dur = params.max_frames_per_packet.map(Duration::new).unwrap_or(Duration::ZERO);`.
`max_frames_per_packet` is a field every fuzz target already sets (or leaves `None`, in which case
behavior is unchanged from before - no regression for codecs that don't declare it, e.g.
`decode_mpa`/`decode_flac` today). For `decode_adpcm` specifically, this makes `dur = 1024 ==
frames_per_block`, so `block_count = 1024 / 1024 = 1`: exactly one real block is decoded per fuzz
iteration, which is what actually exercises `decode_mono`/`decode_stereo` going forward. This is a
minimal, bounded change confined to the shared test harness (`symphonia/fuzz/src/lib.rs`) - it does
not touch any decoder or `symphonia-core` production code.

Verified against more than the exact triggering input (CONFIRMATION.md section 7): re-ran the
patched harness with a 50-byte `packet_data` (`vec![0x11; 50]`) instead of the original 3-byte
input - same result, the block-decode loop is entered and fails trying to read the (still
insufficient) bytes, confirming the fix isn't narrow to one specific input length.

- Re-running `repro/` against the patched tree: `block-decode loop entered = true, output frames =
  0, decode err = Some("unsupported feature: adpcm: block predictor exceeds range")` - i.e. the
  real block-decode loop now genuinely runs (and, on this deliberately truncated/garbage input,
  correctly rejects it) instead of silently skipping it - 3/3 runs, release build, deterministic.
- `symphonia/fuzz` is its own separate Cargo workspace (`[workspace] members = ["."]`), not a
  member of the main Symphonia workspace, so it is never built or tested by the main project's own
  `cargo test --workspace`. Checked both ways for completeness: `cargo check` inside
  `symphonia/fuzz` succeeds both before and after the patch (`Finished dev profile` both times, no
  warnings/errors introduced). Main-workspace `cargo test --workspace`: **106 passed, 0 failed**
  before the patch, **106 passed, 0 failed** after (unaffected either way, since the patch never
  touches main-workspace code - included per the candidate loop's mandatory before/after step for
  completeness, not because a regression there was plausible).

## What's not done / open questions

- This fix only changes the shared `fuzz_audio_decoder` helper, not `decode_flac`/`decode_alac`/
  etc. individually - those targets already don't rely on `block_dur()` today (see "Root cause"),
  so nothing else needed changing this session, but a *future* decoder crate added to this helper
  that does derive work from `block_dur()` would now get a plausible non-zero duration by default,
  which is the intended defense-in-depth effect of fixing the shared helper rather than only
  patching `decode_adpcm.rs`'s own fuzz target.
- Whether a longer, real `cargo-fuzz` campaign against the patched `decode_adpcm` target (run
  separately, via `fuzz-bug-repro`, not by this skill) would now find new crashes inside
  `codec_ms`/`codec_ima_wav`/`codec_ima_qt`'s block-decoding logic - previously unreachable from
  this fuzz target - is untested this session; only the reachability gap itself was confirmed and
  fixed, not run as an actual fuzzing campaign.
- `decode_alac.rs`'s second `fuzz_audio_decoder` call site passes `vec![]` as `packet_data` (empty
  packet) - not investigated further here since ALAC doesn't use `block_dur()` at all (see "Root
  cause"), so it's unaffected by this specific bug regardless.
