// Standalone reproduction for the "predict() guard off-by-one" bug in
// `jpeg_decoder::decoder::lossless::predict`'s H.1.2.1 default-prediction branch
// (src/decoder/lossless.rs, ~line 199-206).
//
// `predict()` guards the default-prediction shift with:
//     if input_precision > 1 + point_transform { 1 << (input_precision - point_transform - 1) }
//     else { 0 }
// For the still-valid, still-accepted case `precision == point_transform + 1` (e.g.
// precision=8, point_transform=7), this guard takes the `else` branch and returns 0, whereas
// the spec-correct (ITU-T T.81 H.1.2.1) default prediction is `1 << (precision - Pt - 1)` = 1.
// `src/parser.rs`'s `parse_sos` independently rejects `point_transform >= frame.precision`
// (commit 95550d7, #282), so `precision - point_transform >= 1` always holds by the time
// `predict()` runs - the underflow this guard was originally added to avoid can no longer
// happen, but its threshold (`> 1 + point_transform` instead of `> point_transform`) is one
// notch too strict and misfires on this legal boundary value.
//
// There is no lossless JPEG encoder available (jpeg-decoder is decode-only), so this program
// hand-builds a minimal, spec-valid lossless JPEG bitstream byte by byte: a 1x1-pixel, single
// -component, SOF3 (lossless) image, precision=8, point_transform=7 (Pt = precision-1, the
// legal boundary the parser now explicitly allows), predictor selection 0 (NoPrediction,
// exercising the generic/non-`Ra` branch of `decode_scan_lossless`), one DC Huffman table
// encoding a single "difference = 0" symbol. With a single pixel at (0,0) and no restart
// interval, the only thing under test is the "start of first line" branch of `predict()`.
//
// Ground truth: per H.1.2.1, the top-left pixel's default prediction is
// `1 << (precision - Pt - 1)` = `1 << 0` = 1; with a decoded DC difference of 0, the
// reconstructed sample (before the `<< point_transform` expansion) is 1, so the final 8-bit
// decoded pixel is `1 << 7` = 128. The buggy decoder instead predicts 0, giving a final pixel
// of 0.

use std::io::Cursor;

const PRECISION: u8 = 8;
const POINT_TRANSFORM: u8 = 7; // Pt = precision - 1, the boundary this guard mishandles
const PREDICTOR_SELECTION_SS: u8 = 0; // NoPrediction -> exercises the generic (non-Ra) branch

const EXPECTED_PIXEL: u8 = 128; // 1 << (precision - Pt - 1) == 1, then << Pt == 128

struct BitWriter {
    buf: Vec<u8>,
    cur: u8,
    nbits: u8,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { buf: Vec::new(), cur: 0, nbits: 0 }
    }

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

    fn align_before_marker(&mut self) {
        if self.nbits > 0 {
            let pad = 8 - self.nbits;
            let byte = (self.cur << pad) | ((1u8 << pad) - 1);
            self.buf.push(byte);
            self.cur = 0;
            self.nbits = 0;
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

fn build_jpeg() -> Vec<u8> {
    let mut out = Vec::new();

    // SOI
    out.extend_from_slice(&[0xFF, 0xD8]);

    // SOF3 (lossless), 1 component, 1x1 pixels
    out.extend_from_slice(&[0xFF, 0xC3]);
    out.extend_from_slice(&[0x00, 0x0B]); // length = 11 (includes itself)
    out.push(PRECISION);
    out.extend_from_slice(&1u16.to_be_bytes()); // height = 1
    out.extend_from_slice(&1u16.to_be_bytes()); // width = 1
    out.push(1); // component count
    out.push(1); // component id
    out.push(0x11); // sampling factors h=1,v=1
    out.push(0x00); // quantization table index (must be 0 for lossless)

    // DHT: one DC table, index 0, single symbol "category 0" at code length 1 (code 0b0)
    out.extend_from_slice(&[0xFF, 0xC4]);
    out.extend_from_slice(&[0x00, 0x14]); // length = 20 (2 + 1 + 16 + 1)
    out.push(0x00); // class=0 (DC), index=0
    let counts: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    out.extend_from_slice(&counts);
    out.push(0); // the single value: category 0 (difference == 0)

    // SOS
    out.extend_from_slice(&[0xFF, 0xDA]);
    out.extend_from_slice(&[0x00, 0x08]); // length = 8
    out.push(1); // component count in scan
    out.push(1); // component selector
    out.push(0x00); // dc table index 0, ac table index 0 (unused)
    out.push(PREDICTOR_SELECTION_SS); // Ss = predictor selection = 0 (NoPrediction)
    out.push(0x00); // Se (must be 0 for lossless)
    out.push(POINT_TRANSFORM); // Ah=0 (high nibble), Al=point_transform (low nibble)

    // Entropy-coded data: single symbol, code 0b0 (1 bit) -> DC difference category 0 -> diff=0
    let mut bw = BitWriter::new();
    bw.put_bits(0b0, 1);
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
        "Params: 1x1 precision={} point_transform={} predictor_selection_Ss={} (NoPrediction)",
        PRECISION, POINT_TRANSFORM, PREDICTOR_SELECTION_SS
    );

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
    assert_eq!(decoded.len(), 1);

    let actual = decoded[0];
    println!(
        "\nExpected top-left pixel (H.1.2.1 default prediction, precision=8, Pt=7): {}",
        EXPECTED_PIXEL
    );
    println!("Decoder produced: {}", actual);

    if actual == EXPECTED_PIXEL {
        // Exit 0 either way: this binary's job is to report the diff, not to encode "the bug
        // should/shouldn't be present" as its own exit code - the justfile checks stdout for the
        // expected marker string in each of its before/after steps instead.
        println!("\nNo mismatch found - image decoded correctly.");
    } else {
        println!(
            "\nBUG REPRODUCED: predict()'s default-prediction guard (`input_precision > 1 + \
             point_transform`) misfires at precision={} point_transform={} (precision == Pt+1), \
             returning 0 instead of 1, producing pixel {} instead of the spec-correct {}.",
            PRECISION, POINT_TRANSFORM, actual, EXPECTED_PIXEL
        );
    }
}
