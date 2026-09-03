//! Real end-to-end reproduction, through the actual production code path
//! (`FileDicomObject<InMemDicomObject<D>>`'s `PixelDataObject` impl, which uses
//! `encoding/src/adapters.rs`'s `frame_pixel_data` default trait method unmodified),
//! of the "Some(0 bytes) instead of None" quirk noted while working candidate 124.
//!
//! Single external artifact (an in-memory DICOM object whose own encapsulated Pixel Data /
//! Basic Offset Table is internally inconsistent - exactly the shape a malformed or
//! adversarial real file's Pixel Data Sequence could produce), no second participant to
//! fake - CONFIRMATION.md section 11 does not apply.
//!
//! Object shape used throughout: 2 frames, 3 fragments (deliberately NOT 1:1
//! fragment-to-frame, so `frame_pixel_data` takes the offset-table-walking "Other cases"
//! branch, not the 1:1 shortcut) - fragment 0+1 legitimately belong to frame 0, fragment 2
//! legitimately belongs to frame 1. Each fragment is 10 bytes, so each item (8-byte header +
//! 10-byte payload) advances the walked `offset` by 18. Frame 1's *real* Basic Offset Table
//! entry would be 36 (= 18 + 18, the position right after frame 0's two fragments).

use dicom_core::{DataElement, Length, VR, value::PixelFragmentSequence};
use dicom_dictionary_std::{tags, uids};
use dicom_encoding::adapters::PixelDataObject;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn build_obj(offset_table: Vec<u32>) -> FileDicomObject<InMemDicomObject> {
    let mut obj = FileDicomObject::new_empty_with_meta(
        FileMetaTableBuilder::new()
            .transfer_syntax(uids::JPEG_BASELINE8_BIT)
            .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
            .media_storage_sop_instance_uid("2.25.1")
            .build()
            .unwrap(),
    );

    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_core::dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_core::dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::SAMPLES_PER_PIXEL, VR::US, dicom_core::dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::BITS_ALLOCATED, VR::US, dicom_core::dicom_value!(U16, [8])));
    obj.put_element(DataElement::new(tags::BITS_STORED, VR::US, dicom_core::dicom_value!(U16, [8])));
    obj.put_element(DataElement::new(
        tags::PHOTOMETRIC_INTERPRETATION,
        VR::CS,
        dicom_core::dicom_value!(Strs, ["MONOCHROME2"]),
    ));
    obj.put_element(DataElement::new(
        tags::NUMBER_OF_FRAMES,
        VR::IS,
        dicom_core::dicom_value!(Strs, ["2"]),
    ));
    obj.put_element(DataElement::new_with_len(
        tags::PIXEL_DATA,
        VR::OB,
        Length::UNDEFINED,
        PixelFragmentSequence::new(
            offset_table,
            vec![vec![0xAAu8; 10], vec![0xBBu8; 10], vec![0xCCu8; 10]],
        ),
    ));

    obj
}

fn run_case(name: &str, obj: &FileDicomObject<InMemDicomObject>, frame: u32) -> Option<usize> {
    let result = obj.frame_pixel_data(frame);
    match &result {
        None => println!("[{name}] frame_pixel_data({frame}) = None"),
        Some(bytes) => println!("[{name}] frame_pixel_data({frame}) = Some({} bytes)", bytes.len()),
    }
    result.map(|c| c.len())
}

fn main() {
    println!("=== well-formed offset table [0, 36] (36 = the real byte offset of frame 1's fragment) ===");
    let obj_good = build_obj(vec![0, 36]);
    let good_f0 = run_case("well-formed", &obj_good, 0);
    let good_f1 = run_case("well-formed", &obj_good, 1);

    println!();
    println!("=== malformed offset table [0, 1000000] (frame 1's entry unreachable by any real fragment offset) ===");
    let obj_bogus = build_obj(vec![0, 1_000_000]);
    let bogus_f0 = run_case("malformed", &obj_bogus, 0);
    let bogus_f1 = run_case("malformed", &obj_bogus, 1);

    println!();
    println!("well-formed case: frame 0 = {:?} (expected Some(20)), frame 1 = {:?} (expected Some(10))", good_f0, good_f1);
    println!("malformed case:   frame 0 = {:?}, frame 1 = {:?}", bogus_f0, bogus_f1);

    let bug_reproduces = bogus_f1 == Some(0);
    println!();
    println!(
        "BUG (frame 1's unreachable offset-table entry yields Some(0 bytes) instead of None, \
         contradicting frame_pixel_data's own documented contract \"Returns None if there is no \
         such frame\"): {bug_reproduces}"
    );

    if bug_reproduces {
        std::process::exit(1);
    }
}
