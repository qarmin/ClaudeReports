//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API,
//! the *default* (non-`gdcm`) `decode_pixel_data_frame` implementation in `pixeldata/src/lib.rs`,
//! on a native (non-encapsulated), uncompressed, 2-frame `FileDicomObject<InMemDicomObject>`
//! built entirely through this crate's own public element-insertion API
//! (`put_element`/`from_element_iter`) - the same public entry point any real caller (a DICOM
//! viewer, a PACS client rendering thumbnails per-frame) would use.
//!
//! The object carries no top-level VOILUTSequence/WindowCenter/WindowWidth, only a real
//! `PerFrameFunctionalGroupsSequence` (5200,9230) with a `FrameVOILUTSequence` (0028,9132)
//! per item giving frame 0 and frame 1 genuinely different VOI LUT Sequence (0028,3010)
//! entries (single-entry LUTs, so any input pixel value maps to a fixed constant output: 0
//! for frame 0's LUT, 255 for frame 1's LUT) - the exact shape `pixeldata/src/attribute.rs`'s
//! `voi_lut_sequence` accessor already special-cases via `get_from_per_frame`.
//!
//! `decode_pixel_data_frame(1)` (the default, non-`gdcm` implementation) already narrows
//! `rescale`/`window`/`voi_lut_function` down to the single entry for the requested frame
//! before returning, but passes `voi_lut_sequence` straight through unnarrowed (still holding
//! both frames' entries). The only way a caller can address the single already-decoded frame
//! is `to_dynamic_image(0)`, which then indexes `voi_lut_sequence[0]` (frame 0's LUT) instead
//! of `voi_lut_sequence[1]` (the LUT for the frame that was actually decoded). This compares
//! that buggy result against the correct reference obtained by calling `decode_pixel_data()`
//! (the full, all-frames decode) and asking it for the very same real frame
//! (`to_dynamic_image(1)`), which does receive the real frame index and so indexes
//! `voi_lut_sequence[1]` correctly.

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
        DataElement::new(
            tags::LUT_DESCRIPTOR,
            VR::US,
            dicom_value!(U16, [1, 0, 8]),
        ),
        DataElement::new(tags::LUT_DATA, VR::US, dicom_value!(U16, [output_value])),
    ])
}

fn frame_functional_group_item(output_value: u16) -> InMemDicomObject {
    // Frame VOI LUT (0028,9132) is a macro item that itself contains a nested
    // VOI LUT Sequence (0028,3010), which is where the actual LUT entries live -
    // matching the real DICOM structure `get_from_per_frame` expects:
    // PerFrameFunctionalGroupsSequence -> item -> FrameVOILUTSequence -> item -> VOILUTSequence -> item(s).
    let voi_lut_seq_item = voi_lut_item(output_value);
    let frame_voi_lut_macro_item = InMemDicomObject::from_element_iter(vec![DataElement::new(
        tags::VOILUT_SEQUENCE,
        VR::SQ,
        Value::from(DataSetSequence::new(
            smallvec![voi_lut_seq_item],
            Length::UNDEFINED,
        )),
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

fn build_object() -> FileDicomObject<InMemDicomObject> {
    // 2x2 = 4 pixels/frame, 8 bits each -> 4 bytes/frame, 2 frames.
    let cols: u16 = 2;
    let rows: u16 = 2;
    let number_of_frames: u32 = 2;

    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::IMPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.333333333333333333333333333333333333333")
        .build()
        .unwrap();

    let mut obj = FileDicomObject::new_empty_with_meta(meta);

    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, [rows])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, [cols])));
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
        PrimitiveValue::from(number_of_frames.to_string()),
    ));

    // Frame 0 pixels: all 10. Frame 1 pixels: all 20. (Values don't matter to the LUT
    // output here - each frame's single-entry LUT maps any input to a fixed constant - only
    // to which real frame's bytes get decoded.)
    let pixel_bytes: Vec<u8> = vec![10, 10, 10, 10, 20, 20, 20, 20];
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        PrimitiveValue::from(pixel_bytes),
    ));

    // Per-frame VOI LUT Sequence, genuinely different between the two frames - frame 0's LUT
    // always outputs 0 (dark), frame 1's LUT always outputs 255 (bright). No top-level
    // VOILUTSequence/WindowCenter/WindowWidth element exists, so
    // pixeldata/src/attribute.rs's voi_lut_sequence() falls through to reading this
    // PerFrameFunctionalGroupsSequence, exactly as a real multi-frame DICOM object would.
    let per_frame_items = smallvec![
        frame_functional_group_item(0),
        frame_functional_group_item(255),
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

    println!("Reference: decode_pixel_data() (all frames) + to_dynamic_image(1) (real frame 1)");
    let full = obj.decode_pixel_data().expect("decode_pixel_data should succeed");
    assert_eq!(full.number_of_frames(), 2, "sanity: object has 2 frames");
    if let Some(seq) = full.voi_lut_sequence().expect("voi_lut_sequence() should succeed") {
        println!("DEBUG: full.voi_lut_sequence() has {} entries:", seq.len());
        for (i, lut) in seq.iter().enumerate() {
            println!(
                "  [{i}] min_pixel_value={} bits_stored={} data={:?}",
                lut.min_pixel_value, lut.bits_stored, lut.data
            );
        }
    } else {
        println!("DEBUG: full.voi_lut_sequence() = None");
    }
    println!("DEBUG: full.number_of_frames() = {}", full.number_of_frames());
    let reference_image = full
        .to_dynamic_image(1)
        .expect("to_dynamic_image(1) on full decode should succeed")
        .to_luma8();
    let reference_pixel = reference_image.get_pixel(0, 0)[0];
    println!("  reference frame-1 pixel value (correct VOI LUT applied) = {reference_pixel}");

    println!(
        "\nUnder test: decode_pixel_data_frame(1) (real frame 1's pixel bytes, default non-gdcm path) + to_dynamic_image(0)"
    );
    let single = obj
        .decode_pixel_data_frame(1)
        .expect("decode_pixel_data_frame(1) should succeed");
    if let Some(seq) = single.voi_lut_sequence().expect("voi_lut_sequence() should succeed") {
        println!("DEBUG: single.voi_lut_sequence() has {} entries:", seq.len());
        for (i, lut) in seq.iter().enumerate() {
            println!(
                "  [{i}] min_pixel_value={} bits_stored={} data={:?}",
                lut.min_pixel_value, lut.bits_stored, lut.data
            );
        }
    } else {
        println!("DEBUG: single.voi_lut_sequence() = None");
    }
    println!(
        "  single.number_of_frames() = {} (hardcoded, not actually validated against anything)",
        single.number_of_frames()
    );
    let single_image = single
        .to_dynamic_image(0)
        .expect("to_dynamic_image(0) on the single-frame decode should succeed")
        .to_luma8();
    let single_pixel = single_image.get_pixel(0, 0)[0];
    println!("  decode_pixel_data_frame(1) result pixel value = {single_pixel}");

    println!(
        "\nBoth paths decode the SAME real DICOM frame (frame index 1, pixel bytes [20,20,20,20])."
    );
    println!(
        "If they applied the same (correct, frame-1) VOI LUT Sequence entry, both pixel values would match."
    );

    if single_pixel != reference_pixel {
        println!(
            "\nBUG CONFIRMED: decode_pixel_data_frame(1)'s to_dynamic_image(0) = {single_pixel}, \
             but the correct value (as computed by the full decode_pixel_data() path for the \
             same real frame) is {reference_pixel} - decode_pixel_data_frame silently applied \
             frame 0's un-narrowed VOI LUT Sequence entry to frame 1's pixel data."
        );
        std::process::exit(1);
    } else {
        println!("\nNo mismatch observed - decode_pixel_data_frame appears to use the correct per-frame VOI LUT Sequence entry.");
    }
}
