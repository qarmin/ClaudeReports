// Standalone reproduction for the "frozen restart flag" bug in
// `jpeg_decoder::decoder::lossless::decode_scan_lossless`'s generic `predict()` branch
// (predictor selections 2-7 / NoPrediction), src/decoder/lossless.rs.
//
// There is no lossless JPEG encoder available (jpeg-decoder is decode-only, and no encoder
// crate was pulled in), so this program hand-builds a minimal, spec-valid lossless JPEG
// bitstream byte by byte: SOF3 (lossless) + DHT + DRI (restart interval = 5) + SOS
// (predictor selection 4, i.e. Ra+Rb-Rc) + Huffman-coded entropy data with RST markers
// inserted at the exact MCU boundaries the restart interval implies, followed by EOI.
//
// The 4x4 pixel grid and its DC differences are chosen by hand from the *correct* spec
// algorithm (Ra/Rb/Rc predictor, with the predictor reset to the frame's default value at
// every restart-interval boundary, per ITU-T T.81 H.1.2.3), so we know the ground-truth
// decoded image ahead of time. We then feed the bytes through the real `jpeg_decoder::Decoder`
// and compare its output against that ground truth.

use std::io::Cursor;

const WIDTH: usize = 4;
const HEIGHT: usize = 4;
const RESTART_INTERVAL: u16 = 5;
const PREDICTOR_SELECTION: u8 = 4; // Ra + Rb - Rc

// Ground truth 4x4 grayscale image (row-major, [y][x]), precision 8.
const GROUND_TRUTH: [[u8; WIDTH]; HEIGHT] = [
    [100, 110, 120, 130],
    [90, 200, 60, 180],
    [40, 170, 30, 210],
    [220, 50, 240, 20],
];

// Canonical DC Huffman table (the standard "typical" table from ITU-T T.81 Annex K.3),
// covering categories 0..=11. counts[i] = number of codes of length i+1.
const DC_COUNTS: [u8; 16] = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
const DC_VALUES: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

/// Returns (code, code_len_bits) for a given DC magnitude category, derived by the same
/// canonical-Huffman-code generation algorithm the decoder itself uses to build tables
/// from counts/values (ITU-T T.81 Annex C.2).
fn huffman_code_for_category(category: u8) -> (u32, u8) {
    let mut code: u32 = 0;
    let mut k = 0usize;
    for (len_idx, &count) in DC_COUNTS.iter().enumerate() {
        let len = (len_idx + 1) as u8;
        for _ in 0..count {
            let sym = DC_VALUES[k];
            if sym == category {
                return (code, len);
            }
            code += 1;
            k += 1;
        }
        code <<= 1;
    }
    panic!("category {} not present in huffman table", category);
}

/// JPEG DC magnitude category for a difference value.
fn category_for_diff(diff: i32) -> u8 {
    if diff == 0 {
        return 0;
    }
    let mag = diff.unsigned_abs();
    32 - mag.leading_zeros() as u8
}

/// The "raw" category-width bits to emit for a diff value (inverse of huffman.rs's `extend`).
fn raw_bits_for_diff(diff: i32, category: u8) -> u32 {
    if diff < 0 {
        (diff + ((1i32 << category) - 1)) as u32
    } else {
        diff as u32
    }
}

struct BitWriter {
    buf: Vec<u8>,
    cur: u8,
    nbits: u8,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { buf: Vec::new(), cur: 0, nbits: 0 }
    }

    /// Write `len` bits of `value`, MSB first. Performs standard JPEG byte stuffing (0xFF ->
    /// 0xFF 0x00) on completed bytes, since these are genuine entropy-coded data bytes.
    fn put_bits(&mut self, value: u32, len: u8) {
        for i in (0..len).rev() {
            let bit = ((value >> i) & 1) as u8;
            self.cur = (self.cur << 1) | bit;
            self.nbits += 1;
            if self.nbits == 8 {
                self.buf.push(self.cur);
                if self.cur == 0xFF {
                    self.buf.push(0x00);
                }
                self.cur = 0;
                self.nbits = 0;
            }
        }
    }

    /// Pad the current partial byte with 1-bits to reach a byte boundary, per spec, WITHOUT
    /// byte-stuffing the result even if it comes out to 0xFF - a marker immediately follows,
    /// and the decoder's marker scanner explicitly treats any run of 0xFF "fill bytes" right
    /// before a marker as just that (see huffman.rs::read_bits), not as stuffed data, as long
    /// as we don't insert a spurious 0x00 after it ourselves.
    fn align_before_marker(&mut self) {
        if self.nbits > 0 {
            let pad = 8 - self.nbits;
            let byte = (self.cur << pad) | ((1u8 << pad) - 1);
            self.buf.push(byte);
            self.cur = 0;
            self.nbits = 0;
        }
    }

    fn write_marker(&mut self, marker_byte: u8) {
        self.buf.push(0xFF);
        self.buf.push(marker_byte);
    }

    fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

/// Correct (spec-conformant) predictor for `predictor_selection` at (x, y), given the
/// already-reconstructed neighbours and whether (x, y) is the first MCU after a restart
/// marker (or the very first pixel of the scan).
fn correct_predictor(grid: &[[u8; WIDTH]; HEIGHT], x: usize, y: usize, is_reset: bool) -> i32 {
    const DEFAULT: i32 = 128; // 1 << (precision - point_transform - 1), precision=8, pt=0
    if (x == 0 && y == 0) || is_reset {
        DEFAULT
    } else if y == 0 {
        grid[y][x - 1] as i32
    } else if x == 0 {
        grid[y - 1][x] as i32
    } else {
        let ra = grid[y][x - 1] as i32;
        let rb = grid[y - 1][x] as i32;
        let rc = grid[y - 1][x - 1] as i32;
        ra + rb - rc // predictor selection 4
    }
}

fn build_jpeg() -> Vec<u8> {
    let mut out = Vec::new();

    // SOI
    out.extend_from_slice(&[0xFF, 0xD8]);

    // SOF3 (lossless), 1 component
    out.extend_from_slice(&[0xFF, 0xC3]);
    out.extend_from_slice(&[0x00, 0x0B]); // length = 11 (includes itself)
    out.push(8); // precision
    out.extend_from_slice(&(HEIGHT as u16).to_be_bytes());
    out.extend_from_slice(&(WIDTH as u16).to_be_bytes());
    out.push(1); // component count
    out.push(1); // component id
    out.push(0x11); // sampling factors h=1,v=1
    out.push(0x00); // quantization table index (must be 0 for lossless)

    // DHT: one DC table, index 0
    out.extend_from_slice(&[0xFF, 0xC4]);
    out.extend_from_slice(&[0x00, 0x1F]); // length = 31
    out.push(0x00); // class=0 (DC), index=0
    out.extend_from_slice(&DC_COUNTS);
    out.extend_from_slice(&DC_VALUES);

    // DRI
    out.extend_from_slice(&[0xFF, 0xDD]);
    out.extend_from_slice(&[0x00, 0x04]);
    out.extend_from_slice(&RESTART_INTERVAL.to_be_bytes());

    // SOS
    out.extend_from_slice(&[0xFF, 0xDA]);
    out.extend_from_slice(&[0x00, 0x08]); // length = 8
    out.push(1); // component count in scan
    out.push(1); // component selector
    out.push(0x00); // dc table index 0, ac table index 0 (unused)
    out.push(PREDICTOR_SELECTION); // Ss
    out.push(0x00); // Se (must be 0 for lossless)
    out.push(0x00); // Ah/Al (both 0)

    // Entropy-coded data, mirroring decode_scan_lossless's own restart bookkeeping exactly.
    let mut bw = BitWriter::new();
    let mut mcus_left = RESTART_INTERVAL;
    let mut expected_rst_num: u8 = 0;
    let mut idx = 0usize;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if RESTART_INTERVAL > 0 {
                if mcus_left == 0 {
                    bw.align_before_marker();
                    bw.write_marker(0xD0 + expected_rst_num);
                    expected_rst_num = (expected_rst_num + 1) % 8;
                    mcus_left = RESTART_INTERVAL;
                }
                mcus_left -= 1;
            }

            let is_reset = RESTART_INTERVAL > 0 && idx > 0 && (idx as u16) % RESTART_INTERVAL == 0;
            let prediction = correct_predictor(&GROUND_TRUTH, x, y, is_reset);
            let sample = GROUND_TRUTH[y][x] as i32;
            let diff = sample - prediction;

            let category = category_for_diff(diff);
            let (code, code_len) = huffman_code_for_category(category);
            bw.put_bits(code, code_len);
            if category > 0 {
                let raw = raw_bits_for_diff(diff, category);
                bw.put_bits(raw, category);
            }

            idx += 1;
        }
    }
    bw.align_before_marker();
    out.extend_from_slice(&bw.into_bytes());

    // EOI
    out.extend_from_slice(&[0xFF, 0xD9]);

    out
}

fn main() {
    let jpeg_bytes = build_jpeg();
    println!("Built synthetic lossless JPEG: {} bytes", jpeg_bytes.len());
    println!(
        "Params: {}x{} predictor_selection={} (Ra+Rb-Rc) restart_interval={}",
        WIDTH, HEIGHT, PREDICTOR_SELECTION, RESTART_INTERVAL
    );

    println!("\nGround truth pixel grid:");
    for row in &GROUND_TRUTH {
        println!("  {:?}", row);
    }

    let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(jpeg_bytes));
    let decoded = match decoder.decode() {
        Ok(d) => d,
        Err(e) => {
            println!("\nDecode FAILED with error: {:?}", e);
            std::process::exit(2);
        }
    };

    let info = decoder.info().unwrap();
    println!(
        "\nDecoded image: {}x{}, {} bytes",
        info.width, info.height, decoded.len()
    );

    assert_eq!(decoded.len(), WIDTH * HEIGHT);

    println!("\nDecoded pixel grid:");
    for y in 0..HEIGHT {
        let row: Vec<u8> = decoded[y * WIDTH..(y + 1) * WIDTH].to_vec();
        println!("  {:?}", row);
    }

    let mut mismatches = Vec::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let expected = GROUND_TRUTH[y][x];
            let actual = decoded[y * WIDTH + x];
            if expected != actual {
                mismatches.push((x, y, expected, actual));
            }
        }
    }

    println!(
        "\n{} of {} pixels mismatched between ground truth and decoder output.",
        mismatches.len(),
        WIDTH * HEIGHT
    );
    for (x, y, expected, actual) in &mismatches {
        println!(
            "  mismatch at (x={}, y={}): expected {}, decoder produced {}",
            x, y, expected, actual
        );
    }

    if mismatches.is_empty() {
        // Exit 0 either way: this binary's job is to report the diff, not to encode "the bug
        // should/shouldn't be present" as its own exit code - the justfile checks stdout for the
        // expected marker string in each of its before/after steps instead.
        println!("\nNo mismatch found - image decoded correctly.");
    } else {
        println!(
            "\nBUG REPRODUCED: decode_scan_lossless's generic predict() branch produced wrong \
             pixel values for a valid lossless JPEG combining a restart interval with predictor \
             selection {} (see RESULT.md).",
            PREDICTOR_SELECTION
        );
    }
}
