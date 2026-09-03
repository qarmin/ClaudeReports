//! Verification harness for candidate 042 (Not reproducible): `PcmDecoder::decode_inner`
//! discards the `Result` of `render_with(None, ...)` via `let _ = match { ... }` and always
//! returns `Ok(())`. The concern was that a truncated packet after a larger one could produce a
//! WRONG (not-floor-division) frame count that gets silently reported as a successful decode,
//! since `num_frames` (computed from `packet.data.len()`) is only ever used to `grow_capacity`
//! (which never shrinks).
//!
//! This drives the real public `PcmDecoder::try_new` / `AudioDecoder::decode_ref` entry points
//! directly (a decoder reacting to packet bytes it already owns in memory - there is no second
//! participant to fake here, see `CONFIRMATION.md` section 11), varying: codec (frame size),
//! channel count, and a "grow big, then shrink with a short/truncated packet" sequence, checking
//! whether the reported frame count and `Result` ever diverge from
//! `floor(data.len() / frame_size)`.
//!
//! Every case matched exactly, in both a dev (debug-assertions on) and a release build - see
//! `RESULT.md` for why (`render_with` only commits a frame after its per-frame closure succeeds,
//! and `PacketRef`'s reader is always in-memory, so a short read always happens at exactly the
//! frame boundary `num_frames` computes).

use symphonia_core::audio::GenericAudioBuffer;
use symphonia_core::audio::layouts::{CHANNEL_LAYOUT_MONO, CHANNEL_LAYOUT_STEREO};
use symphonia_core::codecs::audio::well_known::{
    CODEC_ID_PCM_ALAW, CODEC_ID_PCM_F32LE, CODEC_ID_PCM_S16LE, CODEC_ID_PCM_S24LE,
    CODEC_ID_PCM_U8,
};
use symphonia_core::codecs::audio::{AudioCodecId, AudioCodecParameters, AudioDecoder, AudioDecoderOptions};
use symphonia_core::packet::PacketRef;
use symphonia_core::units::{Duration, Timestamp};

use symphonia_codec_pcm::PcmDecoder;

fn bytes_per_coded_sample(codec: AudioCodecId) -> usize {
    if codec == CODEC_ID_PCM_S16LE {
        2
    }
    else if codec == CODEC_ID_PCM_S24LE {
        3
    }
    else if codec == CODEC_ID_PCM_F32LE {
        4
    }
    else if codec == CODEC_ID_PCM_U8 || codec == CODEC_ID_PCM_ALAW {
        1
    }
    else {
        unreachable!()
    }
}

fn run_case(codec: AudioCodecId, channels: symphonia_core::audio::Channels, sizes_bytes: &[usize]) {
    let mut params = AudioCodecParameters::new();
    params.for_codec(codec).with_sample_rate(44100).with_channels(channels.clone());

    let mut decoder = PcmDecoder::try_new(&params, &AudioDecoderOptions::default()).unwrap();
    let bps = bytes_per_coded_sample(codec);
    let frame_size = bps * channels.count();

    let mut mismatches = Vec::new();

    for &len in sizes_bytes {
        let data = vec![0xABu8; len];
        let packet = PacketRef::new(0, Timestamp::ZERO, Duration::ZERO, &data);
        let expected_frames = len / frame_size;

        match decoder.decode_ref(&packet) {
            Ok(buf) => {
                let actual = buf.frames();
                if actual != expected_frames {
                    mismatches.push(format!(
                        "codec={:?} frame_size={} len={} expected_frames={} actual_frames={} (Ok)",
                        codec, frame_size, len, expected_frames, actual
                    ));
                }
            }
            Err(e) => {
                mismatches.push(format!(
                    "codec={:?} frame_size={} len={} expected_frames={} -> Err({:?}) \
                     (unexpected: decode_inner should never propagate an error)",
                    codec, frame_size, len, expected_frames, e
                ));
            }
        }
    }

    if mismatches.is_empty() {
        println!(
            "codec={:?} channels={} num_sizes_tried={}: ALL cases matched floor(len/frame_size), no error ever surfaced",
            codec,
            channels.count(),
            sizes_bytes.len()
        );
    }
    else {
        println!("codec={:?} MISMATCHES FOUND:", codec);
        for m in mismatches {
            println!("  {}", m);
        }
    }
}

fn main() {
    // Grow big, then shrink with a byte count 1..frame_size short of a whole number of frames,
    // then grow big again, then shrink again - repeated for several codecs/channel counts.
    run_case(
        CODEC_ID_PCM_S16LE,
        CHANNEL_LAYOUT_MONO,
        &[1000 * 2, 999, 500 * 2 - 1, 20, 1, 0, 2000 * 2, 3],
    );

    run_case(
        CODEC_ID_PCM_S16LE,
        CHANNEL_LAYOUT_STEREO,
        &[1000 * 4, 999, 500 * 4 - 1, 500 * 4 - 3, 40, 1, 2, 3, 0, 2000 * 4],
    );

    run_case(
        CODEC_ID_PCM_S24LE,
        CHANNEL_LAYOUT_STEREO,
        &[1000 * 6, 6 * 500 - 1, 6 * 500 - 5, 1, 2, 3, 4, 5, 0],
    );

    run_case(
        CODEC_ID_PCM_F32LE,
        CHANNEL_LAYOUT_MONO,
        &[1000 * 4, 4 * 500 - 1, 4 * 500 - 3, 1, 2, 3, 0],
    );

    run_case(CODEC_ID_PCM_U8, CHANNEL_LAYOUT_STEREO, &[2000, 2 * 500 - 1, 1, 0]);

    run_case(CODEC_ID_PCM_ALAW, CHANNEL_LAYOUT_MONO, &[1000, 500 - 0, 999, 0]);

    // Many small random-ish sizes bouncing up and down repeatedly, mono S16LE.
    let mut sizes = Vec::new();
    let mut x: u64 = 0x2545F4914F6CDD1D;
    for _ in 0..2000 {
        // xorshift64
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        sizes.push((x % 4001) as usize);
    }
    run_case(CODEC_ID_PCM_S16LE, CHANNEL_LAYOUT_MONO, &sizes);
}
