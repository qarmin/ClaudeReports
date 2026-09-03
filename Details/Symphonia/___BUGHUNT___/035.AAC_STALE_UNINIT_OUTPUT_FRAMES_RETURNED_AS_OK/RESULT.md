# AAC_STALE_UNINIT_OUTPUT_FRAMES_RETURNED_AS_OK

`symphonia-codec-aac/src/aac/mod.rs:233-234` (`AacDecoder::decode_inner`) does
`self.buf.clear(); self.buf.render_uninit(None);`, which commits the output buffer's full
1024-frame capacity without writing anything to it (`AudioBuffer::render_uninit` only advances the
frame counter - `symphonia-core/src/audio/buf.rs:389-400`). Only the channels covered by an
actually-decoded SCE/CPE element get written, by `ChannelPair::synth_audio`. Suspected via
HEURISTICS.md's "error swallowing / fallback value used as if it were valid data" pattern
(uninitialized/stale buffer treated as decoded output).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the whole reachability chain: `decode_ga`'s element loop and its `while bs.bits_left() > 3` guard, `set_pair`'s `validate!(channel < max_channels)`, `synth_audio`'s per-plane writes, `dsp::Dsp::synth` (confirmed it writes all 1024 samples of `dst`, so a *fully* decoded packet does overwrite every plane), `AudioBuffer::{new,clear,render_uninit,render_silence}`, and `AdtsReader::{read_body,next_packet}` (confirmed `frame_len == header_len` is accepted, so a zero-length AAC payload is demuxable) | Read source only, no build | Identified three concrete reachable input shapes that leave planes/frames un-overwritten while `decode_ref` still returns `Ok`, and one constraint the candidate description missed: a *lone SCE* only reaches the bug if the previous packet also used SCE elements, because `set_pair` rejects a pair-shape change (`validate!(self.pairs[pair_no].is_pair == pair)`). A packet of two SCEs followed by a packet of one SCE gets past that |
| Standalone repro (`repro/`) crafting real ADTS files bit by bit and decoding them through the real public `AdtsReader` -> `AacDecoder` path; non-silent audio is produced without hand-writing Huffman spectral data by using a single PNS (`NOISE_HCB`) scalefactor band, whose coefficients the decoder's own LCG generates | `cargo run` (dev profile, `debug-assertions = true`, `overflow-checks = true`), 3 runs before and 3 runs after the patch, 8000 Hz / AAC-LC | Reproduced 3/3 before, gone 3/3 after - see `output.txt`. All three malformed shapes return `Ok` with the previous packet's samples, bit-identical; the three well-formed control streams (mono SCE, stereo CPE, stereo SCE+SCE, 3 frames each) decode every frame both before and after |

Both approaches agreed on the first attempt, so the soft cap of 3 escalating approaches was not
needed - no fuzzing/sanitizer/miri run was performed for this candidate (and per the workspace
`README.md`, miri/ASAN have low value on this pure-safe-Rust decode path: nothing here is UB, the
buffer is zero-filled by `AudioBuffer::new`, so a sanitizer has nothing to flag).

## Status

**Confirmed.** Reproduces 3/3 runs, dev build with debug assertions and overflow checks on, from a
pristine clone of ee35874 - deterministic (the crafted bitstreams are fixed, and the decoder's PNS
LCG is seeded with a constant, so the printed sample peaks are bit-for-bit identical run to run).
Three distinct malformed frame shapes reproduce it:

1. `raw_data_block` whose first element is `ID_TERM` (a single `0xe0` payload byte): `id == 7`
   breaks the loop immediately, `cur_pair == 0`, the synth loop does nothing, and 1024 frames of
   the *previous* packet's audio are returned as `Ok`.
2. ADTS frame with `frame_len == header_len`, i.e. a zero-length AAC payload: `bs.bits_left()` is
   0, so the element loop never runs at all - same outcome.
3. Stereo stream (channel configuration 2) where a packet of two SCEs is followed by a packet
   containing only one SCE: channel 0 is decoded normally, plane 1 is never written and is
   returned still holding the previous packet's channel-1 samples.

In case 1 and 2 the whole returned buffer is bit-identical to the previous packet's buffer, so a
stream of such frames replays the same 1024 samples forever rather than erroring or going silent.
The first packet of a stream is unaffected: `AudioBuffer::new` fills the planes with `S::MID`
(0.0 for `f32`), so before any successful decode the stale content is silence, not uninitialized
memory - there is no UB or memory-safety issue here, only wrong audio.

**Fully reproducible**: **Yes**. `repro/` builds the ADTS bytes itself and feeds them to the real
public `AdtsReader::try_probe_new` -> `next_packet` -> `AacDecoder::decode` path, i.e. the same
demuxer+decoder pipeline `symphonia-play` uses; the malformed file *is* the genuine external input
(a single-participant, single-artifact case per CONFIRMATION.md section 11 - there is no second
participant whose behaviour is being stood in for).

## Class

logic

## Root cause

`decode_inner` establishes 1024 output frames up front with `render_uninit`, which is a promise
that the code below will overwrite every sample of every plane. `decode_ga` only keeps that promise
for the channels it actually decoded an element for: it counts elements into `cur_pair`/`cur_ch`
and then calls `synth_audio` exactly `cur_pair` times, and each call writes only
`plane(channel)` (plus `plane(channel + 1)` for a CPE). Nothing checks that the elements present in
the `raw_data_block` cover all channels of the channel configuration, and nothing ever fills the
uncovered planes. Since `AudioBuffer::clear()` merely resets the frame counter and leaves the plane
contents intact, an uncovered plane still holds whatever the last successful `decode` wrote there,
and `decode_ref` returns that buffer as `Ok`. The caller has no way to tell: `frames()` is 1024 and
the buffer looks fully decoded, which also breaks `AudioDecoder::last_decoded`'s documented
contract ("will contain the audio content of the last decoded `Packet`").

This is not ADTS-specific - any container that carries AAC packets (`symphonia-format-isomp4`,
`symphonia-format-mkv`) reaches the same code with the same packet payloads.

## Introduced by

3aeeb22 "aac: Port the NihAV AAC decoder to Symphonia." (2020-08-18) - the initial port already had
`self.buf.clear(); self.buf.render_reserved(None);` in `decode()` followed by
`for pair in 0..cur_pair { ... synth_audio(...) }` with no channel-coverage check, and
`render_reserved` at that commit had exactly today's `render_uninit` semantics (only
`self.n_frames += n_reserved_frames`). f1722b6 "core (audio): New audio module." is only the commit
that renamed `render_reserved` to `render_uninit`, not the one that introduced the bug.

## The fix

`patch.diff` adds, at the end of `decode_ga` and before the synth loop, a check that the elements
decoded from this `raw_data_block` covered every channel of the channel configuration:

```rust
let num_channels = self.asc.channels.as_ref().map_or(0, |channels| channels.count());
validate!(cur_ch == num_channels);
```

`validate!` returns a `DecodeError`, which `decode_inner` propagates to `decode_ref`, which already
does `self.buf.clear()` on the error path - so a malformed packet now yields an `Err` and a
zero-length buffer instead of stale audio, and the `last_decoded` contract is honoured. `cur_ch` can
never exceed `num_channels` on a path that reaches this point (`set_pair`'s
`validate!(if pair { channel + 1 } else { channel } < max_channels)` already rejects an element for
an out-of-range channel), so the equality check only rejects *under*-coverage.

Checked against more than the one triggering input (CONFIRMATION.md section 7): `repro/` also runs
three well-formed control streams of 3 frames each - mono (SCE), stereo (CPE without a common
window), and stereo as two SCEs - and asserts every frame still decodes with the expected plane
count. All three pass before and after the patch. `cargo +nightly fmt --check -p symphonia-codec-aac`
is clean with the patch applied (stable `cargo fmt` cannot be used in this repo - `rustfmt.toml`
uses nightly-only options).

Deliberately *not* chosen: swapping `render_uninit` for `render_silence`. That would replace the
stale audio with silence and keep returning `Ok`, which hides a malformed stream instead of
reporting it, and costs a 4-8 KiB memset per packet on the normal path. It is the more permissive
option if a maintainer would rather not reject a real-world stream that signals more channels than
its `raw_data_block` carries (libavcodec's AAC decoder is lenient about that specific mismatch and
reconfigures its output layout instead - Symphonia cannot, since `AacDecoder`'s output buffer and
channel layout are fixed in `try_new`).

- Project test suite (`cargo test --workspace`, whole repo): 106 passed / 0 failed before ->
  106 passed / 0 failed after. No new failures.

## What's not done / open questions

- **The same bug class exists in `symphonia-bundle-flac`** and was not investigated here (one bug
  per candidate): `symphonia-bundle-flac/src/decoder.rs:172-173` also does
  `self.buf.clear(); self.buf.render_uninit(Some(header.block_num_samples))`, then writes planes
  `0..channels` for `ChannelAssignment::Independant(channels)`, where `channels` comes from the
  frame header (`frame.rs:195-196`, `channels_enc + 1`) and is never cross-checked against the
  StreamInfo channel count the buffer was allocated for. A FLAC frame declaring 1 independent
  channel inside a 2-channel stream should therefore leave plane 1 holding the previous frame's
  samples and still return `Ok`. Filed as a new suspect, not verified.
- Other `render_uninit` call sites were only eyeballed, not verified:
  `symphonia-bundle-mp3/src/layer{1,2,3}/mod.rs`, `symphonia-codec-adpcm/src/lib.rs`,
  `symphonia-codec-vorbis/src/lib.rs`, `symphonia-play/src/resampler.rs`. Whether each of them
  really overwrites every committed frame of every plane on every input was not established.
- Not checked: whether any real-world AAC encoder/muxer emits a `raw_data_block` that omits a
  channel the ADTS header / AudioSpecificConfig declares (the reason the more permissive
  `render_silence` alternative above is spelled out). If such files exist, the strict `validate!`
  turns them from "wrong audio" into "unplayable", which is a maintainer decision.
- Not checked: the analogous question for a packet whose elements cover the channels but whose
  `ID_FIL`/`ID_DSE` handling leaves the bit reader mid-element - that is a separate parsing concern,
  not an output-buffer-coverage one.
- A longer fuzzing campaign would probably not find this on its own: the symptom is silently wrong
  output, not a crash, and `symphonia/fuzz`'s `decode_aac` target only checks that decoding does
  not panic.
