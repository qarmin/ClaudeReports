//! Real, end-to-end reproduction: an `InMemDicomObject` built entirely through the crate's own
//! public API, containing one `DataElement` whose VR is `PN` (an ordinary text VR, not `SQ`) but
//! whose stored value is `Value::Sequence` - a mismatch the type system never prevents (the same
//! recurring "a DataElement's VR and its stored value are never kept in sync" class as candidates
//! 025/026/092/093/094/095/120). `impl From<DataElementHeader> for DataToken`
//! (`parser/src/dataset/mod.rs`) classifies purely from `(vr, tag)`, independently of the actual
//! `Value` - a non-SQ VR always classifies as an ordinary `ElementHeader`/`DataElementTokens::Header`
//! state, whose own `match elem.into_value()` has no arm for `Value::Sequence`/`Value::PixelSequence`,
//! falling into a bare `unreachable!()`.

use dicom_core::value::{DataSetSequence, Value};
use dicom_core::{DataElement, Length, Tag, VR, smallvec::smallvec};
use dicom_dictionary_std::uids;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn main() {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.666666666666666666666666666666666666666")
        .build()
        .unwrap();

    let mut obj = FileDicomObject::new_empty_with_meta(meta);

    // A private-looking tag, wire VR explicitly declared PN (an ordinary text VR - not SQ),
    // but the stored value is a Sequence - exactly the kind of VR/value mismatch nothing in
    // the public API prevents.
    let item: InMemDicomObject = InMemDicomObject::from_element_iter(std::iter::empty());
    obj.put_element(DataElement::new(
        Tag(0x0009, 0x0010),
        VR::PN,
        Value::from(DataSetSequence::new(smallvec![item], Length::UNDEFINED)),
    ));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut bytes = Vec::new();
        obj.write_all(&mut bytes).map(|_| bytes)
    }));

    match result {
        Ok(Ok(bytes)) => {
            println!(
                "OK: no panic - wrote {} bytes successfully (the mismatched sequence value was \
                 gracefully skipped with a warning instead of panicking)",
                bytes.len()
            );
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
                "BUG REPRODUCED: writing a PN-VR element whose stored value is a Sequence hit \
                 an unreachable!() in DataElementTokens::Header instead of a graceful error."
            );
            std::process::exit(1);
        }
    }
}
