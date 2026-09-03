//! Real, end-to-end reproduction: an `InMemDicomObject` built entirely through the crate's
//! own public API, containing one `DataElement` whose VR is `DS` (Decimal String) but whose
//! stored value is `PrimitiveValue::Date` - a mismatch the type system never prevents (see the
//! recurring "a DataElement's VR and its stored value are never kept in sync" class documented
//! in this project's own risk map, candidates 025/026/092/093/094/095). Written out via the
//! real public `write_all`/`write_dataset_with_ts` path (the same one any real caller uses),
//! this reaches `parser/src/stateful/encode.rs`'s `encode_primitive_element` -> (VR is DS, so)
//! `encode_element_as_text`, whose own `match value` has no arm for `PrimitiveValue::Date`
//! (only `Str`/`Strs`/numeric variants) and falls into a bare `unreachable!()`.

use dicom_core::value::{DicomDate, PrimitiveValue};
use dicom_core::{DataElement, Tag, VR};
use dicom_dictionary_std::uids;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn main() {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.555555555555555555555555555555555555555")
        .build()
        .unwrap();

    let mut obj = FileDicomObject::new_empty_with_meta(meta);

    // A private-looking tag, wire VR explicitly declared DS, but the stored value is a Date -
    // exactly the kind of VR/value mismatch nothing in the public API prevents.
    let date = DicomDate::from_ymd(2024, 1, 1).expect("valid date");
    obj.put_element(DataElement::new(
        Tag(0x0009, 0x0010),
        VR::DS,
        PrimitiveValue::Date(dicom_core::smallvec::smallvec![date]),
    ));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut bytes = Vec::new();
        obj.write_all(&mut bytes).map(|_| bytes)
    }));

    match result {
        Ok(Ok(bytes)) => {
            println!("No panic - wrote {} bytes successfully (unexpected)", bytes.len());
            std::process::exit(2);
        }
        Ok(Err(e)) => {
            println!("No panic, but a graceful error was returned instead: {e}");
        }
        Err(payload) => {
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            println!("PANICKED: {message}");
            println!(
                "BUG REPRODUCED: writing a DS-VR element whose stored value is a Date hit an \
                 unreachable!() in encode_element_as_text instead of a graceful error."
            );
            std::process::exit(1);
        }
    }
}
