//! Standalone reproduction for candidate 123: GitHub issue `Enet4/dicom-rs#169`,
//! "Could not read data set token", reported on a real CT file (Implicit VR
//! Little Endian) with several private, undefined-length, UN-VR elements in
//! a row under group `(2001,xxxx)`/`(2005,xxxx)` - dcmtk reads it fine per
//! CP-246, dicom-rs did not (before this candidate's patch, dicom-rs failed
//! with a confusing generic I/O error; after the patch it fails with a
//! clear, specific error - see RESULT.md for why full interoperability
//! parity with dcmtk was judged out of scope for a bounded fix).
//!
//! The original file from the issue isn't available (filed 2021, attachment
//! not reachable without network access), so this independently constructs
//! the same byte shape from first principles: dicom-rs's own CP-246
//! mechanism (`parser/src/dataset/read.rs`, "treat any other undefined
//! length element as a sequence, discarding the VR") converts an
//! undefined-length UN element into a pseudo-sequence containing one
//! explicit-length item. That's correct when the item's content really is
//! a nested data set (dicom-rs already has a passing test for that,
//! `read_dataset_in_dataset`). But CP-246 private elements are just as
//! often *opaque binary blobs* with no sub-element structure at all - and
//! dicom-rs unconditionally tries to decode the item's content as more
//! element headers anyway. When the raw bytes happen to look like a
//! plausible tag+length pair (not at all unlikely for real pixel-derived
//! private data), the derived length is nonsense and driving a read that
//! overruns the item (and often the whole file), surfacing only as an
//! opaque `UnexpectedEof` - which is exactly the "Could not read data set
//! token" message from the original report.

use dicom_core::header::HasLength;
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::IMPLICIT_VR_LITTLE_ENDIAN;

/// Returns `true` if this input hit the *original, confusing* form of the
/// bug (a generic I/O / "UnexpectedEof" error with no indication of what
/// actually went wrong) - `false` if it either parsed fine, or failed with
/// the new, clear `ElementLengthExceedsContainer` error introduced by this
/// candidate's patch.
///
/// Note: the patch does NOT make any of the triggering variations parse
/// successfully - it only replaces a misleading generic error with an
/// accurate, specific one. See RESULT.md for why a fix that actually
/// achieves dcmtk-level interoperability (correctly guessing raw-bytes vs.
/// nested-data-set item content) was judged out of scope here.
fn try_read(label: &str, data: &[u8]) -> bool {
    let ts = IMPLICIT_VR_LITTLE_ENDIAN.erased();
    println!("--- {label} ({} bytes) ---", data.len());
    match InMemDicomObject::read_dataset_with_ts(data, &ts) {
        Ok(obj) => {
            println!(
                "OK: parsed {} top-level elements (no failure)",
                obj.iter().count()
            );
            for elem in obj.iter() {
                println!(
                    "  tag {} vr {} len {:?}",
                    elem.header().tag,
                    elem.header().vr(),
                    elem.header().length()
                );
            }
            false
        }
        Err(e) => {
            let debug = format!("{e:?}");
            println!("ERROR: {e}");
            println!("ERROR (debug): {debug}");
            let is_confusing_generic_error = !debug.contains("ElementLengthExceedsContainer");
            if is_confusing_generic_error {
                println!("  (this is the ORIGINAL bug's confusing generic error)");
            } else {
                println!("  (this is the patched, clear, specific error)");
            }
            is_confusing_generic_error
        }
    }
}

fn main() {
    // Variation A: a single private UN, undefined-length element
    // (2005,1080) whose CP-246 item content is 16 bytes of opaque binary
    // data (here: two copies of the little-endian IEEE-754 bytes of pi) -
    // not a nested data set. The first 8 bytes of that opaque payload are
    // misread as a bogus element header with an oversized length.
    #[rustfmt::skip]
    let variation_a: &[u8] = &[
        0x05, 0x20, 0x80, 0x10, // tag (2005,1080)
        0xff, 0xff, 0xff, 0xff, // length: undefined
        0xfe, 0xff, 0x00, 0xe0, // item start tag
        0x10, 0x00, 0x00, 0x00, // item length: 16 (explicit)
        // opaque private payload: two copies of the IEEE-754 bytes of pi
        0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40,
        0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40,
        0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00, // sequence delimiter
    ];

    // Variation C: several private UN, undefined-length elements in a row
    // (matching the issue's literal description) - the first one has a
    // small, harmless opaque payload, the second is variation A's
    // triggering payload.
    #[rustfmt::skip]
    let variation_c: &[u8] = &[
        // element 1: (2001,0010) private, small harmless opaque payload
        0x01, 0x20, 0x10, 0x00,
        0xff, 0xff, 0xff, 0xff,
        0xfe, 0xff, 0x00, 0xe0,
        0x08, 0x00, 0x00, 0x00,
        b'A', b'C', b'M', b'E', b' ', b' ', b' ', b' ',
        0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00,
        // element 2: (2005,1080), same triggering payload as variation A
        0x05, 0x20, 0x80, 0x10,
        0xff, 0xff, 0xff, 0xff,
        0xfe, 0xff, 0x00, 0xe0,
        0x10, 0x00, 0x00, 0x00,
        0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40,
        0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40,
        0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00,
    ];

    // Variation D: same as A, preceded by an ordinary top-level element, to
    // rule out any dependency on being the very first thing in the stream.
    #[rustfmt::skip]
    let variation_d: &[u8] = &[
        0x08, 0x00, 0x60, 0x00, // tag (0008,0060) Modality
        0x02, 0x00, 0x00, 0x00, // length 2
        b'C', b'T',
        0x05, 0x20, 0x80, 0x10,
        0xff, 0xff, 0xff, 0xff,
        0xfe, 0xff, 0x00, 0xe0,
        0x10, 0x00, 0x00, 0x00,
        0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40,
        0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40,
        0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00,
    ];

    // Control: an all-zero opaque payload happens to decode as a chain of
    // valid-looking zero-length (0000,0000) elements that exactly fill the
    // item - included to show the bug is payload-content-dependent, not a
    // blanket "CP-246 items never parse" failure.
    #[rustfmt::skip]
    let control_all_zero: &[u8] = &[
        0x05, 0x20, 0x80, 0x10,
        0xff, 0xff, 0xff, 0xff,
        0xfe, 0xff, 0x00, 0xe0,
        0x10, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00,
    ];

    let mut reproduced = false;
    reproduced |= try_read("Variation A: opaque payload misread as element header", variation_a);
    reproduced |= try_read("Variation C: several private UN elements in a row", variation_c);
    reproduced |= try_read("Variation D: preceded by a normal element", variation_d);
    let control_failed = try_read("Control: all-zero opaque payload (expected OK)", control_all_zero);

    if reproduced {
        println!(
            "\nBUG REPRODUCED: a CP-246-style private UN undefined-length element's opaque \
             item payload was misread as a nested element header, causing a spurious parse \
             failure (\"Could not read data set token\" once wrapped by dicom-object)."
        );
    }
    if control_failed {
        println!("\nNOTE: control case (expected to parse OK) unexpectedly failed too.");
    }

    std::process::exit(if reproduced && !control_failed { 1 } else { 0 });
}
