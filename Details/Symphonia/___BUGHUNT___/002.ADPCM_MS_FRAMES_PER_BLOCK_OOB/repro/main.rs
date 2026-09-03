//! Standalone reproduction for: index-out-of-bounds panic in symphonia-codec-adpcm's MS ADPCM
//! decoder when `frames_per_block == 1`.
//!
//! `AdpcmDecoder::try_new` only rejects `frames_per_block == 0` (or missing), not the actual
//! minimum MS ADPCM needs (2, since every block stores 2 uncompressed preamble samples before any
//! compressed nibbles). `codec_ms::decode_mono`/`decode_stereo` unconditionally write index 1 of
//! their output buffer (whose length is exactly `frames_per_block`), so `frames_per_block == 1`
//! panics instead of returning a decode error.
//!
//! Uses the real public decoder API (`RegisterableAudioDecoder::try_registry_new` +
//! `AudioDecoder::decode`), the same entry point `symphonia::default`'s codec registry and every
//! format reader in this workspace goes through.

use symphonia_core::audio::layouts::CHANNEL_LAYOUT_STEREO;
use symphonia_core::codecs::audio::AudioCodecParameters;
use symphonia_core::codecs::audio::well_known::CODEC_ID_ADPCM_MS;
use symphonia_core::codecs::registry::RegisterableAudioDecoder;
use symphonia_core::packet::Packet;
use symphonia_core::units::{Duration, Timestamp};
use symphonia_codec_adpcm::AdpcmDecoder;

fn main() {
    let mut params = AudioCodecParameters::new();
    params
        .for_codec(CODEC_ID_ADPCM_MS)
        .with_sample_rate(44100)
        .with_channels(CHANNEL_LAYOUT_STEREO)
        .with_frames_per_block(1) // The minimum value AdpcmDecoder::try_new() accepts as "valid".
        .with_max_frames_per_packet(1);

    let mut decoder = AdpcmDecoder::try_registry_new(&params, &Default::default())
        .expect("decoder should construct with frames_per_block=1");

    // MS ADPCM stereo preamble is 14 bytes: left/right predictor(1 each) + left/right delta(2
    // each) + left/right sample1(2 each) + left/right sample2(2 each).
    let data = vec![0u8; 14];

    // A real demuxer sets `dur` to the actual number of decoded frames in the packet (1 block *
    // 1 frame/block here).
    let packet = Packet::new(0, Timestamp::ZERO, Duration::new(1), data);

    match decoder.decode(&packet) {
        Ok(_) => println!("decoded OK (no crash)"),
        Err(e) => println!("decode error (no crash): {e}"),
    }
}
