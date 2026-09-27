use dicom_core::header::Length;
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

    // Pixel Data (7FE0,0010) with VR OB and an undefined length is what
    // `DataToken::from(header)` classifies as the start of an *encapsulated*
    // pixel data sequence - but nothing stops a caller from constructing
    // this exact tag/VR/length combination with an ordinary primitive value
    // instead (e.g. from a bulk edit, a copy-and-modify of an existing
    // element, or simply a caller who didn't realize the significance of an
    // undefined length on this one specific tag).
    let elem = DataElement::new_with_len(
        Tag(0x7fe0, 0x0010),
        VR::OB,
        Length::UNDEFINED,
        dicom_value!(U8, [1, 2, 3, 4]),
    );
    obj.put_element(elem);

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
