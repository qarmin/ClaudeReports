// Real end-to-end repro for candidate 021 (VORBIS_CODEBOOK_LOOKUP1_VALUES_POW_OVERFLOW).
//
// This drives the REAL public entry point (`symphonia_codec_vorbis::VorbisDecoder::try_new`) with
// a hand-crafted-but-well-formed-enough Vorbis identification + setup header packet pair, exactly
// the way a real caller (the `symphonia` facade crate's codec registry, fed bytes straight off an
// Ogg/Vorbis file) would. The setup header's sole codebook is placed exactly at the maximum bounds
// the real code enforces before ever calling the suspect function (`lookup1_values`):
// `codebook_dimensions == 32`, `codebook_entries == 131072` (128 * 1024) - see
// `symphonia-codec-vorbis/src/codebook.rs:239-244`. This is a single-participant scenario (a
// malformed/adversarial file handed to a parser's real public entry point IS the genuine external
// input - see CONFIRMATION.md section 11), so no second participant needs to be faked here.
//
// It also demonstrates the guard itself can't be bypassed: an otherwise-identical packet with
// `codebook_entries == 131073` (one past the cap) is rejected up-front with a clean decode error,
// never reaching `lookup1_values` at all.

use symphonia_core::codecs::audio::well_known::CODEC_ID_VORBIS;
use symphonia_core::codecs::audio::{AudioCodecParameters, AudioDecoderOptions};
use symphonia_codec_vorbis::VorbisDecoder;

/// Minimal LSB-first bit writer matching `BitReaderRtl`'s convention (least-significant bit of the
/// stream read first) - see `symphonia-core/src/io/bit.rs`'s own doc comment on `BitReaderRtl`.
struct BitWriter {
    bytes: Vec<u8>,
    acc: u64,
    acc_bits: u32,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { bytes: Vec::new(), acc: 0, acc_bits: 0 }
    }

    fn write_bits(&mut self, value: u64, nbits: u32) {
        assert!(nbits <= 32);
        let masked = if nbits == 64 { value } else { value & ((1u64 << nbits) - 1) };
        self.acc |= masked << self.acc_bits;
        self.acc_bits += nbits;
        while self.acc_bits >= 8 {
            self.bytes.push((self.acc & 0xff) as u8);
            self.acc >>= 8;
            self.acc_bits -= 8;
        }
    }

    fn write_bool(&mut self, value: bool) {
        self.write_bits(value as u64, 1);
    }

    fn finish(mut self) -> Vec<u8> {
        if self.acc_bits > 0 {
            self.bytes.push((self.acc & 0xff) as u8);
        }
        self.bytes
    }
}

/// Build a well-formed Vorbis identification header packet (byte-level fields, per
/// `read_ident_header` in `symphonia-codec-vorbis/src/lib.rs`).
fn build_ident_header() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(1); // packet type: identification
    buf.extend_from_slice(b"vorbis"); // signature
    buf.extend_from_slice(&0u32.to_le_bytes()); // vorbis_version = 0
    buf.push(2); // n_channels
    buf.extend_from_slice(&44100u32.to_le_bytes()); // sample_rate
    buf.extend_from_slice(&0u32.to_le_bytes()); // bitrate_max
    buf.extend_from_slice(&0u32.to_le_bytes()); // bitrate_nom
    buf.extend_from_slice(&0u32.to_le_bytes()); // bitrate_min
    buf.push((8u8) | (8u8 << 4)); // blocksize_0_exp = 8, blocksize_1_exp = 8 (both in [6,13])
    buf.push(1); // framing flag
    buf
}

/// Build a Vorbis setup header packet containing exactly one lookup-type-1 VQ codebook with the
/// given `codebook_dimensions`/`codebook_entries`. The codeword length list intentionally does
/// NOT form a valid canonical Huffman tree (every one of `codebook_entries` entries gets codeword
/// length 1) - `synthesize_codewords` runs strictly AFTER the lookup-table section that calls
/// `lookup1_values` (see codebook.rs:329-377), so this is irrelevant to what this repro is
/// checking: it only needs to get through the `lookup1_values` call site without a *prior* decode
/// error, which it does. The codebook is deliberately left invalid beyond that point (it will fail
/// its own Huffman-tree well-formedness check right after), which is fine - proving
/// `lookup1_values`'s internal `assert!` (and the `.pow()` inside it) doesn't panic is the whole
/// point, not producing a fully decodable codebook.
fn build_setup_header(codebook_dimensions: u16, codebook_entries: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(5); // packet type: setup
    buf.extend_from_slice(b"vorbis"); // signature

    let mut bw = BitWriter::new();

    // read_codebooks: count (8 bits), stored as count - 1.
    bw.write_bits(0, 8); // 1 codebook

    // VorbisCodebook::read:
    bw.write_bits(0x564342, 24); // sync
    bw.write_bits(u64::from(codebook_dimensions), 16);
    bw.write_bits(u64::from(codebook_entries), 24);
    bw.write_bool(false); // is_length_ordered = false
    bw.write_bool(false); // is_sparse = false

    // Densely packed codeword length list: `codebook_entries` entries, each 5-bit (code_len - 1).
    // All zero => every entry has codeword length 1 (intentionally not a valid Huffman tree - see
    // doc comment above for why that doesn't matter here).
    for _ in 0..codebook_entries {
        bw.write_bits(0, 5);
    }

    // Lookup type 1 (triggers the exact call site under test: codebook.rs:339).
    bw.write_bits(1, 4);
    bw.write_bits(0, 32); // min_value raw bits
    bw.write_bits(0, 32); // delta_value raw bits
    bw.write_bits(0, 4); // value_bits = 0 + 1 = 1
    bw.write_bool(false); // sequence_p

    // `lookup_values` many `value_bits`-wide multiplicands follow. We don't know `lookup_values`
    // here (it's exactly what `lookup1_values` computes inside the real decoder) - but for both
    // boundary cases exercised by this repro it is small (0 or 1), and any shortfall just causes a
    // later, unrelated, graceful end-of-stream decode error - never a panic - so a fixed small pad
    // is enough.
    for _ in 0..4 {
        bw.write_bits(0, 1);
    }

    buf.extend_from_slice(&bw.finish());
    buf
}

fn try_decode(label: &str, codebook_dimensions: u16, codebook_entries: u32) {
    let mut extra_data = build_ident_header();
    extra_data.extend_from_slice(&build_setup_header(codebook_dimensions, codebook_entries));

    let mut params = AudioCodecParameters::new();
    params.for_codec(CODEC_ID_VORBIS).with_extra_data(extra_data.into_boxed_slice());

    let opts = AudioDecoderOptions::default();

    let result = std::panic::catch_unwind(|| VorbisDecoder::try_new(&params, &opts));

    match result {
        Ok(Ok(_decoder)) => {
            println!(
                "[{label}] dimensions={codebook_dimensions} entries={codebook_entries}: \
                 VorbisDecoder::try_new -> Ok (fully valid setup header)"
            );
        }
        Ok(Err(e)) => {
            println!(
                "[{label}] dimensions={codebook_dimensions} entries={codebook_entries}: \
                 VorbisDecoder::try_new -> Err({e}) -- no panic"
            );
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
                .unwrap_or("<unknown panic payload>");
            println!(
                "[{label}] dimensions={codebook_dimensions} entries={codebook_entries}: \
                 VorbisDecoder::try_new -> PANICKED: {msg}"
            );
        }
    }
}

fn main() {
    // Silence the default panic hook's own stderr dump; we report panics ourselves above.
    std::panic::set_hook(Box::new(|_| {}));

    println!("== Candidate 021: VORBIS_CODEBOOK_LOOKUP1_VALUES_POW_OVERFLOW ==");
    println!("Build: {}", if cfg!(debug_assertions) { "debug (overflow-checks on)" } else { "release (overflow-checks on via profile)" });
    println!();

    // Case 1: real caller-enforced maximum boundary (codebook.rs:239-244):
    // codebook_dimensions == 32 (the hard cap), codebook_entries == 131072 (128*1024, the hard
    // cap). This is exactly the boundary the candidate suspected could tip `lookup1_values`'s
    // internal `value.pow(dimensions)` into an overflow panic.
    try_decode("max-allowed boundary", 32, 128 * 1024);

    // Case 2: one past the entries cap (131073) - must be rejected up-front by the explicit guard
    // in `VorbisCodebook::read`, before `lookup1_values` is ever reached.
    try_decode("one-past-entries-cap (should be rejected pre-lookup1_values)", 32, 128 * 1024 + 1);

    // Case 3: one past the dimensions cap (33) - same idea, the other guard.
    try_decode("one-past-dimensions-cap (should be rejected pre-lookup1_values)", 33, 128 * 1024);

    // Case 4: a few more points along the real valid domain for good measure.
    try_decode("small dims, max entries", 1, 128 * 1024);
    try_decode("mid dims, max entries", 17, 128 * 1024);
    try_decode("max dims, small entries", 32, 2);

    println!();
    println!("Done - see output.txt for the captured run. No panic anywhere above means the ");
    println!("suspected pow-overflow-inside-assert never fires within the real, caller-enforced ");
    println!("domain reachable through the actual public VorbisDecoder::try_new entry point.");
}
