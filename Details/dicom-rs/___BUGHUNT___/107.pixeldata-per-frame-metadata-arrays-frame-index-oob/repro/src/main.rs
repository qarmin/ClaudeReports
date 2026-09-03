//! Real, end-to-end reproduction using the public `dicom_pixeldata` API
//! (`DecodedPixelData::to_dynamic_image`/`to_vec_frame`) on an ordinary
//! `FileDicomObject<InMemDicomObject>` built entirely through this crate's own public
//! element-insertion API - the same public entry point any real caller (a DICOM viewer,
//! a PACS client rendering an incoming multi-frame image) would use.
//!
//! `pixeldata/src/lib.rs`'s `build_monochrome_image`/`convert_pixel_slice` repeatedly use
//! the idiom `if X.len() > 1 { X[frame as usize] } else { X[0] }` for several per-frame
//! metadata arrays (RescaleIntercept/RescaleSlope, VOI LUT Sequence, VOI LUT Function,
//! WindowCenter/WindowWidth). The `len() > 1` check only proves the array has *more than
//! one* entry - not that it has *at least `frame + 1`* entries. These arrays' lengths come
//! from attributes that are read completely independently of the raw PixelData buffer's
//! own actual frame count (`NumberOfFrames`, PerFrameFunctionalGroupsSequence item counts,
//! and the top-level VOI LUT Sequence's own item count are three unrelated numbers with no
//! cross-validation between them) - so a `frame` argument that is in range for the actual
//! pixel data buffer (and therefore passes `frame_data()`'s own bounds check) can still be
//! out of range for one of these shorter per-frame arrays, causing a plain
//! out-of-bounds slice-index panic instead of a graceful error.
//!
//! This reproduces two independent occurrences of the same idiom in two different
//! functions/features:
//! 1. `to_dynamic_image()` (the `image`-feature image-rendering path), via a VOI LUT
//!    Sequence with fewer entries than the actual pixel data buffer's frame count.
//! 2. `to_vec_frame()` (the plain numeric-array path, no `image` feature needed), via a
//!    per-frame RescaleIntercept/RescaleSlope (through PerFrameFunctionalGroupsSequence)
//!    with fewer entries than the actual pixel data buffer's frame count.

use dicom_core::value::DataSetSequence;
use dicom_core::{DataElement, PrimitiveValue, VR, dicom_value};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn build_meta(sop_instance_uid: &str) -> dicom_object::FileMetaTable {
    FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid(sop_instance_uid)
        .build()
        .unwrap()
}

/// Case 1: VOI LUT Sequence has 2 entries (matching declared NumberOfFrames = 2), but the
/// raw PixelData buffer itself holds 4 frames' worth of bytes.
fn build_object_voi_lut_mismatch() -> FileDicomObject<InMemDicomObject> {
    let mut obj = FileDicomObject::new_empty_with_meta(build_meta(
        "2.25.111111111111111111111111111111111111111",
    ));

    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, [4])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, [4])));
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
    obj.put_element(DataElement::new(tags::BITS_ALLOCATED, VR::US, dicom_value!(U16, [8])));
    obj.put_element(DataElement::new(tags::BITS_STORED, VR::US, dicom_value!(U16, [8])));
    obj.put_element(DataElement::new(tags::HIGH_BIT, VR::US, dicom_value!(U16, [7])));
    obj.put_element(DataElement::new(
        tags::PIXEL_REPRESENTATION,
        VR::US,
        dicom_value!(U16, [0]),
    ));
    obj.put_element(DataElement::new(
        tags::NUMBER_OF_FRAMES,
        VR::IS,
        PrimitiveValue::from("2"),
    ));
    // 4 frames' worth of raw bytes (4x4x1x8bit x 4 frames = 64 bytes), even though
    // NumberOfFrames declares only 2.
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        PrimitiveValue::U8((0..64u8).collect()),
    ));

    let make_voi_lut_item = || {
        InMemDicomObject::from_element_iter([
            DataElement::new(tags::LUT_DESCRIPTOR, VR::US, dicom_value!(U16, [2, 0, 8])),
            DataElement::new(tags::LUT_DATA, VR::US, dicom_value!(U16, [0, 255])),
        ])
    };
    obj.put_element(DataElement::new(
        tags::VOILUT_SEQUENCE,
        VR::SQ,
        DataSetSequence::from(vec![make_voi_lut_item(), make_voi_lut_item()]),
    ));

    obj
}

/// Case 2: per-frame RescaleIntercept/RescaleSlope (via PerFrameFunctionalGroupsSequence)
/// has 2 entries (matching declared NumberOfFrames = 2), but the raw PixelData buffer
/// itself holds 4 frames' worth of 16-bit samples.
fn build_object_rescale_mismatch() -> FileDicomObject<InMemDicomObject> {
    let mut obj = FileDicomObject::new_empty_with_meta(build_meta(
        "2.25.222222222222222222222222222222222222222",
    ));

    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, [4])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, [4])));
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
        PrimitiveValue::from("2"),
    ));
    // 4 frames' worth of 16-bit samples (2 bytes/sample x 16 samples/frame x 4 frames =
    // 128 bytes), even though NumberOfFrames declares only 2.
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OW,
        PrimitiveValue::U16((0..64u16).collect()),
    ));

    // No top-level RescaleIntercept/RescaleSlope element - a top-level element's own
    // `to_float64()` conversion only ever takes its first value regardless of VM, so this
    // is the only way to get a `self.rescale` field with more than one entry.
    let make_transform_item = |intercept: &str, slope: &str| {
        InMemDicomObject::from_element_iter([DataElement::new(
            tags::PIXEL_VALUE_TRANSFORMATION_SEQUENCE,
            VR::SQ,
            DataSetSequence::from(vec![InMemDicomObject::from_element_iter([
                DataElement::new(tags::RESCALE_INTERCEPT, VR::DS, PrimitiveValue::from(intercept)),
                DataElement::new(tags::RESCALE_SLOPE, VR::DS, PrimitiveValue::from(slope)),
            ])]),
        )])
    };
    obj.put_element(DataElement::new(
        tags::PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE,
        VR::SQ,
        DataSetSequence::from(vec![
            make_transform_item("0", "1"),
            make_transform_item("1", "1"),
        ]),
    ));

    obj
}

fn main() {
    use dicom_pixeldata::PixelDecoder;

    println!("=== case 1: VOI LUT Sequence shorter than actual pixel data frame count ===");
    let obj1 = build_object_voi_lut_mismatch();
    let pixel_data1 = obj1.decode_pixel_data().unwrap();
    println!(
        "declared number_of_frames = {}, VOI LUT Sequence entries = {}",
        pixel_data1.number_of_frames(),
        pixel_data1.voi_lut_sequence().unwrap().map(|v| v.len()).unwrap_or(0)
    );
    println!("calling to_dynamic_image(3) (in range for the actual pixel data buffer)...");
    let case1_ok = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pixel_data1.to_dynamic_image(3)
    })) {
        Ok(Ok(_)) => {
            println!("did not panic - returned Ok");
            true
        }
        Ok(Err(e)) => {
            println!("did not panic - returned a graceful Err: {e}");
            true
        }
        Err(_) => {
            println!("PANICKED (bug reproduced)");
            false
        }
    };

    println!();
    println!("=== case 2: per-frame RescaleIntercept/RescaleSlope shorter than actual pixel data frame count ===");
    let obj2 = build_object_rescale_mismatch();
    let pixel_data2 = obj2.decode_pixel_data().unwrap();
    println!(
        "declared number_of_frames = {}, rescale entries = {}",
        pixel_data2.number_of_frames(),
        pixel_data2.rescale().unwrap().len()
    );
    println!("calling to_vec_frame::<u16>(3) (in range for the actual pixel data buffer)...");
    let case2_ok = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pixel_data2.to_vec_frame::<u16>(3)
    })) {
        Ok(Ok(_)) => {
            println!("did not panic - returned Ok");
            true
        }
        Ok(Err(e)) => {
            println!("did not panic - returned a graceful Err: {e}");
            true
        }
        Err(_) => {
            println!("PANICKED (bug reproduced)");
            false
        }
    };

    if !case1_ok || !case2_ok {
        std::process::exit(1);
    }
}
