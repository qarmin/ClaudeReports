//! Reproduces: `dicom_json`'s `DataElementVisitor::visit_map` (backing the public
//! `dicom_json::from_str`/`from_reader`/`from_slice`/`from_value` DICOM+JSON deserializers)
//! accepts the `"InlineBinary"` JSON field (base64-encoded raw bytes) for ANY declared VR, with
//! no check against PS3.19 Table A.1.5-2's list of VRs actually allowed to use InlineBinary
//! (OB/OD/OF/OL/OV/OW/UN). A JSON data element declaring a text/numeric VR (e.g. "DS") together
//! with "InlineBinary" deserializes successfully into a `PrimitiveValue::U8` (raw bytes) even
//! though its declared VR says "DS" - a type-confused, spec-violating in-memory element that
//! nothing in the public API rejects.
//!
//! This also demonstrates the downstream effect of writing such an element to a real DICOM file
//! via the crate's own public `write_to_file`/`open_file` round trip: no panic occurs, but the
//! file written to disk (and read back) silently no longer contains the raw bytes the JSON
//! payload specified - the value is silently re-interpreted along the way, which is itself
//! further evidence that VR and actual value type are assumed to be consistent throughout the
//! crate, an invariant this deserializer alone fails to enforce.
use dicom_core::Tag;
use dicom_object::{FileMetaTableBuilder, InMemDicomObject, open_file};

fn main() {
    let json = r#"{"00100010":{"vr":"DS","InlineBinary":"AAE="}}"#;
    println!("input JSON (DICOM+JSON, PS3.18): {json}");

    let obj: InMemDicomObject = match dicom_json::from_str(json) {
        Ok(obj) => obj,
        Err(e) => {
            println!("correctly rejected: {e}");
            println!("BUG NOT PRESENT");
            return;
        }
    };

    let tag = Tag(0x0010, 0x0010);
    let elem = obj.element(tag).unwrap();
    println!("declared VR = {:?}", elem.header().vr());
    println!("actual in-memory value = {:?}", elem.value());
    println!(
        "BUG REPRODUCED: a \"DS\" (Decimal String, text) element was deserialized with an \
         actual PrimitiveValue::U8 (raw byte) payload via InlineBinary, which PS3.19 Table \
         A.1.5-2 does not permit for VR DS."
    );

    // Downstream effect: write this type-confused object to a real DICOM file through the
    // crate's own public writer, then read it back through the crate's own public reader.
    let file_object = obj
        .with_meta(
            FileMetaTableBuilder::default()
                .transfer_syntax("1.2.840.10008.1.2.1")
                .media_storage_sop_class_uid("1.2.840.10008.5.1.4.1.1.1")
                .media_storage_sop_instance_uid("1.4.645.212121"),
        )
        .expect("with_meta should succeed");

    let path = std::env::temp_dir().join("candidate084_repro.dcm");
    file_object
        .write_to_file(&path)
        .expect("write_to_file should not panic");
    println!("write_to_file: OK (no panic)");

    let readback = open_file(&path).expect("open_file should not panic");
    let elem2 = readback.element(tag).unwrap();
    println!("readback VR = {:?}", elem2.header().vr());
    println!("readback value = {:?}", elem2.value());
    println!(
        "note: the on-disk bytes for this DS element are the *decimal-text rendering* of the \
         original raw bytes [0, 1] (\"0\\\\1\"), not the raw bytes themselves - a silent, \
         VR-driven reinterpretation happening one layer further down in the encoder \
         (parser/src/stateful/encode.rs's DS/IS special case), independent of and downstream \
         from the deserializer bug demonstrated above."
    );
}
