# CAF -> ADPCM IMA QT decoder: index-out-of-bounds panic via attacker-controlled `frames_per_packet`

Follow-on from candidate 002: after finding that `symphonia-codec-adpcm`'s inner codecs assume a
buffer length matching their own idea of "frames per block" without validating it, checked which
of the three inner ADPCM variants (MS, IMA WAV, IMA QT) are actually reachable with an
attacker-chosen `frames_per_block` through a demuxer that ships in this repo. `codec_ima_qt.rs`
hardcodes 64 frames/block (`for byte in 0..32 { buffer[byte*2]=...; buffer[byte*2+1]=...; }`,
ignoring the `frames_per_block` argument entirely - it's `_` in the function signature), while
`symphonia-format-caf`'s demuxer passes an entirely attacker-controlled 32-bit
`frames_per_packet` field from the CAF file's `desc` chunk straight through as
`with_frames_per_block(...)` for the `ima4` codec (`CODEC_ID_ADPCM_IMA_QT`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `symphonia-codec-adpcm/src/codec_ima_qt.rs` | manual | `decode_mono` ignores its `frames_per_block` parameter and always writes 64 samples (indices `0..=63`) |
| Read `symphonia-format-caf/src/demuxer.rs::read_audio_description_chunk` | manual | For any `desc.frames_per_packet` other than `0` or `1`, `with_frames_per_block(u64::from(desc.frames_per_packet))` is set directly from the file's own header field, with no validation against the codec's real block size |
| Standalone repro calling the decoder directly (`AdpcmDecoder::try_registry_new` + `decode`, `frames_per_block=2`) | release build, 1 run | Panics: `index out of bounds: the len is 2 but the index is 2` at `codec_ima_qt.rs:33` |
| Full end-to-end repro: a 102-byte crafted `.caf` file through the real `symphonia` crate's public `probe -> demux (CafReader) -> decode (registered ADPCM IMA QT decoder)` pipeline | release build, 3 runs | Reliably panics every run, same message, same location |

## Status

**Confirmed**, 3/3 runs, release build, deterministic (no randomness involved - same 102-byte
crafted file every time).

**Fully reproducible**: **Yes**. `repro/main.rs` builds a minimal, well-formed-except-for-one-field
CAF file in-process and feeds it through `symphonia::default::get_probe().probe(...)` ->
`format.next_packet()` -> `decoder.decode(...)` - the exact same public API sequence a real
application using this crate (including `symphonia-play`, this workspace's own example player)
would use to play an untrusted `.caf` file. The only "attacker-controlled" field is the `desc`
chunk's `frames_per_packet`, which is a genuine, spec-legal-looking field position in a real CAF
file; nothing about the repro stands in for a participant that wouldn't really exist - the
"second participant" here is simply the untrusted file, produced and parsed exactly as a real one
would be.

## Class

crash (index-out-of-bounds panic on a maliciously crafted, otherwise well-formed container file).

## Root cause

Two independent gaps compound:

1. `symphonia-format-caf`'s demuxer (`read_audio_description_chunk`) sets
   `AudioCodecParameters::frames_per_block` directly from the CAF file's own `frames_per_packet`
   header field for compressed formats (anything other than the `frames_per_packet == 1`
   "uncompressed" special case), with no cross-check against what the chosen codec (`ima4` here)
   actually requires.
2. `symphonia-codec-adpcm`'s `codec_ima_qt::decode_mono` doesn't use the `frames_per_block`
   parameter it's given at all - it hardcodes the assumption that every IMA4 block is exactly 64
   frames (from the format's real, fixed 34-byte block structure) and writes indices `0..=63` into
   a buffer whose actual length is `frames_per_block` (here, 2, from `AdpcmDecoder::decode_inner`'s
   `buffer_range = offset..(offset + frames_per_block)`).

Either gap alone would be relatively benign; together, a single 32-bit field in a file header
reaches all the way into an unconditional out-of-bounds write two crate boundaries away.

## Introduced by

Two independent commits, matching the two-part root cause:
- Decoder side (`codec_ima_qt.rs::decode_mono` ignoring `frames_per_block`, hardcoding 64): d35eeb2 "adpcm: Support IMA QuickTime (#384)" (2025-08-20).
- Demuxer side (`symphonia-format-caf` passing `desc.frames_per_packet` straight through to `with_frames_per_block` for the compressed case with no cross-check): e8e986f "caf: Properly support variable and fixed sized packets. (#385)" (2025-10-01) - confirmed via `git show`, which switched this from a hardcoded `1` to the untrusted field.

## The fix

Added a `buffer.len() < 64` check at the top of `codec_ima_qt::decode_mono`
(`symphonia-codec-adpcm/src/codec_ima_qt.rs`), returning a `decode_error` instead of indexing past
the buffer's real length; `decode_stereo` delegates to `decode_mono` for each channel buffer, so
this single check covers both. (`patch.diff` also includes the MS ADPCM fix from candidate 002,
fixed in the same session; this candidate's own `repro/` only exercises the IMA QT / CAF path.)

- Re-running `repro/` against the patched tree, full end-to-end through `CafReader` and the real
  decoder registry: `decode error (no crash): malformed stream: adpcm (ima qt): frames_per_block
  must be at least 64`, 3/3 runs.
- Project test suite (`cargo test -p symphonia-codec-adpcm -p symphonia-format-caf -p symphonia`):
  `symphonia-format-caf` 2 passed / 0 failed (unrelated chunk-parsing tests, unaffected);
  `symphonia-codec-adpcm` and `symphonia` have 0 unit tests in this workspace; identical results
  before and after the fix.

## What's not done / open questions

- The demuxer-side gap (CAF trusting `frames_per_packet` without validating it against the
  chosen codec) was left alone in favor of fixing the decoder side, since the decoder is the
  component whose own documented contract is violated (its `frames_per_block` parameter exists
  specifically to tell it the block layout, and it silently ignores it) and the fix there also
  protects any other current or future caller of `AdpcmDecoder`, not just this one demuxer. A
  maintainer may also want to make `symphonia-format-caf` reject/clamp an implausible
  `frames_per_packet` for known fixed-block codecs as defense in depth.
- Only `AppleIMA4`/`CODEC_ID_ADPCM_IMA_QT` was checked for this exact CAF-side gap; other codecs
  CAF can select (AAC, ALAC, FLAC, Opus, MPEG layers) were not re-audited for the same
  "container-supplied `frames_per_packet` trusted without cross-checking the codec's real
  requirements" pattern this session.
