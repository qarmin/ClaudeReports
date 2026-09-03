use dicom_core::{DataElement, PrimitiveValue, VR, dicom_value};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::{ConvertOptions, ModalityLutOption, PixelDecoder};

/// Builds a DICOM object whose Rows/Columns/SamplesPerPixel/BitsAllocated are
/// all legal within their own attribute's native (16-bit unsigned, "US") range,
/// but whose product overflows a 64-bit `usize` once multiplied by an
/// otherwise entirely ordinary frame index. Exercises the *public*
/// `DecodedPixelData::frame_data` path (also used internally by
/// `to_vec`/`to_dynamic_image` for most bit depths/sample counts).
fn build_object_frame_data() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.111111111111111111111111111111111111111")
        .build()
        .unwrap();

    let mut obj = FileDicomObject::new_empty_with_meta(meta);

    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, [65535])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, [65535])));
    obj.put_element(DataElement::new(
        tags::SAMPLES_PER_PIXEL,
        VR::US,
        dicom_value!(U16, [65535]),
    ));
    obj.put_element(DataElement::new(
        tags::PHOTOMETRIC_INTERPRETATION,
        VR::CS,
        PrimitiveValue::from("MONOCHROME2"),
    ));
    obj.put_element(DataElement::new(
        tags::BITS_ALLOCATED,
        VR::US,
        dicom_value!(U16, [65535]),
    ));
    obj.put_element(DataElement::new(tags::BITS_STORED, VR::US, dicom_value!(U16, [16])));
    obj.put_element(DataElement::new(tags::HIGH_BIT, VR::US, dicom_value!(U16, [15])));
    obj.put_element(DataElement::new(
        tags::PIXEL_REPRESENTATION,
        VR::US,
        dicom_value!(U16, [0]),
    ));
    obj.put_element(DataElement::new(
        tags::NUMBER_OF_FRAMES,
        VR::IS,
        PrimitiveValue::from("10"),
    ));
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        dicom_value!(U8, [0, 0, 0, 0]),
    ));

    obj
}

/// Builds a DICOM object that takes the `ModalityLutOption::None` branch of
/// `to_dynamic_image_with_options`'s monochrome/16-bit code path, whose own
/// separate (hardcoded `* 2`) frame-size computation has the same unchecked
/// multiply-then-late-bounds-check shape.
fn build_object_modality_lut_none() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.111111111111111111111111111111111111112")
        .build()
        .unwrap();

    let mut obj = FileDicomObject::new_empty_with_meta(meta);

    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, [65535])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, [65535])));
    obj.put_element(DataElement::new(
        tags::SAMPLES_PER_PIXEL,
        VR::US,
        dicom_value!(U16, [1]),
    ));
    obj.put_element(DataElement::new(
        tags::PHOTOMETRIC_INTERPRETATION,
        VR::CS,
        PrimitiveValue::from("MONOCHROME2"),
    ));
    obj.put_element(DataElement::new(tags::BITS_ALLOCATED, VR::US, dicom_value!(U16, [16])));
    obj.put_element(DataElement::new(tags::BITS_STORED, VR::US, dicom_value!(U16, [16])));
    obj.put_element(DataElement::new(tags::HIGH_BIT, VR::US, dicom_value!(U16, [15])));
    obj.put_element(DataElement::new(
        tags::PIXEL_REPRESENTATION,
        VR::US,
        dicom_value!(U16, [0]),
    ));
    obj.put_element(DataElement::new(
        tags::NUMBER_OF_FRAMES,
        VR::IS,
        PrimitiveValue::from("1"),
    ));
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OW,
        dicom_value!(U16, [0, 0]),
    ));

    obj
}

fn report(label: &str, result: std::thread::Result<dicom_pixeldata::Result<()>>) -> bool {
    match result {
        Ok(Ok(())) => {
            println!("{label}: did not reproduce - completed with no panic");
            false
        }
        Ok(Err(e)) => {
            println!("{label}: did not reproduce - returned a graceful error: {e}");
            false
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            println!("{label}: REPRODUCED - panicked: {msg}");
            true
        }
    }
}

fn main() {
    let obj1 = build_object_frame_data();
    let reproduced_1 = report(
        "frame_data(9) on a 10-claimed-frame object",
        std::panic::catch_unwind(|| {
            let decoded = obj1.decode_pixel_data().expect("should decode pixel data");
            decoded.frame_data(9).map(|_| ())
        }),
    );

    let obj2 = build_object_modality_lut_none();
    let reproduced_2 = report(
        "to_dynamic_image_with_options(3_000_000_000, ModalityLutOption::None)",
        std::panic::catch_unwind(|| {
            let decoded = obj2.decode_pixel_data().expect("should decode pixel data");
            let options = ConvertOptions::new().with_modality_lut(ModalityLutOption::None);
            decoded
                .to_dynamic_image_with_options(3_000_000_000, &options)
                .map(|_| ())
        }),
    );

    if reproduced_1 || reproduced_2 {
        std::process::exit(1);
    }
}
