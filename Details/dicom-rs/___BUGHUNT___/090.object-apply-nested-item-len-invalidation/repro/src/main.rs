//! Reproduces: `InMemDicomObject::apply` (the `dicom_core::ops::ApplyOp`/`AttributeOp`-based
//! mutation entry point) walks an `AttributeSelector` down into nested sequence items by
//! reassigning a local `obj` binding at each `Nested` step, but never invalidates that
//! intermediate item's own cached `len: Length` field. Only the innermost (leaf-owning) object's
//! `len` gets invalidated (by `apply_leaf`'s mutating arms) - any ancestor item traversed on the
//! way to the leaf keeps its stale length. When the caller opts into
//! `ExplicitLengthSqItemStrategy::NoChange` on write (the crate's default, `SetUndefined`, already
//! sidesteps this), that stale ancestor length is written verbatim, producing a data set whose
//! item boundary no longer matches its actual encoded content - detected here by round-tripping
//! the writer's own output back through the crate's own real reader.
//!
//! This is the same defect class already confirmed and fixed for `update_value_at`/`entry_at_mut`
//! in candidate 082 (`object-selector-nested-item-len-invalidation`) - `apply` has its own,
//! separate traversal loop that 082's fix did not touch.
use dicom_core::header::Tag;
use dicom_core::ops::{ApplyOp, AttributeAction, AttributeOp};
use dicom_object::InMemDicomObject;
use dicom_parser::dataset::write::{DataSetWriterOptions, ExplicitLengthSqItemStrategy};
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};

fn main() {
    let ts = TransferSyntaxRegistry.get("1.2.840.10008.1.2.1").unwrap();

    // Two levels of nesting: outer SQ (0008,1140) -> item0 (explicit len) ->
    // inner SQ (0008,1141) -> item1 (explicit len) -> leaf PatientName (0010,0010).
    // This mirrors a real-world "sequence of sequences" structure (e.g. a sequence
    // nested inside another sequence's item). All lengths are legal, well-formed
    // Explicit VR Little Endian encodings - explicit (defined) item lengths are a
    // completely ordinary, if less common, encoding choice, not malformed input.
    let mut bytes: Vec<u8> = Vec::new();

    // outer SQ (0008,1140), undefined length
    bytes.extend_from_slice(&[0x08, 0x00, 0x40, 0x11]);
    bytes.extend_from_slice(b"SQ");
    bytes.extend_from_slice(&[0x00, 0x00]); // reserved
    bytes.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes());

    // item0 header, EXPLICIT length (patched in below once content is known)
    let item0_header_pos = bytes.len();
    bytes.extend_from_slice(&[0xFE, 0xFF, 0x00, 0xE0]);
    bytes.extend_from_slice(&0u32.to_le_bytes()); // placeholder, patched below
    let item0_content_start = bytes.len();

    // inner SQ (0008,1141), undefined length
    bytes.extend_from_slice(&[0x08, 0x00, 0x41, 0x11]);
    bytes.extend_from_slice(b"SQ");
    bytes.extend_from_slice(&[0x00, 0x00]); // reserved
    bytes.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes());

    // item1 header, EXPLICIT length = 10
    bytes.extend_from_slice(&[0xFE, 0xFF, 0x00, 0xE0]);
    bytes.extend_from_slice(&10u32.to_le_bytes());

    // item1 content: PatientName (0010,0010) VR=PN, length=2, value="AB"
    bytes.extend_from_slice(&[0x10, 0x00, 0x10, 0x00]);
    bytes.extend_from_slice(b"PN");
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(b"AB");

    // inner SQ delimiter
    bytes.extend_from_slice(&[0xFE, 0xFF, 0xDD, 0xE0]);
    bytes.extend_from_slice(&0u32.to_le_bytes());

    let item0_content_len = (bytes.len() - item0_content_start) as u32;
    bytes[item0_header_pos + 4..item0_header_pos + 8]
        .copy_from_slice(&item0_content_len.to_le_bytes());

    // outer SQ delimiter
    bytes.extend_from_slice(&[0xFE, 0xFF, 0xDD, 0xE0]);
    bytes.extend_from_slice(&0u32.to_le_bytes());

    println!(
        "input bytes: {} bytes, item0 declared len = {}",
        bytes.len(),
        item0_content_len
    );

    let mut obj = InMemDicomObject::read_dataset_with_ts(bytes.as_slice(), ts)
        .expect("should parse the well-formed input dataset");

    // Mutate the doubly-nested leaf via the public AttributeOp/apply API, growing its
    // encoded size well past what fits in item0's originally-declared 38-byte length.
    obj.apply(AttributeOp::new(
        (
            Tag(0x0008, 0x1140),
            0u32,
            Tag(0x0008, 0x1141),
            0u32,
            Tag(0x0010, 0x0010),
        ),
        AttributeAction::SetStr("ABCDEFGHIJKLMNOP".into()),
    ))
    .expect("selector should resolve");

    let mut out = Vec::new();
    let options = DataSetWriterOptions::default()
        .explicit_length_sq_item_strategy(ExplicitLengthSqItemStrategy::NoChange);
    obj.write_dataset_with_ts_options(&mut out, ts, options)
        .expect("write should succeed regardless of the bug");

    println!("output bytes: {} bytes", out.len());

    // Re-parse what was just written, using the same real reading entry point, to show the
    // corruption is actually observable, not just "a stale field sits unused".
    match InMemDicomObject::read_dataset_with_ts(out.as_slice(), ts) {
        Ok(reparsed) => {
            println!("re-parsed successfully (bug not present): {reparsed:?}");
        }
        Err(e) => {
            println!("BUG REPRODUCED: re-parsing the writer's own output failed: {e}");
            let mut source = std::error::Error::source(&e);
            while let Some(s) = source {
                println!("  caused by: {s}");
                source = std::error::Error::source(s);
            }
        }
    }
}
