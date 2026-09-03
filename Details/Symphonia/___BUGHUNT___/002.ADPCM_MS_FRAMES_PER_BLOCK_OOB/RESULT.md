# MS ADPCM decoder: index-out-of-bounds panic when `frames_per_block == 1`

While reading `symphonia-codec-adpcm` (looking for the classic "container-declared block/step
size used to index a fixed-size table/buffer without validation" pattern - the existing
`decode_adpcm` fuzz target hardcodes `frames_per_block(1024)` and `Packet::dur =
Duration::ZERO` via the shared `symphonia_fuzz::fuzz_audio_decoder` helper, so it never explores
small `frames_per_block` values, and `Duration::ZERO` actually makes `AdpcmDecoder::decode_inner`'s
`block_count` always 0 - meaning the fuzz harness never calls into `codec_ms`/`codec_ima_wav`/
`codec_ima_qt`'s block-decoding functions *at all*, regardless of packet content), found that
`AdpcmDecoder::try_new` only rejects `frames_per_block == 0`, not the actual minimum each inner
codec needs.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `symphonia-codec-adpcm/src/{lib,codec_ms,codec_ima_wav,codec_ima_qt}.rs` for buffer-length assumptions vs. validated minimums | manual | `codec_ms.rs`'s `decode_mono`/`decode_stereo` unconditionally write `buffer[1]` (and `buffers[0][1]`/`buffers[1][1]`), but the only validation in `AdpcmDecoder::try_new` is `frames_per_block != 0` |
| Standalone repro via the real public decoder API (`try_registry_new` + `decode`) | release build, `frames_per_block=1`, 3 runs | Reliably panics every run: `index out of bounds: the len is 1 but the index is 1` at `codec_ms.rs:126` |
| Checked whether any demuxer currently in this repo can actually produce `frames_per_block == 1` for `CODEC_ID_ADPCM_MS` | read `symphonia-format-riff/src/common.rs::FormatAdpcm::make_packet_info` | **No** - the WAV/AIFF (RIFF) demuxer is the only place in this repo that constructs ADPCM_MS codec params, and its formula `frames_per_block = (8*(block_align - 7*num_channels))/bits_per_frame + 2` always yields >= 2 (the `+2` is unconditional). No other format reader in this repo maps to `CODEC_ID_ADPCM_MS`. |

## Status

**Confirmed**, 3/3 runs, release build, deterministic.

**Fully reproducible**: **Yes**, in the sense that `repro/main.rs` drives the real, public
`AdpcmDecoder::try_registry_new` + `AudioDecoder::decode` entry point - the same API
`symphonia::default`'s codec registry and every format reader in this workspace uses, and the
decoder's own validation logic explicitly implies any nonzero `frames_per_block` should be safe
("frames_per_block is required" / rejects only `0`). **However**, unlike candidate 003 (the CAF /
IMA-QT sibling bug), no demuxer currently shipped in this repository can actually produce
`frames_per_block == 1` for `CODEC_ID_ADPCM_MS` specifically - see the third row above. This is
therefore a genuine defect in the decoder crate's own input validation (reachable by any current
or future caller of its public API, including a hypothetically different/future demuxer, or an
application embedding symphonia that constructs `AudioCodecParameters` from its own file parsing),
but not, today, reachable by handing a malicious file to any format reader that ships in this
repo. Recorded honestly rather than overclaiming file-level reachability.

## Class

crash (index-out-of-bounds panic).

## Root cause

`codec_ms.rs::decode_mono` writes `buffer[0]` and `buffer[1]` unconditionally (the 2 uncompressed
preamble samples every MS ADPCM block starts with), and `decode_stereo` likewise writes index 0
and 1 of both channel buffers unconditionally. The buffer passed in has length exactly
`frames_per_block` (see `AdpcmDecoder::decode_inner`'s `buffer_range = offset..(offset +
frames_per_block)`). `AdpcmDecoder::try_new` validates only `frames_per_block != 0`, not `>= 2`,
so a caller-supplied `frames_per_block == 1` passes construction and panics on the first block
decoded.

## Introduced by

4c508b2 "adpcm: Add ADPCM Decoder (#160)" (2022-11-08) - `codec_ms.rs`'s `decode_mono`/`decode_stereo` writing `buffer[1]` unconditionally, and `try_new` validating only `frames_per_block != 0`, were both present as-written in this commit, which added the ADPCM decoder crate from scratch (confirmed via `git blame` on the affected lines).

## The fix

Added an explicit `frames_per_block < 2` check at the top of both `decode_mono` and
`decode_stereo` in `symphonia-codec-adpcm/src/codec_ms.rs`, returning a `decode_error` instead of
panicking. (`patch.diff` also includes the analogous fix for `codec_ima_qt.rs` - see candidate 003
- since both were found and fixed in the same session; this candidate's own `repro/` only
exercises the MS ADPCM path.)

- Re-running `repro/` against the patched tree: `decode error (no crash): malformed stream: adpcm
  (ms): frames_per_block must be at least 2`, 3/3 runs.
- Project test suite (`cargo test -p symphonia-codec-adpcm`): 0 tests exist in this crate, both
  before and after (N/A - no regression risk from the fix beyond what compiling verifies).

## What's not done / open questions

- `codec_ima_wav.rs` was also checked by hand for the same pattern: its `decode_mono`/
  `decode_stereo` already derive their loop bounds from `frames_per_block` itself (rather than a
  separate hardcoded constant) and were confirmed *not* panicking at `frames_per_block == 1` via a
  quick manual check, so no fix was needed there.
- Whether the project maintainers consider a decoder crate responsible for defending against
  "impossible via any current demuxer" caller-supplied parameters is a judgment call left to them;
  the fix is applied here on the principle that a public library API's own stated validation
  ("frames_per_block is required") should match what it actually needs to not panic.
