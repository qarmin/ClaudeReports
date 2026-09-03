//! Standalone bench + correctness harness for hayro-ccitt's `BitReader`/`decode_run_inner`
//! hot loop (candidate 044).
//!
//! Two workloads, both driven through the real public `hayro_ccitt::decode` entry point:
//!
//! 1. A large synthetic Group3-1D (Modified Huffman) fax image, generated here with a
//!    from-scratch MH encoder (standard ITU-T T.4 code tables, transcribed independently of
//!    hayro-ccitt's own decode tables) so the bench doesn't depend on finding a real scanned
//!    fax PDF in the repo. This is the actual timed workload - it exercises
//!    `decode_run_inner`/`read_bit` at realistic density (mix of short terminating-only runs
//!    and long runs needing one or more make-up codes, i.e. up to 13-bit codes). Round-tripping
//!    it back to the exact original run lengths is itself a strong correctness check on the
//!    decoder (independent of hayro-ccitt's own internals, since the encoder here is separate
//!    code).
//! 2. A deterministic sample of `hayro-fuzz`'s `fuzz_ccitt` corpus (`testdata/corpus/`, copied
//!    in), replayed byte-for-byte through the same 10-byte-header + payload format the fuzz
//!    target uses. These are *not* valid CCITT streams in general (the corpus was seeded from
//!    whole test PDFs, not crafted fuzz_ccitt inputs) - most error out quickly - but that's
//!    exactly why they're valuable here: they exercise `read_bit`'s end-of-input/short-read
//!    behavior (the part of the change under test) far more than the synthetic image does.
//!    Included in the correctness hash, not in the timed loop (too fast/varied to time
//!    meaningfully, and not representative of steady-state decode throughput).
//!
//! Usage: `bench <testdata-dir> <mode>`
//!   mode = `bench`  -> run N timed iterations of the synthetic-image decode, print
//!                      min/p50/mean/p95/max, plus one correctness hash line.
//!   mode = `hash`   -> just print the combined correctness hash (synthetic image + corpus
//!                      replay) and exit; used to compare baseline vs patched builds cheaply.

use hayro_ccitt::{DecodeSettings, Decoder, DecoderContext, EncodingMode};
use std::hash::{Hash, Hasher};
use std::time::Instant;

// ---------------------------------------------------------------------------------------------
// Standard ITU-T T.4 Modified Huffman code tables (Table 2/T.4, Table 3a/T.4, Table 3b/T.4).
// These are the same publicly-standardized codes hayro-ccitt's own `state_machine.rs` decodes
// from - transcribed here independently to build an *encoder*, so round-tripping through
// hayro_ccitt::decode is a real check, not a tautology against the code under test.
// (run_length, code_length_bits, code_value)
// ---------------------------------------------------------------------------------------------

const WHITE_TERMINATING: [(u32, u8, u16); 64] = [
    (0, 8, 0b00110101),
    (1, 6, 0b000111),
    (2, 4, 0b0111),
    (3, 4, 0b1000),
    (4, 4, 0b1011),
    (5, 4, 0b1100),
    (6, 4, 0b1110),
    (7, 4, 0b1111),
    (8, 5, 0b10011),
    (9, 5, 0b10100),
    (10, 5, 0b00111),
    (11, 5, 0b01000),
    (12, 6, 0b001000),
    (13, 6, 0b000011),
    (14, 6, 0b110100),
    (15, 6, 0b110101),
    (16, 6, 0b101010),
    (17, 6, 0b101011),
    (18, 7, 0b0100111),
    (19, 7, 0b0001100),
    (20, 7, 0b0001000),
    (21, 7, 0b0010111),
    (22, 7, 0b0000011),
    (23, 7, 0b0000100),
    (24, 7, 0b0101000),
    (25, 7, 0b0101011),
    (26, 7, 0b0010011),
    (27, 7, 0b0100100),
    (28, 7, 0b0011000),
    (29, 8, 0b00000010),
    (30, 8, 0b00000011),
    (31, 8, 0b00011010),
    (32, 8, 0b00011011),
    (33, 8, 0b00010010),
    (34, 8, 0b00010011),
    (35, 8, 0b00010100),
    (36, 8, 0b00010101),
    (37, 8, 0b00010110),
    (38, 8, 0b00010111),
    (39, 8, 0b00101000),
    (40, 8, 0b00101001),
    (41, 8, 0b00101010),
    (42, 8, 0b00101011),
    (43, 8, 0b00101100),
    (44, 8, 0b00101101),
    (45, 8, 0b00000100),
    (46, 8, 0b00000101),
    (47, 8, 0b00001010),
    (48, 8, 0b00001011),
    (49, 8, 0b01010010),
    (50, 8, 0b01010011),
    (51, 8, 0b01010100),
    (52, 8, 0b01010101),
    (53, 8, 0b00100100),
    (54, 8, 0b00100101),
    (55, 8, 0b01011000),
    (56, 8, 0b01011001),
    (57, 8, 0b01011010),
    (58, 8, 0b01011011),
    (59, 8, 0b01001010),
    (60, 8, 0b01001011),
    (61, 8, 0b00110010),
    (62, 8, 0b00110011),
    (63, 8, 0b00110100),
];

const WHITE_MAKEUP: [(u32, u8, u16); 27] = [
    (64, 5, 0b11011),
    (128, 5, 0b10010),
    (192, 6, 0b010111),
    (256, 7, 0b0110111),
    (320, 8, 0b00110110),
    (384, 8, 0b00110111),
    (448, 8, 0b01100100),
    (512, 8, 0b01100101),
    (576, 8, 0b01101000),
    (640, 8, 0b01100111),
    (704, 9, 0b011001100),
    (768, 9, 0b011001101),
    (832, 9, 0b011010010),
    (896, 9, 0b011010011),
    (960, 9, 0b011010100),
    (1024, 9, 0b011010101),
    (1088, 9, 0b011010110),
    (1152, 9, 0b011010111),
    (1216, 9, 0b011011000),
    (1280, 9, 0b011011001),
    (1344, 9, 0b011011010),
    (1408, 9, 0b011011011),
    (1472, 9, 0b010011000),
    (1536, 9, 0b010011001),
    (1600, 9, 0b010011010),
    (1664, 6, 0b011000),
    (1728, 9, 0b010011011),
];

const BLACK_TERMINATING: [(u32, u8, u16); 64] = [
    (0, 10, 0b0000110111),
    (1, 3, 0b010),
    (2, 2, 0b11),
    (3, 2, 0b10),
    (4, 3, 0b011),
    (5, 4, 0b0011),
    (6, 4, 0b0010),
    (7, 5, 0b00011),
    (8, 6, 0b000101),
    (9, 6, 0b000100),
    (10, 7, 0b0000100),
    (11, 7, 0b0000101),
    (12, 7, 0b0000111),
    (13, 8, 0b00000100),
    (14, 8, 0b00000111),
    (15, 9, 0b000011000),
    (16, 10, 0b0000010111),
    (17, 10, 0b0000011000),
    (18, 10, 0b0000001000),
    (19, 11, 0b00001100111),
    (20, 11, 0b00001101000),
    (21, 11, 0b00001101100),
    (22, 11, 0b00000110111),
    (23, 11, 0b00000101000),
    (24, 11, 0b00000010111),
    (25, 11, 0b00000011000),
    (26, 12, 0b000011001010),
    (27, 12, 0b000011001011),
    (28, 12, 0b000011001100),
    (29, 12, 0b000011001101),
    (30, 12, 0b000001101000),
    (31, 12, 0b000001101001),
    (32, 12, 0b000001101010),
    (33, 12, 0b000001101011),
    (34, 12, 0b000011010010),
    (35, 12, 0b000011010011),
    (36, 12, 0b000011010100),
    (37, 12, 0b000011010101),
    (38, 12, 0b000011010110),
    (39, 12, 0b000011010111),
    (40, 12, 0b000001101100),
    (41, 12, 0b000001101101),
    (42, 12, 0b000011011010),
    (43, 12, 0b000011011011),
    (44, 12, 0b000001010100),
    (45, 12, 0b000001010101),
    (46, 12, 0b000001010110),
    (47, 12, 0b000001010111),
    (48, 12, 0b000001100100),
    (49, 12, 0b000001100101),
    (50, 12, 0b000001010010),
    (51, 12, 0b000001010011),
    (52, 12, 0b000000100100),
    (53, 12, 0b000000110111),
    (54, 12, 0b000000111000),
    (55, 12, 0b000000100111),
    (56, 12, 0b000000101000),
    (57, 12, 0b000001011000),
    (58, 12, 0b000001011001),
    (59, 12, 0b000000101011),
    (60, 12, 0b000000101100),
    (61, 12, 0b000001011010),
    (62, 12, 0b000001100110),
    (63, 12, 0b000001100111),
];

const BLACK_MAKEUP: [(u32, u8, u16); 27] = [
    (64, 10, 0b0000001111),
    (128, 12, 0b000011001000),
    (192, 12, 0b000011001001),
    (256, 12, 0b000001011011),
    (320, 12, 0b000000110011),
    (384, 12, 0b000000110100),
    (448, 12, 0b000000110101),
    (512, 13, 0b0000001101100),
    (576, 13, 0b0000001101101),
    (640, 13, 0b0000001001010),
    (704, 13, 0b0000001001011),
    (768, 13, 0b0000001001100),
    (832, 13, 0b0000001001101),
    (896, 13, 0b0000001110010),
    (960, 13, 0b0000001110011),
    (1024, 13, 0b0000001110100),
    (1088, 13, 0b0000001110101),
    (1152, 13, 0b0000001110110),
    (1216, 13, 0b0000001110111),
    (1280, 13, 0b0000001010010),
    (1344, 13, 0b0000001010011),
    (1408, 13, 0b0000001010100),
    (1472, 13, 0b0000001010101),
    (1536, 13, 0b0000001011010),
    (1600, 13, 0b0000001011011),
    (1664, 13, 0b0000001100100),
    (1728, 13, 0b0000001100101),
];

const COMMON_MAKEUP: [(u32, u8, u16); 13] = [
    (1792, 11, 0b00000001000),
    (1856, 11, 0b00000001100),
    (1920, 11, 0b00000001101),
    (1984, 12, 0b000000010010),
    (2048, 12, 0b000000010011),
    (2112, 12, 0b000000010100),
    (2176, 12, 0b000000010101),
    (2240, 12, 0b000000010110),
    (2304, 12, 0b000000010111),
    (2368, 12, 0b000000011100),
    (2432, 12, 0b000000011101),
    (2496, 12, 0b000000011110),
    (2560, 12, 0b000000011111),
];

struct BitWriter {
    buf: Vec<u8>,
    cur: u8,
    nbits: u8,
}

impl BitWriter {
    fn new() -> Self {
        Self {
            buf: Vec::new(),
            cur: 0,
            nbits: 0,
        }
    }

    fn push_bits(&mut self, code: u16, len: u8) {
        for i in (0..len).rev() {
            let bit = (code >> i) & 1;
            self.cur = (self.cur << 1) | (bit as u8);
            self.nbits += 1;
            if self.nbits == 8 {
                self.buf.push(self.cur);
                self.cur = 0;
                self.nbits = 0;
            }
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.nbits > 0 {
            self.cur <<= 8 - self.nbits;
            self.buf.push(self.cur);
        }
        self.buf
    }
}

fn lookup_makeup(white: bool, val: u32) -> (u8, u16) {
    let table: &[(u32, u8, u16)] = if val >= 1792 {
        &COMMON_MAKEUP
    } else if white {
        &WHITE_MAKEUP
    } else {
        &BLACK_MAKEUP
    };
    for &(v, len, code) in table {
        if v == val {
            return (len, code);
        }
    }
    panic!("no makeup code for {val}");
}

fn lookup_terminating(white: bool, val: u32) -> (u8, u16) {
    let table: &[(u32, u8, u16)] = if white {
        &WHITE_TERMINATING
    } else {
        &BLACK_TERMINATING
    };
    for &(v, len, code) in table {
        if v == val {
            return (len, code);
        }
    }
    panic!("no terminating code for {val}");
}

/// Encode a single run length using make-up codes (in 64-pixel steps, capped at 2560) followed
/// by exactly one terminating code, per T.4 Section 4.1.1.
fn encode_run(bw: &mut BitWriter, white: bool, mut len: u32) {
    while len >= 64 {
        let makeup_val = (len.min(2560) / 64) * 64;
        let (code_len, code) = lookup_makeup(white, makeup_val);
        bw.push_bits(code, code_len);
        len -= makeup_val;
    }
    let (code_len, code) = lookup_terminating(white, len);
    bw.push_bits(code, code_len);
}

struct Xorshift64(u64);

impl Xorshift64 {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    // Returns a value in `0..bound`.
    fn below(&mut self, bound: u32) -> u32 {
        (self.next() % bound as u64) as u32
    }
}

/// Generates `num_rows` rows of `columns`-wide Modified-Huffman-encoded synthetic fax data,
/// plus the original per-row run lengths (starting with white, per T.4) so the decoded output
/// can be checked for an exact round trip.
fn generate_mh_image(columns: u32, num_rows: u32, seed: u64) -> (Vec<u8>, Vec<Vec<u32>>) {
    let mut rng = Xorshift64(seed);
    let mut bw = BitWriter::new();
    let mut rows = Vec::with_capacity(num_rows as usize);

    for _ in 0..num_rows {
        let mut runs = Vec::new();
        let mut remaining = columns;
        let mut white = true;

        while remaining > 0 {
            // ~18% of runs are "long" (needs 1+ make-up codes), rest are short
            // (terminating code only) - a rough approximation of scanned-text density where
            // most runs are short but background/margin gaps are long.
            let is_long = rng.below(100) < 18;
            let max_len = if is_long {
                remaining.min(3000)
            } else {
                remaining.min(63)
            };
            let run_len = if is_long && max_len >= 64 {
                64 + rng.below(max_len - 63)
            } else {
                rng.below(max_len + 1)
            };

            encode_run(&mut bw, white, run_len);
            runs.push(run_len);
            remaining -= run_len;
            white = !white;
        }

        rows.push(runs);
    }

    (bw.finish(), rows)
}

/// A `Decoder` that hashes the exact sequence of `push_pixels`/`next_line` calls it receives,
/// and also records them (for the round-trip check against the synthetic image's known input).
struct RecordingDecoder {
    hasher: std::collections::hash_map::DefaultHasher,
    rows: Vec<Vec<(bool, u32)>>,
    current_row: Vec<(bool, u32)>,
}

impl RecordingDecoder {
    fn new() -> Self {
        Self {
            hasher: std::collections::hash_map::DefaultHasher::new(),
            rows: Vec::new(),
            current_row: Vec::new(),
        }
    }
}

impl Decoder for RecordingDecoder {
    fn push_pixels(&mut self, white: bool, count: u32) {
        (white, count).hash(&mut self.hasher);
        self.current_row.push((white, count));
    }

    fn next_line(&mut self) {
        0xACu8.hash(&mut self.hasher);
        self.rows.push(std::mem::take(&mut self.current_row));
    }
}

fn decode_synthetic(data: &[u8], columns: u32, num_rows: u32) -> (u64, Vec<Vec<(bool, u32)>>) {
    let settings = DecodeSettings {
        columns,
        rows: num_rows,
        end_of_block: false,
        end_of_line: false,
        rows_are_byte_aligned: false,
        encoding: EncodingMode::Group3_1D,
        invert_black: false,
    };
    let mut decoder = RecordingDecoder::new();
    let mut ctx = DecoderContext::new(settings);
    let result = hayro_ccitt::decode(data, &mut decoder, &mut ctx);
    assert!(
        result.is_ok(),
        "synthetic image failed to decode: {:?}",
        result.err()
    );
    let mut hasher = decoder.hasher;
    result.unwrap().hash(&mut hasher);
    (hasher.finish(), decoder.rows)
}

fn verify_roundtrip(original: &[Vec<u32>], decoded: &[Vec<(bool, u32)>]) {
    assert_eq!(
        original.len(),
        decoded.len(),
        "row count mismatch: {} vs {}",
        original.len(),
        decoded.len()
    );
    for (row_idx, (orig_runs, dec_runs)) in original.iter().zip(decoded.iter()).enumerate() {
        // Decoded pushes can merge/split zero-length runs differently at the boundary, but
        // since our encoder never emits a zero-length *non-final* run implicitly, the sequence
        // of (color, len) pairs with len > 0 must match exactly, in order.
        let orig_nonzero: Vec<(bool, u32)> = {
            let mut white = true;
            let mut v = Vec::new();
            for &r in orig_runs {
                if r > 0 {
                    v.push((white, r));
                }
                white = !white;
            }
            v
        };
        let dec_nonzero: Vec<(bool, u32)> =
            dec_runs.iter().copied().filter(|&(_, c)| c > 0).collect();
        assert_eq!(
            orig_nonzero, dec_nonzero,
            "row {row_idx} mismatch: original {orig_nonzero:?} vs decoded {dec_nonzero:?}"
        );
    }
}

const HEADER_SIZE: usize = 10;

struct DiscardDecoder;
impl Decoder for DiscardDecoder {
    fn push_pixels(&mut self, _white: bool, _count: u32) {}
    fn next_line(&mut self) {}
}

/// Replays the fuzz corpus sample the same way `fuzz_targets/fuzz_ccitt.rs` interprets it, and
/// hashes the (white, count) event stream plus the final Result variant for every file, in
/// deterministic (sorted) file order.
fn hash_corpus_replay(dir: &std::path::Path) -> u64 {
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .expect("read corpus dir")
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();

    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    for path in paths {
        let data = std::fs::read(&path).expect("read corpus file");
        if data.len() < HEADER_SIZE {
            continue;
        }

        let columns = u16::from_le_bytes([data[0], data[1]]).max(1) as u32;
        let rows = u16::from_le_bytes([data[2], data[3]]).max(1) as u32;
        let end_of_block = data[4] != 0;
        let end_of_line = data[5] != 0;
        let rows_are_byte_aligned = data[6] != 0;
        let encoding = match data[7] % 3 {
            0 => EncodingMode::Group4,
            1 => EncodingMode::Group3_1D,
            _ => EncodingMode::Group3_2D {
                k: (data[8] as u32).max(1),
            },
        };
        let invert_black = data[9] != 0;

        let settings = DecodeSettings {
            columns,
            rows,
            end_of_block,
            end_of_line,
            rows_are_byte_aligned,
            encoding,
            invert_black,
        };

        struct HashingDecoder<'a>(&'a mut std::collections::hash_map::DefaultHasher);
        impl Decoder for HashingDecoder<'_> {
            fn push_pixels(&mut self, white: bool, count: u32) {
                (white, count).hash(self.0);
            }
            fn next_line(&mut self) {
                0xACu8.hash(self.0);
            }
        }

        let mut decoder = HashingDecoder(&mut hasher);
        let mut ctx = DecoderContext::new(settings);
        let result = hayro_ccitt::decode(&data[HEADER_SIZE..], &mut decoder, &mut ctx);
        // Hash which error variant (if any) - use the Debug repr, which is stable across the
        // baseline/patched builds we compare (no field values that would differ, only variant
        // shape) since DecodeError has no data-carrying variants.
        format!("{result:?}").hash(&mut hasher);
    }

    hasher.finish()
}

fn combined_correctness_hash(testdata_dir: &std::path::Path) -> (u64, Vec<Vec<u32>>) {
    let columns = 1728;
    let num_rows = 3000;
    let (data, original_rows) = generate_mh_image(columns, num_rows, 0x5EED_1234_C0FF_EE01);
    let (synthetic_hash, decoded_rows) = decode_synthetic(&data, columns, num_rows);
    verify_roundtrip(&original_rows, &decoded_rows);

    let corpus_dir = testdata_dir.join("corpus");
    let corpus_hash = if corpus_dir.is_dir() {
        hash_corpus_replay(&corpus_dir)
    } else {
        eprintln!("warning: no corpus dir at {corpus_dir:?}, skipping corpus replay check");
        0
    };

    let mut combined = std::collections::hash_map::DefaultHasher::new();
    synthetic_hash.hash(&mut combined);
    corpus_hash.hash(&mut combined);
    (combined.finish(), original_rows)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let testdata_dir = std::path::PathBuf::from(args.get(1).expect("usage: bench <testdata-dir> <mode>"));
    let mode = args.get(2).map(String::as_str).unwrap_or("bench");

    let (hash, _) = combined_correctness_hash(&testdata_dir);
    println!("correctness_hash={hash:016x}");

    if mode == "hash" {
        return;
    }

    // Timed workload: re-decode the synthetic image repeatedly (data generated once, decode
    // context/decoder recreated per iteration so no state leaks between runs).
    let columns = 1728;
    let num_rows = 3000;
    let (data, _original_rows) = generate_mh_image(columns, num_rows, 0x5EED_1234_C0FF_EE01);

    const ITERS: usize = 300;
    const WARMUP: usize = 30;

    let mut timings = Vec::with_capacity(ITERS);
    for i in 0..ITERS {
        let settings = DecodeSettings {
            columns,
            rows: num_rows,
            end_of_block: false,
            end_of_line: false,
            rows_are_byte_aligned: false,
            encoding: EncodingMode::Group3_1D,
            invert_black: false,
        };
        let mut decoder = DiscardDecoder;
        let mut ctx = DecoderContext::new(settings);

        let start = Instant::now();
        let result = std::hint::black_box(hayro_ccitt::decode(
            std::hint::black_box(&data),
            &mut decoder,
            &mut ctx,
        ));
        let elapsed = start.elapsed();
        assert!(result.is_ok());

        if i >= WARMUP {
            timings.push(elapsed);
        }
    }

    timings.sort();
    let n = timings.len();
    let min = timings[0];
    let p50 = timings[n / 2];
    let p95 = timings[(n * 95) / 100];
    let max = timings[n - 1];
    let mean: std::time::Duration = timings.iter().sum::<std::time::Duration>() / n as u32;

    println!(
        "iters={n} min={min:?} p50={p50:?} mean={mean:?} p95={p95:?} max={max:?}"
    );
}
