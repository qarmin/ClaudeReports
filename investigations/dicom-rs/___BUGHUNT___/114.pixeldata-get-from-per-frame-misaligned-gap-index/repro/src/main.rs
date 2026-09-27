//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API and the
//! default (non-`gdcm`) `decode_pixel_data` (the *whole-object*, all-frames decode, not
//! `decode_pixel_data_frame`) in `pixeldata/src/lib.rs`.
//!
//! Builds a real, native (non-encapsulated), uncompressed, 3-frame
//! `FileDicomObject<InMemDicomObject>` entirely through this crate's own public element-insertion
//! API (`put_element`/`from_element_iter`), with a real `PerFrameFunctionalGroupsSequence`
//! (5200,9230) that gives frame 0 and frame 2 genuinely different, real per-frame VOI LUT
//! Sequence entries (frame 0's LUT maps every input to output 0, frame 2's LUT maps every input
//! to output 255), while frame 1's item in the sequence carries no `FrameVOILUTSequence`
//! (0028,9132) at all - a real-world-plausible gap (a non-fully-compliant per-frame functional
//! groups sequence), not a fabricated internal state.
//!
//! `pixeldata/src/attribute.rs`'s `get_from_per_frame` reads this sequence via
//! `.iter().filter_map(...)`, which silently *skips* frame 1's item (since it has no matching
//! attribute) rather than keeping its position - so the resulting `Vec<VoiLut>` has only 2
//! entries (frame 0's and frame 2's), at indices 0 and 1, while `NumberOfFrames` is genuinely 3.
//! `DecodedPixelData::voi_lut_sequence()`'s length-mismatch fallback (`len=2 != number_of_frames=3`)
//! then fires with `enforce_frame_fg_vm_match: false` (which `decode_pixel_data` always sets),
//! silently returning element 0 (frame 0's LUT) for every frame instead of erroring - so
//! `decode_pixel_data().to_dynamic_image(2)` applies frame 0's VOI LUT to frame 2's own real
//! pixel data instead of frame 2's own (real, present, per-frame) VOI LUT.

use dicom_core::value::{DataSetSequence, Value};
use dicom_core::{DataElement, Length, PrimitiveValue, Tag, VR, dicom_value, smallvec::smallvec};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

const FRAME_VOILUT_SEQUENCE: Tag = tags::FRAME_VOILUT_SEQUENCE;
const PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE: Tag = tags::PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE;

fn voi_lut_item(output_value: u16) -> InMemDicomObject {
    // LUT Descriptor = [LUT length = 1, first input value mapped = 0, bits stored = 8].
    // A single-entry LUT: every input pixel value maps to the same constant output value.
    InMemDicomObject::from_element_iter(vec![
        DataElement::new(tags::LUT_DESCRIPTOR, VR::US, dicom_value!(U16, [1, 0, 8])),
        DataElement::new(tags::LUT_DATA, VR::US, dicom_value!(U16, [output_value])),
    ])
}

fn frame_group_with_voi(output_value: u16) -> InMemDicomObject {
    // PerFrameFunctionalGroupsSequence -> item -> FrameVOILUTSequence -> item -> VOILUTSequence -> item.
    let voi_lut_seq_item = voi_lut_item(output_value);
    let frame_voi_lut_macro_item = InMemDicomObject::from_element_iter(vec![DataElement::new(
        tags::VOILUT_SEQUENCE,
        VR::SQ,
        Value::from(DataSetSequence::new(smallvec![voi_lut_seq_item], Length::UNDEFINED)),
    )]);
    InMemDicomObject::from_element_iter(vec![DataElement::new(
        FRAME_VOILUT_SEQUENCE,
        VR::SQ,
        Value::from(DataSetSequence::new(
            smallvec![frame_voi_lut_macro_item],
            Length::UNDEFINED,
        )),
    )])
}

fn frame_group_without_voi() -> InMemDicomObject {
    // A real per-frame functional group item that simply doesn't carry VOI LUT info for its
    // frame - no FrameVOILUTSequence element at all.
    InMemDicomObject::from_element_iter(Vec::<DataElement<InMemDicomObject>>::new())
}

fn build_object() -> FileDicomObject<InMemDicomObject> {
    // 2x2 = 4 pixels/frame, 8 bits each -> 4 bytes/frame, 3 frames.
    let cols: u16 = 2;
    let rows: u16 = 2;
    let number_of_frames: u32 = 3;

    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::IMPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.555555555555555555555555555555555555555")
        .build()
        .unwrap();

    let mut obj = FileDicomObject::new_empty_with_meta(meta);

    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, [rows])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, [cols])));
    obj.put_element(DataElement::new(tags::SAMPLES_PER_PIXEL, VR::US, dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(
        tags::PHOTOMETRIC_INTERPRETATION,
        VR::CS,
        PrimitiveValue::from("MONOCHROME2"),
    ));
    obj.put_element(DataElement::new(tags::BITS_ALLOCATED, VR::US, dicom_value!(U16, [8])));
    obj.put_element(DataElement::new(tags::BITS_STORED, VR::US, dicom_value!(U16, [8])));
    obj.put_element(DataElement::new(tags::HIGH_BIT, VR::US, dicom_value!(U16, [7])));
    obj.put_element(DataElement::new(tags::PIXEL_REPRESENTATION, VR::US, dicom_value!(U16, [0])));
    obj.put_element(DataElement::new(
        tags::NUMBER_OF_FRAMES,
        VR::IS,
        PrimitiveValue::from(number_of_frames.to_string()),
    ));

    // Frame 0 pixels: all 10. Frame 1: all 20. Frame 2: all 30.
    let pixel_bytes: Vec<u8> = vec![10, 10, 10, 10, 20, 20, 20, 20, 30, 30, 30, 30];
    obj.put_element(DataElement::new(tags::PIXEL_DATA, VR::OB, PrimitiveValue::from(pixel_bytes)));

    // Frame 0's real per-frame VOI LUT outputs 0. Frame 1's item has NO VOI LUT group at all
    // (the gap). Frame 2's real per-frame VOI LUT outputs 255.
    let per_frame_items = smallvec![
        frame_group_with_voi(0),
        frame_group_without_voi(),
        frame_group_with_voi(255),
    ];
    obj.put_element(DataElement::new(
        PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE,
        VR::SQ,
        Value::from(DataSetSequence::new(per_frame_items, Length::UNDEFINED)),
    ));

    obj
}

fn main() {
    let obj = build_object();

    let decoded = obj.decode_pixel_data().expect("decode_pixel_data() should succeed");
    assert_eq!(decoded.number_of_frames(), 3, "sanity: object declares 3 frames");

    let seq = decoded
        .voi_lut_sequence()
        .expect("voi_lut_sequence() accessor should not error (enforce_frame_fg_vm_match is false)");
    match &seq {
        Some(entries) => {
            println!("decoded.voi_lut_sequence() = Some, {} entries:", entries.len());
            for (i, lut) in entries.iter().enumerate() {
                println!("  [{i}] data={:?}", lut.data);
            }
        }
        None => println!("decoded.voi_lut_sequence() = None"),
    }

    // Frame 2's own real per-frame VOI LUT (present directly in the object, item 2 of
    // PerFrameFunctionalGroupsSequence) maps every input to 255; its own pixel bytes are all 30.
    let frame2_pixel = decoded
        .to_dynamic_image(2)
        .expect("to_dynamic_image(2) should succeed")
        .to_luma8()
        .get_pixel(0, 0)[0];

    println!("decode_pixel_data().to_dynamic_image(2) pixel value = {frame2_pixel}");
    println!(
        "Frame 2's own real per-frame VOI LUT (present in the object) maps every input to 255; \
         frame 0's real per-frame VOI LUT (a DIFFERENT frame's data) maps every input to 0."
    );

    if frame2_pixel == 0 {
        println!(
            "\nBUG CONFIRMED: decode_pixel_data().to_dynamic_image(2) = 0, matching frame 0's VOI \
             LUT output instead of frame 2's own (255) - get_from_per_frame silently dropped frame \
             1's group-less item, misaligning the resulting 2-entry array against the declared \
             3-frame count, and the length-mismatch fallback in voi_lut_sequence() then silently \
             returned frame 0's entry for every frame."
        );
        std::process::exit(1);
    } else if frame2_pixel == 255 {
        println!("\nNo mismatch observed - decode_pixel_data() applied the correct per-frame VOI LUT for frame 2.");
    } else {
        println!("\nUnexpected pixel value {frame2_pixel} (neither 0 nor 255) - investigate further.");
        std::process::exit(2);
    }
}
