use dicom_dictionary_std::{tags, uids};
use dicom_object::{DicomAttribute, DicomObject, FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn main() {
    // Implementation Version Name (0002,0013) is a Type 3 (optional) file
    // meta attribute per PS3.10. Build a perfectly valid file meta table
    // that has an Implementation Class UID but omits the (optional)
    // Implementation Version Name - a common, spec-compliant combination.
    let meta = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.111111111111111111111111111111111111111")
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .implementation_class_uid("1.2.3.4.5.6.7.8.9")
        // deliberately NOT calling .implementation_version_name(...)
        .build()
        .expect("valid file meta table");

    assert!(
        meta.implementation_version_name.is_none(),
        "test setup invariant: implementation_version_name must be absent"
    );

    let file: FileDicomObject<InMemDicomObject> =
        FileDicomObject::new_empty_with_meta(meta);

    // ordinary generic attribute access, as documented for DicomObject::attr
    let attr = match file.attr(tags::IMPLEMENTATION_VERSION_NAME) {
        Ok(attr) => attr,
        Err(e) => {
            println!(
                "did not reproduce - attr() correctly reports the attribute as absent: {e}"
            );
            return;
        }
    };

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        attr.to_primitive_value()
    }));

    match result {
        Ok(Ok(value)) => {
            println!("did not reproduce - to_primitive_value() returned {value:?}");
        }
        Ok(Err(e)) => {
            println!("did not reproduce - to_primitive_value() returned a graceful error: {e}");
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - panicked in to_primitive_value(): {msg}");
            std::process::exit(1);
        }
    }
}
