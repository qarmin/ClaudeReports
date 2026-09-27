// Hand-crafted minimal Ogg/Vorbis stream that triggers Residue::deinterleave_2's
// off-by-design index bug (symphonia-codec-vorbis/src/residue.rs) via the real,
// public, end-to-end Symphonia decode API (probe + decode), not internal calls.
//
// 4 channels, num_submaps=2, multiplex=[0,0,1,1]: submap 1 owns channels {2,3}.
// deinterleave_2 does `channels.split_at_mut(ch0).1.split_at_mut(ch1)` -- for
// ch0=2, ch1=3 that's `channels[2..4].split_at_mut(3)`, i.e. split_at_mut(3) on a
// 2-element slice, which panics (mid > len).

use std::io::Cursor;

use symphonia::core::formats::{FormatOptions, TrackType, probe::Hint};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::codecs::{CodecParameters, audio::AudioDecoderOptions};

struct BitWriter {
    out: Vec<u8>,
    acc: u64,
    nbits: u32,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { out: Vec::new(), acc: 0, nbits: 0 }
    }

    // Matches symphonia_core::io::bit::BitReaderRtl semantics: bits are packed
    // LSB-first within each byte, and multi-bit fields are also read/written
    // LSB-first (the first bit written becomes the LSB of the value read back).
    fn write_bits(&mut self, value: u32, n: u32) {
        if n == 0 {
            return;
        }
        let mask: u64 = if n == 32 { u32::MAX as u64 } else { (1u64 << n) - 1 };
        self.acc |= (u64::from(value) & mask) << self.nbits;
        self.nbits += n;
        while self.nbits >= 8 {
            self.out.push((self.acc & 0xff) as u8);
            self.acc >>= 8;
            self.nbits -= 8;
        }
    }

    fn write_bool(&mut self, b: bool) {
        self.write_bits(b as u32, 1);
    }

    fn finish(mut self) -> Vec<u8> {
        if self.nbits > 0 {
            self.out.push((self.acc & 0xff) as u8);
        }
        self.out
    }
}

fn build_ident_packet(n_channels: u8, sample_rate: u32, bs0_exp: u8, bs1_exp: u8) -> Vec<u8> {
    let mut v = Vec::new();
    v.push(1u8); // packet type: identification
    v.extend_from_slice(b"vorbis");
    v.extend_from_slice(&0u32.to_le_bytes()); // version
    v.push(n_channels);
    v.extend_from_slice(&sample_rate.to_le_bytes());
    v.extend_from_slice(&0u32.to_le_bytes()); // bitrate_max
    v.extend_from_slice(&0u32.to_le_bytes()); // bitrate_nom
    v.extend_from_slice(&0u32.to_le_bytes()); // bitrate_min
    v.push((bs0_exp & 0x0f) | ((bs1_exp & 0x0f) << 4));
    v.push(0x01); // framing flag
    assert_eq!(v.len(), 30, "identification header must be exactly 30 bytes");
    v
}

fn build_comment_packet() -> Vec<u8> {
    let mut v = Vec::new();
    v.push(3u8); // packet type: comment
    v.extend_from_slice(b"vorbis");
    v.extend_from_slice(&0u32.to_le_bytes()); // vendor_len = 0
    v.extend_from_slice(&0u32.to_le_bytes()); // num_comments = 0
    v
}

fn build_setup_packet() -> Vec<u8> {
    let mut bw = BitWriter::new();

    // --- Codebooks: 1 minimal scalar codebook (dims=1, entries=2, code_lens=[1,1], no VQ). ---
    bw.write_bits(0, 8); // codebook count - 1 = 0 -> 1 codebook
    bw.write_bits(0x564342, 24); // codebook sync
    bw.write_bits(1, 16); // dimensions
    bw.write_bits(2, 24); // entries
    bw.write_bool(false); // not length-ordered
    bw.write_bool(false); // not sparse
    bw.write_bits(0, 5); // entry 0 code_len - 1 = 0 -> length 1
    bw.write_bits(0, 5); // entry 1 code_len - 1 = 0 -> length 1
    bw.write_bits(0, 4); // lookup_type = 0 (no VQ table)

    // --- Time domain transforms: 1 entry, all zero (placeholder). ---
    bw.write_bits(0, 6); // count - 1 = 0 -> 1
    bw.write_bits(0, 16);

    // --- Floors: 1 floor, type 1, 0 partitions (fully legal, no codebooks needed). ---
    bw.write_bits(0, 6); // count - 1 = 0 -> 1 floor
    bw.write_bits(1, 16); // floor type 1
    bw.write_bits(0, 5); // floor1_partitions = 0
    bw.write_bits(0, 2); // floor1_multiplier - 1 = 0 -> 1
    bw.write_bits(0, 4); // rangebits = 0

    // --- Residues: 1 residue, type 2, minimal (no partitions ever decoded). ---
    bw.write_bits(0, 6); // count - 1 = 0 -> 1 residue
    bw.write_bits(2, 16); // residue type 2
    bw.write_bits(0, 24); // residue_begin = 0
    bw.write_bits(0, 24); // residue_end = 0 (residue_len == 0, no partitions to read)
    bw.write_bits(0, 24); // residue_partition_size - 1 = 0 -> 1
    bw.write_bits(0, 6); // residue_classifications - 1 = 0 -> 1
    bw.write_bits(0, 8); // residue_classbook = codebook #0
    bw.write_bits(0, 3); // classification 0: low_bits = 0
    bw.write_bool(false); // classification 0: no high_bits -> is_used = 0

    // --- Mappings: 1 mapping, type 0, num_submaps=2, multiplex=[0,0,1,1]. ---
    bw.write_bits(0, 6); // count - 1 = 0 -> 1 mapping
    bw.write_bits(0, 16); // mapping type 0
    bw.write_bool(true); // submaps field present
    bw.write_bits(1, 4); // num_submaps - 1 = 1 -> num_submaps = 2
    bw.write_bool(false); // no channel couplings
    bw.write_bits(0, 2); // reserved
    // multiplex per channel (4 channels): submap0 owns {0,1}, submap1 owns {2,3}
    bw.write_bits(0, 4);
    bw.write_bits(0, 4);
    bw.write_bits(1, 4);
    bw.write_bits(1, 4);
    // submap 0: reserved(8) + floor(8) + residue(8)
    bw.write_bits(0, 8);
    bw.write_bits(0, 8); // floor #0
    bw.write_bits(0, 8); // residue #0
    // submap 1: reserved(8) + floor(8) + residue(8)
    bw.write_bits(0, 8);
    bw.write_bits(0, 8); // floor #0
    bw.write_bits(0, 8); // residue #0

    // --- Modes: 1 mode, short block, mapping #0. ---
    bw.write_bits(0, 6); // count - 1 = 0 -> 1 mode
    bw.write_bool(false); // block_flag = short window
    bw.write_bits(0, 16); // window_type = 0
    bw.write_bits(0, 16); // transform_type = 0
    bw.write_bits(0, 8); // mapping #0

    // Framing bit.
    bw.write_bool(true);

    let body = bw.finish();

    let mut v = Vec::new();
    v.push(5u8); // packet type: setup
    v.extend_from_slice(b"vorbis");
    v.extend_from_slice(&body);
    v
}

fn build_audio_packet() -> Vec<u8> {
    // packet type bit (0 = audio), 0 mode-selection bits (only 1 mode exists),
    // short window (no window flag bits), then 4 floor "unused" bits (one per
    // channel, all 0). That's the entire packet - residue decode for the
    // {2,3} submap never reads a single bit before deinterleave_2 panics.
    let mut bw = BitWriter::new();
    bw.write_bool(false); // audio packet
    bw.write_bool(false); // channel 0 floor unused
    bw.write_bool(false); // channel 1 floor unused
    bw.write_bool(false); // channel 2 floor unused
    bw.write_bool(false); // channel 3 floor unused
    bw.finish()
}

// CRC-32/OGG: poly 0x04c11db7, init 0, no reflection, no final xor - matches
// symphonia_core::checksum::Crc32 exactly (verified against its source).
struct OggCrc {
    table: [u32; 256],
}

impl OggCrc {
    fn new() -> Self {
        let mut table = [0u32; 256];
        for i in 0..256u32 {
            let mut r = i << 24;
            for _ in 0..8 {
                r = if r & 0x8000_0000 != 0 { (r << 1) ^ 0x04c1_1db7 } else { r << 1 };
            }
            table[i as usize] = r;
        }
        OggCrc { table }
    }

    fn compute(&self, data: &[u8]) -> u32 {
        let mut state: u32 = 0;
        for &b in data {
            state = (state << 8) ^ self.table[usize::from((state >> 24) as u8 ^ b)];
        }
        state
    }
}

fn build_ogg_page(
    crc: &OggCrc,
    serial: u32,
    seq: u32,
    header_type: u8,
    granule: i64,
    packet: &[u8],
) -> Vec<u8> {
    assert!(packet.len() < 255, "single-segment page only");

    let mut page = Vec::new();
    page.extend_from_slice(b"OggS");
    page.push(0); // version
    page.push(header_type);
    page.extend_from_slice(&granule.to_le_bytes());
    page.extend_from_slice(&serial.to_le_bytes());
    page.extend_from_slice(&seq.to_le_bytes());
    page.extend_from_slice(&0u32.to_le_bytes()); // crc placeholder
    page.push(1); // page_segments
    page.push(packet.len() as u8); // lacing value (< 255, terminates the packet)
    page.extend_from_slice(packet);

    let checksum = crc.compute(&page);
    page[22..26].copy_from_slice(&checksum.to_le_bytes());

    page
}

fn main() {
    let crc = OggCrc::new();
    let serial = 0x1234_5678u32;

    let ident = build_ident_packet(4, 44100, 6, 6);
    let comment = build_comment_packet();
    let setup = build_setup_packet();
    let audio = build_audio_packet();

    eprintln!(
        "packet sizes: ident={} comment={} setup={} audio={}",
        ident.len(),
        comment.len(),
        setup.len(),
        audio.len()
    );

    let mut stream = Vec::new();
    stream.extend(build_ogg_page(&crc, serial, 0, 0x02, 0, &ident)); // BOS
    stream.extend(build_ogg_page(&crc, serial, 1, 0x00, 0, &comment));
    stream.extend(build_ogg_page(&crc, serial, 2, 0x00, 0, &setup));
    stream.extend(build_ogg_page(&crc, serial, 3, 0x04, 64, &audio)); // EOS

    let mss = MediaSourceStream::new(Box::new(Cursor::new(stream)), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("ogg");

    let fmt_opts: FormatOptions = Default::default();
    let meta_opts: MetadataOptions = Default::default();

    let mut format = symphonia::default::get_probe()
        .probe(&hint, mss, fmt_opts, meta_opts)
        .expect("probe should succeed on this hand-crafted but spec-legal Ogg/Vorbis stream");

    let track = format
        .default_track(TrackType::Audio)
        .expect("should find an audio track")
        .clone();

    let codec_params = match track.codec_params.as_ref() {
        Some(CodecParameters::Audio(audio)) => audio,
        _ => panic!("expected audio codec params"),
    };

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(codec_params, &AudioDecoderOptions::default())
        .expect("decoder should be created for a well-formed Vorbis extradata");

    let track_id = track.id;

    while let Some(packet) = format.next_packet().expect("demuxing should not fail") {
        if packet.track_id != track_id {
            continue;
        }
        eprintln!("decoding packet, {} bytes", packet.data.len());
        let _ = decoder.decode(&packet).expect("decode should not return a recoverable error");
    }

    println!("BUG NOT REPRODUCED: decode completed without panicking");
}
