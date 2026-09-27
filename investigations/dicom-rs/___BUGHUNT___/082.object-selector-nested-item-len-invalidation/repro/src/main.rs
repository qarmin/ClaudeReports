//! Reproduces: `InMemDicomObject::update_value_at`/`entry_at_mut`'s selector-based walk into a
//! nested sequence item never invalidates the ITEM's own cached `len` field when mutating a leaf
//! further down - only the root object's `len` gets invalidated. When the caller opts into
//! `ExplicitLengthSqItemStrategy::NoChange` on write (the crate's default, `SetUndefined`, already
//! sidesteps this), the stale item length is written verbatim, producing a data set whose item
//! boundary no longer matches its actual encoded content - detected here by round-tripping the
//! writer's own output back through the crate's own real reader.
use dicom_core::header::Tag;
use dicom_core::value::{PrimitiveValue, Value};
use dicom_object::InMemDicomObject;
use dicom_parser::dataset::write::{DataSetWriterOptions, ExplicitLengthSqItemStrategy};
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};

fn main() {
    let ts = TransferSyntaxRegistry.get("1.2.840.10008.1.2.1").unwrap();

    // Hand-built Explicit VR Little Endian bytes representing a single SQ element
    // (tag 0008,1140) with undefined length, containing exactly one item of
    // EXPLICIT (defined) length 10, whose only content is PatientName="AB" (2 bytes).
    // This is an ordinary, well-formed data set - explicit item lengths are a completely
    // legal (if less common) encoding choice, not malformed input.
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(&[0x08, 0x00, 0x40, 0x11]); // tag (0008,1140)
    bytes.extend_from_slice(b"SQ");
    bytes.extend_from_slice(&[0x00, 0x00]); // reserved
    bytes.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes()); // undefined length

    // Item header: tag (FFFE,E000) + length(4) = 10 (defined)
    bytes.extend_from_slice(&[0xFE, 0xFF, 0x00, 0xE0]);
    bytes.extend_from_slice(&10u32.to_le_bytes());

    // Item content: PatientName (0010,0010) VR=PN, length=2, value="AB"
    bytes.extend_from_slice(&[0x10, 0x00, 0x10, 0x00]);
    bytes.extend_from_slice(b"PN");
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(b"AB");

    // Sequence delimitation item: tag (FFFE,E0DD) + length(4) = 0
    bytes.extend_from_slice(&[0xFE, 0xFF, 0xDD, 0xE0]);
    bytes.extend_from_slice(&0u32.to_le_bytes());

    println!("input bytes: {} bytes, item declared len = 10", bytes.len());

    let mut obj = InMemDicomObject::read_dataset_with_ts(bytes.as_slice(), ts)
        .expect("should parse the well-formed input dataset");

    {
        let entry = obj
            .entry_at((Tag(0x0008, 0x1140), 0u32, Tag(0x0010, 0x0010)))
            .unwrap();
        println!("leaf value before mutation: {:?}", entry.value());
    }

    // Mutate the nested leaf via the public selector-based API, growing its encoded size
    // well past what fits in the item's originally-declared 10-byte length.
    obj.update_value_at((Tag(0x0008, 0x1140), 0u32, Tag(0x0010, 0x0010)), |e| {
        *e = Value::Primitive(PrimitiveValue::from("ABCDEFGHIJKLMNOP"));
    })
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
        }
    }
}
