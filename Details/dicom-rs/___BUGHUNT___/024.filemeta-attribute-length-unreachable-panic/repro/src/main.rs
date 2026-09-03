use dicom_core::header::HasLength;
use dicom_dictionary_std::{tags, uids};
use dicom_object::{DicomObject, FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn main() {
    // An entirely ordinary file meta table - every mandatory field populated
    // the normal way through the builder.
    let meta = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.111111111111111111111111111111111111111")
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .build()
        .expect("valid file meta table");

    let file: FileDicomObject<InMemDicomObject> = FileDicomObject::new_empty_with_meta(meta);

    // File Meta Information Version (0002,0001) is a mandatory attribute
    // present in every DICOM file meta group - attr() is expected to always
    // succeed for it.
    let attr = file
        .attr(tags::FILE_META_INFORMATION_VERSION)
        .expect("File Meta Information Version should always be present");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| attr.length()));

    match result {
        Ok(len) => {
            println!("did not reproduce - length() returned {len:?}");
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - panicked in length(): {msg}");
            std::process::exit(1);
        }
    }
}
