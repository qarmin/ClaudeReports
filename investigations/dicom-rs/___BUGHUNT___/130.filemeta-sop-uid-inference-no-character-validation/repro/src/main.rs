//! Real end-to-end reproduction, through the real public read entry point
//! (`dicom_object::from_reader`, which internally calls `FileDicomObject::
//! read_parts_with_all_options_impl`'s SOP-UID-inference step in `object/src/mem.rs`), of a gap
//! left open by candidates 053/054 (which fixed this exact code's missing *padding*, but never
//! added any character-repertoire validation) and candidate 077 (which added UID-character
//! validation, but only to `object/src/meta.rs`'s own file-meta-group parsing loop, not to this
//! separate inference call site in `mem.rs`).
//!
//! Builds a real, well-formed-except-for-this-one-thing file: a file meta group that
//! deliberately omits Media Storage SOP Class UID (triggering the inference fallback), and a
//! main data set whose own SOP Class UID (0008,0016) - a completely ordinary, real dataset
//! element, not itself part of the file meta group's own repertoire-checked read path - contains
//! characters PS3.5 6.2's UI (Unique Identifier) VR never permits (only "0"-"9", ".", and
//! trailing NUL padding are allowed). This is constructed entirely through the crate's own
//! public API (`FileMetaTableBuilder`, `InMemDicomObject::put_element`), then handed through the
//! real public `dicom_object::from_reader` entry point - the same one any real caller uses to
//! open a DICOM file.
//!
//! Single external artifact (a real file whose own dataset element already contains
//! non-conformant characters - the same shape a malformed/adversarial real file could have), no
//! second participant to fake - CONFIRMATION.md section 11 does not apply.

use dicom_core::{DataElement, VR};
use dicom_dictionary_std::{tags, uids};
use dicom_object::{FileDicomObject, FileMetaTableBuilder};

fn is_uid_conformant(s: &str) -> bool {
    s.chars()
        .all(|c| c == '.' || c.is_ascii_digit() || c == '\0' || c.is_ascii_whitespace())
}

fn main() {
    // Step 1: build a real object with a NON-conformant SOP Class UID in the main data set
    // (contains letters and an exclamation mark - never valid in a UI VR value), and a file meta
    // group that omits Media Storage SOP Class UID entirely (so the inference fallback fires).
    let mut obj = FileDicomObject::new_empty_with_meta(
        FileMetaTableBuilder::new()
            .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
            .media_storage_sop_instance_uid("2.25.1")
            // deliberately NOT calling .media_storage_sop_class_uid(..)
            .build()
            .unwrap(),
    );
    let bogus_sop_class_uid = "1.2.NOT-A-REAL-UID!\0";
    assert!(
        !is_uid_conformant(bogus_sop_class_uid),
        "sanity: the test value must itself be non-conformant"
    );
    obj.put_element(DataElement::new(
        tags::SOP_CLASS_UID,
        VR::UI,
        dicom_core::PrimitiveValue::from(bogus_sop_class_uid),
    ));

    // Step 2: write it out through the real public writer.
    let mut bytes = Vec::new();
    obj.write_all(&mut bytes).expect("write_all should succeed");

    // Step 3: read it back through the real public entry point any caller would use - this is
    // where the SOP-UID-inference fallback (object/src/mem.rs) runs.
    let obj2 = dicom_object::from_reader(bytes.as_slice()).expect("from_reader should succeed");

    let inferred = obj2.meta().media_storage_sop_class_uid();
    println!("inferred Media Storage SOP Class UID: {:?}", inferred);
    println!("is UID-repertoire-conformant (PS3.5 6.2, digits/./NUL/whitespace only): {}", is_uid_conformant(inferred));

    // Step 4: show the actual written bytes for the file meta group's own (0002,0002) element,
    // proving the non-conformant characters are really being placed into the formal Media
    // Storage SOP Class UID *file meta* field on disk, not just held in memory.
    let mut bytes2 = Vec::new();
    obj2.write_all(&mut bytes2).expect("second write_all should succeed");
    let needle = b"\x02\x00\x02\x00UI";
    if let Some(pos) = bytes2.windows(needle.len()).position(|w| w == needle) {
        let len_start = pos + needle.len();
        let len = u16::from_le_bytes([bytes2[len_start], bytes2[len_start + 1]]) as usize;
        let val_start = len_start + 2;
        let val = &bytes2[val_start..val_start + len];
        println!(
            "on-disk (0002,0002) Media Storage SOP Class UID bytes: {:?} = {:?}",
            val,
            String::from_utf8_lossy(val)
        );
    }

    let bug_reproduces = !is_uid_conformant(inferred);
    println!();
    println!(
        "BUG (file meta group's Media Storage SOP Class UID ends up containing characters PS3.5 \
         never permits in a UI VR value, silently copied verbatim from the data set by the \
         SOP-UID-inference fallback with no character validation): {bug_reproduces}"
    );

    if bug_reproduces {
        std::process::exit(1);
    }
}
