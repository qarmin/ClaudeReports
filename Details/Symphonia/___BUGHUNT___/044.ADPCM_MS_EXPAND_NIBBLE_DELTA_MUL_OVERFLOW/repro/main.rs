//! Standalone reproduction for: `attempt to multiply with overflow` panic in
//! `symphonia-codec-adpcm`'s MS ADPCM decoder, in
//! `AdpcmMsBlockStatus::expand_nibble` (`symphonia-codec-adpcm/src/codec_ms.rs:96`):
//!
//! ```ignore
//! self.delta = (MS_ADAPTATION_TABLE[nibble as usize] * self.delta) / 256;
//! ```
//!
//! `self.delta` is seeded directly from an attacker/file-controlled 16-bit value in the block
//! preamble (`u16_to_i32!` sign-extends it as `i16`, so its usable range is -32768..=32767) and is
//! never bounded above afterwards (only `.max(DELTA_MIN)` clamps it from below). Each
//! adaptation-table entry of 768 (index 8) multiplies `delta` by exactly 3 (768 / 256). Repeating
//! nibble value 8 a handful of times in a row grows `delta` past `i32::MAX / 768` (~2,796,207), at
//! which point the multiplication itself overflows `i32` before the `/ 256` ever runs -
//! `attempt to multiply with overflow` in a debug-assertions/overflow-checks build (the default
//! for `cargo fuzz build`, and for any consumer building Symphonia in debug mode). In a plain
//! release build (overflow-checks off) the same input silently wraps `delta` instead of panicking
//! - a real, silent logic bug either way.
//!
//! Found via real fuzzing of the real `decode_adpcm` cargo-fuzz target (see bug-hunt candidate
//! 044's RESULT.md for how the harness needed candidate 012's fix first to reach this code path
//! at all) - this repro reduces that fuzzer-found crash to a minimal, deterministic, hand-crafted
//! MS ADPCM stereo block.
//!
//! Uses the real public decoder API (`RegisterableAudioDecoder::try_registry_new` +
//! `AudioDecoder::decode`), the same entry point `symphonia::default`'s codec registry and every
//! format reader in this workspace goes through - exactly candidate 002/003's repro style. `dur`
//! is set to a real, correct, non-zero block duration (as any real demuxer, e.g. the WAV reader,
//! would set it for an MS ADPCM packet) - this bug is reachable by real production decoding of a
//! malformed/adversarial MS ADPCM WAV file, not just by the fuzz harness.

use symphonia_codec_adpcm::AdpcmDecoder;
use symphonia_core::audio::layouts::CHANNEL_LAYOUT_STEREO;
use symphonia_core::codecs::audio::AudioCodecParameters;
use symphonia_core::codecs::audio::well_known::CODEC_ID_ADPCM_MS;
use symphonia_core::codecs::registry::RegisterableAudioDecoder;
use symphonia_core::packet::Packet;
use symphonia_core::units::{Duration, Timestamp};

fn main() {
    let mut params = AudioCodecParameters::new();
    params
        .for_codec(CODEC_ID_ADPCM_MS)
        .with_sample_rate(44100)
        .with_channels(CHANNEL_LAYOUT_STEREO)
        .with_frames_per_block(1024)
        .with_max_frames_per_packet(1024);

    let mut decoder = AdpcmDecoder::try_registry_new(&params, &Default::default())
        .expect("decoder should construct with frames_per_block=1024");

    // MS ADPCM stereo preamble (14 bytes), see `AdpcmMsBlockStatus::read_stereo_preamble`:
    //   left_block_predictor(1) right_block_predictor(1)
    //   left_delta(u16 LE) right_delta(u16 LE)
    //   left_sample1(u16 LE) right_sample1(u16 LE)
    //   left_sample2(u16 LE) right_sample2(u16 LE)
    let mut data = Vec::new();
    data.push(0u8); // left_block_predictor = 0 (valid, 0..=6)
    data.push(0u8); // right_block_predictor = 0 (valid, 0..=6)
    data.extend_from_slice(&0x7FFFu16.to_le_bytes()); // left_delta = 32767 (u16_to_i32! sign-extends as i16, so 0x7FFF is the largest positive value it can produce)
    data.extend_from_slice(&0x0010u16.to_le_bytes()); // right_delta = 16 (irrelevant to the crash)
    data.extend_from_slice(&0u16.to_le_bytes()); // left_sample1
    data.extend_from_slice(&0u16.to_le_bytes()); // right_sample1
    data.extend_from_slice(&0u16.to_le_bytes()); // left_sample2
    data.extend_from_slice(&0u16.to_le_bytes()); // right_sample2

    // Payload bytes: upper nibble drives the left channel's `expand_nibble` (one step per byte),
    // lower nibble drives the right channel. Upper nibble = 8 selects MS_ADAPTATION_TABLE[8] =
    // 768 (x3 growth per step). 6 steps starting from delta=32767 is enough to overflow the
    // multiplication (verified: overflow occurs exactly on the 6th step, see RESULT.md).
    for _ in 0..6 {
        data.push(0x80);
    }

    // A real demuxer sets `dur` to the actual number of decoded frames in the packet
    // (1 block * 1024 frames/block here) - this is not the fuzz harness's `Duration::ZERO` bug
    // (candidate 012), it's what real production decoding looks like.
    let packet = Packet::new(0, Timestamp::ZERO, Duration::new(1024), data);

    match decoder.decode(&packet) {
        Ok(_) => println!("decoded OK (no crash) - did not reproduce"),
        Err(e) => println!("decode error (no crash): {e} - did not reproduce"),
    }
}
