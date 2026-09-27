// Reproduction for candidate 043: ALAC_BUF_TRUNCATE_USES_LAST_ELEMENTS_SAMPLE_COUNT.
//
// A crafted, but well-formed-per-spec-checks, multichannel (3-channel) ALAC packet is fed
// through the real public decode path (`AlacDecoder::try_new` + `AudioDecoder::decode_ref`).
// The packet contains two elements:
//   - A Single Channel Element (SCE) that decodes 8 real samples (100..107) using the
//     uncompressed sample path.
//   - A Channel Pair Element (CPE), decoded *after* the SCE, that only decodes 3 real
//     samples per channel (a partial frame).
//
// `decode_inner` overwrites `num_frames` with the return value of *each* element in turn, so
// after the loop `num_frames == 3` (the CPE's count), not 8 (the SCE's count). The final
// `self.buf.truncate(num_frames)` then truncates ALL channels - including the SCE channel that
// validly decoded 8 real samples - down to 3 frames, silently discarding samples 103..107 with
// no error and no indication anything was lost.

use symphonia_core::audio::{Audio, GenericAudioBufferRef};
use symphonia_core::codecs::audio::well_known::CODEC_ID_ALAC;
use symphonia_core::codecs::audio::{AudioCodecParameters, AudioDecoder, AudioDecoderOptions};
use symphonia_core::packet::PacketRef;
use symphonia_core::units::{Duration, Timestamp};

use symphonia_codec_alac::AlacDecoder;

/// A simple MSB-first ("left-to-right") bit writer, matching `BitReaderLtr`'s bit order.
struct BitWriter {
    bytes: Vec<u8>,
    cur: u8,
    nbits: u8,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { bytes: Vec::new(), cur: 0, nbits: 0 }
    }

    fn push_bit(&mut self, bit: u8) {
        self.cur = (self.cur << 1) | (bit & 1);
        self.nbits += 1;
        if self.nbits == 8 {
            self.bytes.push(self.cur);
            self.cur = 0;
            self.nbits = 0;
        }
    }

    fn push_bits(&mut self, value: u64, n: u32) {
        for i in (0..n).rev() {
            self.push_bit(((value >> i) & 1) as u8);
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.nbits > 0 {
            self.cur <<= 8 - self.nbits;
            self.bytes.push(self.cur);
        }
        self.bytes
    }
}

fn build_magic_cookie(frame_length: u32, bit_depth: u8, num_channels: u8) -> Vec<u8> {
    let mut c = Vec::with_capacity(24);
    c.extend_from_slice(&frame_length.to_be_bytes()); // frame_length
    c.push(0); // compatible_version
    c.push(bit_depth); // bit_depth
    c.push(40); // pb
    c.push(10); // mb
    c.push(14); // kb
    c.push(num_channels); // num_channels
    c.extend_from_slice(&255u16.to_be_bytes()); // max_run
    c.extend_from_slice(&0u32.to_be_bytes()); // max_frame_bytes
    c.extend_from_slice(&0u32.to_be_bytes()); // avg_bit_rate
    c.extend_from_slice(&44100u32.to_be_bytes()); // sample_rate
    assert_eq!(c.len(), 24);
    c
}

/// Builds a 3-channel ALAC packet: one SCE with 8 real samples, followed by one CPE with only
/// 3 real samples (a shorter partial frame than the SCE that came before it).
fn build_packet() -> Vec<u8> {
    let mut bw = BitWriter::new();

    // --- Element 1: SCE, full frame (8 samples), uncompressed. ---
    bw.push_bits(0b000, 3); // ALAC_ELEM_TAG_SCE
    bw.push_bits(0, 4); // elem_instance_tag
    bw.push_bits(0, 12); // unused header bits (must be 0)
    bw.push_bits(0, 1); // is_partial_frame = false -> num_samples = config.frame_length (8)
    bw.push_bits(0, 2); // shift = 0
    bw.push_bits(1, 1); // is_uncompressed = true

    let sce_samples: [i32; 8] = [100, 101, 102, 103, 104, 105, 106, 107];
    for &v in &sce_samples {
        bw.push_bits(v as u16 as u64, 16);
    }

    // --- Element 2: CPE, partial frame (3 samples only), uncompressed. ---
    bw.push_bits(0b001, 3); // ALAC_ELEM_TAG_CPE
    bw.push_bits(0, 4); // elem_instance_tag
    bw.push_bits(0, 12); // unused header bits (must be 0)
    bw.push_bits(1, 1); // is_partial_frame = true
    bw.push_bits(0, 2); // shift = 0
    bw.push_bits(1, 1); // is_uncompressed = true
    bw.push_bits(3, 32); // num_samples = 3 (partial frame length, read as a raw u32)

    let cpe0: [i32; 3] = [200, 201, 202];
    let cpe1: [i32; 3] = [300, 301, 302];
    for i in 0..3 {
        bw.push_bits(cpe0[i] as u16 as u64, 16);
        bw.push_bits(cpe1[i] as u16 as u64, 16);
    }

    // After the CPE, next_channel == num_channels (1 + 2 == 3), so `decode_inner` breaks out of
    // the element loop on its own - no END tag is required.

    bw.finish()
}

fn main() {
    const FRAME_LENGTH: u32 = 8;
    const BIT_DEPTH: u8 = 16;
    const NUM_CHANNELS: u8 = 3;

    let magic_cookie = build_magic_cookie(FRAME_LENGTH, BIT_DEPTH, NUM_CHANNELS);
    let packet_data = build_packet();

    let mut codec_params = AudioCodecParameters::new();
    codec_params.for_codec(CODEC_ID_ALAC);
    codec_params.extra_data = Some(magic_cookie.into_boxed_slice());

    let opts = AudioDecoderOptions::default();
    let mut decoder = AlacDecoder::try_new(&codec_params, &opts)
        .expect("failed to construct AlacDecoder from a well-formed 3-channel magic cookie");

    let packet =
        PacketRef::new(0, Timestamp::new(0), Duration::new(FRAME_LENGTH as u64), &packet_data);

    let decoded = match decoder.decode_ref(&packet) {
        Ok(decoded) => decoded,
        Err(e) => {
            // A fix that rejects the malformed packet outright (elements disagreeing on frame
            // length) instead of silently truncating with the wrong count is an acceptable,
            // non-corrupting outcome.
            println!(
                "did not reproduce - decode_ref returned an error instead of silently \
                 corrupting output: {:?}",
                e
            );
            std::process::exit(0);
        }
    };

    let buf = match decoded {
        GenericAudioBufferRef::S32(buf) => buf,
        other => panic!("unexpected sample format: {:?}", std::mem::discriminant(&other)),
    };

    println!("decoded frames in buffer: {}", buf.frames());

    // Channel layout for 3 channels (MPEG_3P0_B) maps the first-decoded element (the SCE) to
    // canonical buffer plane index 2 - see `map_channels` in symphonia-codec-alac/src/lib.rs.
    let sce_plane = buf.plane(2).expect("plane 2 (SCE / center channel) should exist");

    println!("SCE (center) plane contents: {:?}", sce_plane);
    println!("expected (if the SCE's own 8 decoded samples were preserved): {:?}", [
        100, 101, 102, 103, 104, 105, 106, 107,
    ]);

    let expected_full: [i32; 8] = [100, 101, 102, 103, 104, 105, 106, 107];

    if buf.frames() == 8 && sce_plane == expected_full {
        println!("did not reproduce - all 8 SCE samples were preserved");
        std::process::exit(0);
    }
    else {
        println!(
            "REPRODUCED - buf.frames() == {} (expected 8); SCE plane only has {} sample(s), \
             samples 103..107 (indices 3..8) were silently dropped even though the SCE decoder \
             validly produced them",
            buf.frames(),
            sce_plane.len()
        );
        std::process::exit(1);
    }
}
