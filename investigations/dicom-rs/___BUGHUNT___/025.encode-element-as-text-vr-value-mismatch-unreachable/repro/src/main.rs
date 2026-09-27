use dicom_core::ops::{ApplyOp, AttributeAction, AttributeOp};
use dicom_core::value::DicomDate;
use dicom_core::{DataElement, Tag, VR, dicom_value};
use dicom_dictionary_std::uids;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn main() {
    let meta = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.111111111111111111111111111111111111111")
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .build()
        .unwrap();

    let mut obj: FileDicomObject<InMemDicomObject> = FileDicomObject::new_empty_with_meta(meta);

    // an ordinary DA-VR element holding a Date value
    let tag = Tag(0x0009, 0x0010);
    obj.put_element(DataElement::new(
        tag,
        VR::DA,
        dicom_value!(Date, [DicomDate::from_ymd(2020, 1, 1).unwrap()]),
    ));

    // SetVr is documented as leaving the underlying value untouched -
    // this is an ordinary, supported operation
    obj.apply(AttributeOp::new(tag, AttributeAction::SetVr(VR::DS)))
        .expect("SetVr should succeed");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut buf = Vec::new();
        obj.write_dataset_with_ts(
            &mut buf,
            &dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN.erased(),
        )
    }));

    match result {
        Ok(Ok(())) => println!("did not reproduce - write succeeded"),
        Ok(Err(e)) => println!("did not reproduce - write returned a graceful error: {e}"),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - panicked while writing: {msg}");
            std::process::exit(1);
        }
    }
}
