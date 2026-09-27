// Standalone reproduction for candidate 101, a follow-up to candidate 095.
//
// Candidate 095 fixed `StatefulDecoder::determine_vr_based_on_pixel_representation`
// (parser/src/stateful/decode.rs) so its `Xs` (US/SS) VR override only fires when the
// decoder that produced the header actually guessed the VR from the dictionary
// (implicit VR), never when the wire itself already declared a concrete VR (explicit
// VR). That fix added `DecodeFrom::is_implicit_vr()`, overridden by
// `AdaptiveVRLittleEndianDecoder` (encoding/src/decode/adaptive_le.rs) to read its own
// per-instance `state: Cell<VrState>` - but candidate 095's own session verified that
// override by code reading only, since neither `dicom_object::open_file`/`from_reader`
// nor the transfer-syntax registry select `AdaptiveVRLittleEndianDecoder` from a
// transfer syntax UID alone.
//
// This repro drives the real entry point that *does* select it:
// `dicom_parser::dataset::read::DataSetReader::new_with_ts_options` with
// `DataSetReaderOptions::default().flexible_decoding(true)` - a plain public API of
// the dicom-parser crate, reachable by any external caller that wants tolerant
// reading of non-conformant Explicit-VR-declared-but-Implicit-VR-encoded files,
// without going through dicom-object at all.
//
// A single-process, single-participant repro is appropriate here per
// CONFIRMATION.md section 11: the input is a single external artifact (a hand-crafted
// but internally consistent DICOM data set, exactly the byte layout a real such file
// would have) handed to the parser's own real public entry point - there is no second
// participant being impersonated.

use dicom_core::PrimitiveValue;
use dicom_encoding::TransferSyntaxIndex;
use dicom_parser::dataset::read::DataSetReaderOptions;
use dicom_parser::dataset::{DataSetReader, DataToken};
use dicom_transfer_syntax_registry::TransferSyntaxRegistry;
use std::io::Cursor;

/// Builds a data set consisting of:
///   (0028,0103) Pixel Representation = 1 (signed)
///   (0028,0106) SmallestImagePixelValue = 0xFFFF, dictionary VR `Xs` (US or SS)
///
/// `implicit` selects whether both elements are physically encoded in Implicit VR
/// form (no VR bytes on the wire - the non-conformant case AdaptiveVRLittleEndianDecoder
/// exists for) or Explicit VR form (VR bytes present on the wire, matching what the
/// declared transfer syntax, Explicit VR Little Endian, actually promises).
fn build_dataset(implicit: bool) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend_from_slice(&[0x28, 0x00, 0x03, 0x01]); // (0028,0103)
    if implicit {
        out.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]); // length 2, no VR bytes
    } else {
        out.extend_from_slice(b"US");
        out.extend_from_slice(&[0x02, 0x00]); // length 2
    }
    out.extend_from_slice(&[0x01, 0x00]); // value: 1 (signed)

    out.extend_from_slice(&[0x28, 0x00, 0x06, 0x01]); // (0028,0106)
    if implicit {
        out.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]); // length 2, no VR bytes
    } else {
        out.extend_from_slice(b"US");
        out.extend_from_slice(&[0x02, 0x00]); // length 2
    }
    out.extend_from_slice(&[0xFF, 0xFF]); // value bytes: 0xFFFF

    out
}

/// Runs the data set through the real `flexible_decoding` entry point and returns
/// the decoded VR and value of (0028,0106) SmallestImagePixelValue.
fn decode_smallest_image_pixel_value(implicit: bool) -> (dicom_core::VR, Option<PrimitiveValue>) {
    let bytes = build_dataset(implicit);
    let registry = TransferSyntaxRegistry;
    // Declared transfer syntax UID, exactly as it would be read from a real file's
    // (0002,0010) Transfer Syntax UID element.
    let ts = registry
        .get("1.2.840.10008.1.2.1") // Explicit VR Little Endian
        .expect("Explicit VR LE must be registered");

    let options = DataSetReaderOptions::default().flexible_decoding(true);
    let reader = DataSetReader::new_with_ts_options(Cursor::new(bytes), ts, options)
        .expect("should construct a data set reader");

    let mut vr = None;
    let mut value = None;
    let mut pending_header_is_target = false;
    for token in reader {
        let token = token.expect("should decode token without error");
        match &token {
            DataToken::ElementHeader(header) => {
                pending_header_is_target = header.tag.0 == 0x0028 && header.tag.1 == 0x0106;
                if pending_header_is_target {
                    vr = Some(header.vr());
                }
            }
            DataToken::PrimitiveValue(v) => {
                if pending_header_is_target {
                    value = Some(v.clone());
                    pending_header_is_target = false;
                }
            }
            _ => {}
        }
    }

    (vr.expect("target element must have been decoded"), value)
}

fn main() {
    // Case A: the file declares Explicit VR Little Endian but is actually encoded in
    // Implicit VR (the exact non-conformant scenario AdaptiveVRLittleEndianDecoder
    // exists for). The Xs -> SS override is legitimate here and must keep working.
    let (vr_a, value_a) = decode_smallest_image_pixel_value(true);
    println!("[implicit-actually] decoded VR = {vr_a:?}, value = {value_a:?}");

    // Case B: the file declares Explicit VR Little Endian and really is Explicit VR
    // on the wire (a well-formed data set, just read through a tolerant/defensive
    // caller that always opts into flexible_decoding). The Xs -> SS override must NOT
    // fire here: the wire already says US.
    let (vr_b, value_b) = decode_smallest_image_pixel_value(false);
    println!("[explicit-actually] decoded VR = {vr_b:?}, value = {value_b:?}");

    let case_a_ok = vr_a == dicom_core::VR::SS
        && matches!(&value_a, Some(PrimitiveValue::I16(v)) if v.as_slice() == [-1]);
    let case_b_ok = vr_b == dicom_core::VR::US
        && matches!(&value_b, Some(PrimitiveValue::U16(v)) if v.as_slice() == [0xFFFF]);

    if case_a_ok && case_b_ok {
        println!(
            "did not reproduce - AdaptiveVRLittleEndianDecoder's Explicit-locked state was \
             respected by the Xs override, and its Implicit-locked state still resolves it"
        );
        std::process::exit(0);
    } else {
        eprintln!(
            "REPRODUCED - through DataSetReaderOptions::flexible_decoding(true) \
             (AdaptiveVRLittleEndianDecoder): case A (implicit-actually) ok={case_a_ok} \
             (vr={vr_a:?}, value={value_a:?}), case B (explicit-actually) ok={case_b_ok} \
             (vr={vr_b:?}, value={value_b:?}) - expected case B to keep VR::US/U16([65535]) \
             since the wire itself declared VR::US, but the Xs->SS override fired anyway"
        );
        std::process::exit(1);
    }
}
