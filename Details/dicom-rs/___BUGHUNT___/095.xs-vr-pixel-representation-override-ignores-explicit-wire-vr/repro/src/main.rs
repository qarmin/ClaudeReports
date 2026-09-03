// Standalone reproduction for candidate 095: the "Xs" (US/SS) VR resolution in
// `parser/src/stateful/decode.rs` overrides a data element's VR to `SS` whenever
// _Pixel Representation_ (0028,0103) is signed and the tag's dictionary VR is
// `VirtualVr::Xs` - unconditionally, even under an Explicit VR transfer syntax where
// the wire itself already carries a concrete, unambiguous VR for that element.
//
// This builds a real, well-formed DICOM P10 file (file meta + dataset) entirely
// through dicom-object's own public API (`InMemDicomObject`/`with_meta`/
// `write_to_file`), so the VR that ends up on the wire for tag (0028,0106)
// "SmallestImagePixelValue" is genuinely VR::US (verified by scanning the written
// bytes below, not assumed) - then reads that exact file back through the public
// `dicom_object::open_file` entry point, the same one any real caller uses to open
// a DICOM file from disk.
//
// A single-process, single-participant repro is appropriate here per
// CONFIRMATION.md section 11: the input is a single external artifact (a malformed
// or, as here, perfectly well-formed DICOM file) handed to the parser's real public
// `open_file` entry point - there is no second participant being impersonated.

use dicom_core::header::{DataElement, Tag};
use dicom_core::{PrimitiveValue, VR};
use dicom_object::{FileMetaTableBuilder, InMemDicomObject, open_file};

fn build_and_write(path: &std::path::Path) {
    let sop_uid = "1.4.999.888777";
    let mut obj = InMemDicomObject::new_empty();

    // Pixel Representation = 1 (signed)
    obj.put(DataElement::new(
        Tag(0x0028, 0x0103),
        VR::US,
        PrimitiveValue::U16(smallvec::smallvec![1]),
    ));

    // SmallestImagePixelValue (0028,0106) has dictionary VR "Xs" (US or SS,
    // resolved by context). The file *explicitly* declares it as VR::US on the
    // wire - that's the whole point of Explicit VR.
    obj.put(DataElement::new(
        Tag(0x0028, 0x0106),
        VR::US,
        PrimitiveValue::U16(smallvec::smallvec![0xFFFF]),
    ));

    let file_object = obj
        .with_meta(
            FileMetaTableBuilder::default()
                .transfer_syntax("1.2.840.10008.1.2.1") // Explicit VR Little Endian
                .media_storage_sop_class_uid("1.2.840.10008.5.1.4.1.1.1")
                .media_storage_sop_instance_uid(sop_uid),
        )
        .unwrap();

    file_object.write_to_file(path).unwrap();
}

fn main() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("repro.dcm");

    build_and_write(&file_path);

    // Sanity check: confirm the bytes actually on disk really declare VR::US for
    // tag (0028,0106) - i.e. this is genuinely an Explicit VR wire encoding, not
    // an assumption about what the writer produced.
    let raw = std::fs::read(&file_path).unwrap();
    let needle = [0x28, 0x00, 0x06, 0x01, b'U', b'S'];
    let wire_declares_us = raw.windows(6).any(|w| w == needle);
    println!("wire bytes explicitly declare VR::US for (0028,0106): {wire_declares_us}");
    if !wire_declares_us {
        eprintln!("SETUP FAILURE: expected wire encoding not found, repro is invalid");
        std::process::exit(2);
    }

    // Now read the exact same file back through the real public API.
    let saved_object = open_file(&file_path).unwrap();
    let elem = saved_object.element(Tag(0x0028, 0x0106)).unwrap();

    println!("decoded header VR = {:?}", elem.header().vr);
    println!("decoded value     = {:?}", elem.value());

    if elem.header().vr == VR::US {
        println!("did not reproduce - decoder respected the file's own explicit VR::US declaration");
        std::process::exit(0);
    } else {
        eprintln!(
            "REPRODUCED - decoder silently overrode the file's own explicit VR::US declaration \
             to {:?} (value reinterpreted as {:?} instead of the wire-declared unsigned 65535)",
            elem.header().vr,
            elem.value()
        );
        std::process::exit(1);
    }
}
