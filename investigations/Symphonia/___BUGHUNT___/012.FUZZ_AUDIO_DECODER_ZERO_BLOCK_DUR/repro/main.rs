// Standalone repro for candidate 012 (FUZZ_AUDIO_DECODER_ZERO_BLOCK_DUR).
//
// This calls the REAL, unmodified fuzz harness entry point,
// `symphonia_fuzz::fuzz_audio_decoder` (symphonia/fuzz/src/lib.rs), exactly as every
// `decode_*` fuzz target in this repo does - not a hand-built stand-in for it. The only
// repro-local addition is a thin observing wrapper decoder (`ObservingAdpcmDecoder`) that just
// forwards every call to the real, public `AdpcmDecoder` and records what happened, so this
// process can tell from the outside whether the harness's own packet construction ever let the
// real block-decode loop (`codec_ms::decode_mono`/`decode_stereo`) run. No second participant is
// faked - there's only one real caller here (the fuzz harness / a caller of the public decoder
// API), driven directly - see CONFIRMATION.md section 11.
//
// `codec_params` below mirrors symphonia/fuzz/fuzz_targets/decode_adpcm.rs exactly (ADPCM MS,
// 44100 Hz, stereo, frames_per_block=1024, max_frames_per_packet=1024). `packet_data` is
// deliberately too short for a real block, so entering the block-decode loop at all must produce
// an `Err` (proving the loop actually ran and read the bytes) - it cannot silently succeed.
//
// Usage: `cargo run --release` against the pristine harness (Duration::ZERO) reproduces the bug:
// the real block-decode loop never runs, for any packet_data. Re-run against the patched harness
// (patch.diff applied to ./clone) to see it now actually enter the loop.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use symphonia_codec_adpcm::AdpcmDecoder;
use symphonia_core::audio::layouts::CHANNEL_LAYOUT_STEREO;
use symphonia_core::codecs::CodecInfo;
use symphonia_core::audio::GenericAudioBufferRef;
use symphonia_core::codecs::audio::{
    AudioCodecParameters, AudioDecoder, AudioDecoderOptions, FinalizeResult,
};
use symphonia_core::codecs::registry::{RegisterableAudioDecoder, SupportedAudioCodec};
use symphonia_core::errors::Result;
use symphonia_core::packet::PacketRef;
use symphonia_fuzz::fuzz_audio_decoder;

static DECODE_LOOP_ENTERED: AtomicBool = AtomicBool::new(false);
static OUTPUT_FRAMES: AtomicUsize = AtomicUsize::new(0);
static DECODE_ERR: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// Thin observer around the real, public `AdpcmDecoder`: forwards every call unchanged, and
/// records (via process-wide statics, since `fuzz_audio_decoder` owns the decoder internally and
/// never exposes it back to the caller) whether the real block-decode loop actually ran.
struct ObservingAdpcmDecoder(AdpcmDecoder);

impl AudioDecoder for ObservingAdpcmDecoder {
    fn reset(&mut self) {
        self.0.reset();
    }

    fn codec_info(&self) -> &CodecInfo {
        self.0.codec_info()
    }

    fn codec_params(&self) -> &AudioCodecParameters {
        self.0.codec_params()
    }

    fn decode_ref(&mut self, packet: &PacketRef<'_>) -> Result<GenericAudioBufferRef<'_>> {
        let result = self.0.decode_ref(packet);
        match &result {
            // Ok with 0 frames means `block_count == 0`: decode_mono/decode_stereo were never
            // called at all. Ok with >0 frames, or an Err, both mean the loop actually ran (an
            // Err here specifically means it ran and then failed to read our deliberately
            // truncated packet_data - proving it genuinely tried).
            Ok(buf) => {
                OUTPUT_FRAMES.store(buf.frames(), Ordering::SeqCst);
                DECODE_LOOP_ENTERED.store(buf.frames() > 0, Ordering::SeqCst);
            }
            Err(e) => {
                DECODE_LOOP_ENTERED.store(true, Ordering::SeqCst);
                *DECODE_ERR.lock().unwrap() = Some(e.to_string());
            }
        }
        result
    }

    fn finalize(&mut self) -> FinalizeResult {
        self.0.finalize()
    }

    fn last_decoded(&self) -> GenericAudioBufferRef<'_> {
        self.0.last_decoded()
    }
}

impl RegisterableAudioDecoder for ObservingAdpcmDecoder {
    fn try_registry_new(
        params: &AudioCodecParameters,
        opts: &AudioDecoderOptions,
    ) -> Result<Box<dyn AudioDecoder>> {
        Ok(Box::new(ObservingAdpcmDecoder(AdpcmDecoder::try_new(params, opts)?)))
    }

    fn supported_codecs() -> &'static [SupportedAudioCodec] {
        AdpcmDecoder::supported_codecs()
    }
}

fn main() {
    // Mirrors symphonia/fuzz/fuzz_targets/decode_adpcm.rs exactly.
    let supported_codecs = AdpcmDecoder::supported_codecs();
    let codec_id = supported_codecs[0].id; // CODEC_ID_ADPCM_MS

    let mut codec_params = AudioCodecParameters::new();
    codec_params
        .for_codec(codec_id)
        .with_sample_rate(44100)
        .with_channels(CHANNEL_LAYOUT_STEREO)
        .with_frames_per_block(1024)
        .with_max_frames_per_packet(1024);

    // Deliberately truncated/garbage packet payload: nowhere near enough bytes for even one
    // real ADPCM MS block. If the block-decode loop ever actually runs on this, it must fail to
    // read it - it cannot silently succeed.
    let packet_data: Vec<u8> = vec![0xAB; 3];

    // Call the REAL, unmodified harness entry point - exactly what every decode_adpcm fuzz
    // iteration does.
    fuzz_audio_decoder::<ObservingAdpcmDecoder>(&codec_params, packet_data);

    let entered = DECODE_LOOP_ENTERED.load(Ordering::SeqCst);
    let frames = OUTPUT_FRAMES.load(Ordering::SeqCst);
    let err = DECODE_ERR.lock().unwrap().clone();

    println!("block-decode loop entered = {entered}, output frames = {frames}, decode err = {err:?}");

    if !entered {
        eprintln!(
            "REPRODUCED: symphonia_fuzz::fuzz_audio_decoder::<AdpcmDecoder> never entered the \
             real ADPCM block-decode loop (decode_mono/decode_stereo) for this packet_data - it \
             constructs every Packet with Duration::ZERO, so Packet::block_dur() is always 0 and \
             `block_count = packet.block_dur().get() as usize / frames_per_block` is always 0, \
             for any packet_data whatsoever."
        );
        std::process::exit(1);
    }
    else {
        println!(
            "did not reproduce - the real block-decode loop actually ran (err = {:?})",
            err
        );
    }
}
