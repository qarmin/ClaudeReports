use dicom_core::{DataElement, PrimitiveValue, VR, dicom_value};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

fn build_object() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.111111111111111111111111111111111111111")
        .build()
        .unwrap();

    let mut obj = FileDicomObject::new_empty_with_meta(meta);

    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, [1])));
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
    obj.put_element(DataElement::new(
        tags::BITS_ALLOCATED,
        VR::US,
        dicom_value!(U16, [8]),
    ));
    obj.put_element(DataElement::new(
        tags::BITS_STORED,
        VR::US,
        dicom_value!(U16, [8]),
    ));
    obj.put_element(DataElement::new(tags::HIGH_BIT, VR::US, dicom_value!(U16, [7])));
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

    // A huge rescale slope: rescale.apply(v) = v * slope + intercept can wildly
    // overshoot into the extremes of the f64/i32 range.
    obj.put_element(DataElement::new(
        tags::RESCALE_SLOPE,
        VR::DS,
        PrimitiveValue::from("1.0e30"),
    ));
    obj.put_element(DataElement::new(
        tags::RESCALE_INTERCEPT,
        VR::DS,
        PrimitiveValue::from("0"),
    ));

    // VOI LUT Sequence, with a LUT Descriptor whose min_pixel_value is
    // deeply negative (VR=SS forces a signed interpretation).
    let mut lut_item = InMemDicomObject::new_empty();
    lut_item.put_element(DataElement::new(
        tags::LUT_DESCRIPTOR,
        VR::SS,
        dicom_value!(I16, [2, -32768, 8]),
    ));
    lut_item.put_element(DataElement::new(
        tags::LUT_DATA,
        VR::US,
        dicom_value!(U16, [0, 255]),
    ));
    obj.put_element(DataElement::new(
        tags::VOILUT_SEQUENCE,
        VR::SQ,
        dicom_core::value::Value::Sequence(
            dicom_core::value::DataSetSequence::from(vec![lut_item]),
        ),
    ));

    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        dicom_value!(U8, [255]),
    ));

    obj
}

fn main() {
    let obj = build_object();

    let result = std::panic::catch_unwind(|| {
        let pixel_data = obj.decode_pixel_data().expect("should decode pixel data");
        pixel_data.to_dynamic_image(0)
    });

    match result {
        Ok(Ok(image)) => {
            let pixel = image.to_luma8().get_pixel(0, 0).0[0];
            println!(
                "did not reproduce (this build) - image conversion completed with no panic; \
                 pixel(0,0) = {pixel} (note: with overflow-checks off, the same i32 subtraction \
                 silently wraps instead of panicking - see RESULT.md)"
            );
        }
        Ok(Err(e)) => {
            println!(
                "did not reproduce - image conversion returned a graceful error: {e}"
            );
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - panicked during VOI LUT transform: {msg}");
            std::process::exit(1);
        }
    }
}
