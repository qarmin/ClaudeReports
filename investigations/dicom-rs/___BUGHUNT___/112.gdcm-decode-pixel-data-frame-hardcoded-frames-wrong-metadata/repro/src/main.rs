//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API
//! (the `gdcm` feature - the exact code path in `pixeldata/src/gdcm.rs`) on a native
//! (non-encapsulated), uncompressed, 2-frame `FileDicomObject<InMemDicomObject>` built
//! entirely through this crate's own public element-insertion API (`put_element`/
//! `from_element_iter`) - the same public entry point any real caller (a DICOM viewer, a
//! PACS client rendering thumbnails per-frame) would use.
//!
//! The object carries no top-level WindowCenter/WindowWidth, only a real
//! `PerFrameFunctionalGroupsSequence` (5200,9230) with a `FrameVOILUTSequence` (0028,9132)
//! per item giving frame 0 and frame 1 genuinely different WindowCenter/WindowWidth - the
//! same per-frame functional-group idiom real multi-frame DICOM objects use, and the exact
//! shape `pixeldata/src/attribute.rs`'s `window_center`/`window_width` accessors already
//! special-case via `get_from_per_frame`.
//!
//! `decode_pixel_data_frame(1)` builds the *full*, un-truncated window/rescale arrays via
//! those same accessors (so `window[1]` really is frame 1's own WindowCenter/WindowWidth),
//! but hardcodes `number_of_frames: 1` on the returned `DecodedPixelData` - so the only way
//! a caller can address the frame that object actually holds is `to_dynamic_image(0)`,
//! which then indexes `window[0]` (frame 0's values) instead of `window[1]` (the frame
//! that was actually decoded). This compares that buggy result against the correct
//! reference obtained by calling `decode_pixel_data()` (the full, all-frames decode) and
//! asking it for the very same real frame (`to_dynamic_image(1)`), which does receive the
//! real frame index and so indexes `window[1]` correctly.

use dicom_core::value::{DataSetSequence, Value};
use dicom_core::{DataElement, Length, PrimitiveValue, Tag, VR, dicom_value, smallvec::smallvec};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

const FRAME_VOILUT_SEQUENCE: Tag = tags::FRAME_VOILUT_SEQUENCE;
const PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE: Tag = tags::PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE;

fn voi_lut_item(window_center: f64, window_width: f64) -> InMemDicomObject {
    InMemDicomObject::from_element_iter(vec![
        DataElement::new(
            tags::WINDOW_CENTER,
            VR::DS,
            PrimitiveValue::from(window_center.to_string()),
        ),
        DataElement::new(
            tags::WINDOW_WIDTH,
            VR::DS,
            PrimitiveValue::from(window_width.to_string()),
        ),
    ])
}

fn frame_functional_group_item(window_center: f64, window_width: f64) -> InMemDicomObject {
    let voi_lut_seq_item = voi_lut_item(window_center, window_width);
    InMemDicomObject::from_element_iter(vec![DataElement::new(
        FRAME_VOILUT_SEQUENCE,
        VR::SQ,
        Value::from(DataSetSequence::new(
            smallvec![voi_lut_seq_item],
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
        .media_storage_sop_instance_uid("2.25.222222222222222222222222222222222222222")
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

    // Frame 0 pixels: all 10 (dark). Frame 1 pixels: all 200 (bright).
    let pixel_bytes: Vec<u8> = vec![10, 10, 10, 10, 200, 200, 200, 200];
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        PrimitiveValue::from(pixel_bytes),
    ));

    // Per-frame WindowCenter/WindowWidth, genuinely different between the two frames -
    // frame 0 is windowed narrow around its own dark pixel value (center=50, width=100),
    // frame 1 is windowed narrow around its own bright pixel value (center=200, width=100).
    // No top-level WindowCenter/WindowWidth element exists, so
    // pixeldata/src/attribute.rs's window_center()/window_width() fall through to reading
    // this PerFrameFunctionalGroupsSequence, exactly as a real multi-frame DICOM object would.
    let per_frame_items = smallvec![
        frame_functional_group_item(50.0, 100.0),
        frame_functional_group_item(200.0, 100.0),
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
    let reference_image = full
        .to_dynamic_image(1)
        .expect("to_dynamic_image(1) on full decode should succeed")
        .to_luma8();
    let reference_pixel = reference_image.get_pixel(0, 0)[0];
    println!(
        "  reference frame-1 pixel value (correct window applied) = {reference_pixel}"
    );

    println!(
        "\nUnder test: decode_pixel_data_frame(1) (real frame 1's pixel bytes) + to_dynamic_image(0)"
    );
    let single = obj
        .decode_pixel_data_frame(1)
        .expect("decode_pixel_data_frame(1) should succeed");
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
        "\nBoth paths decode the SAME real DICOM frame (frame index 1, pixel bytes [200,200,200,200])."
    );
    println!(
        "If they applied the same (correct, frame-1) window/rescale metadata, both pixel values would match."
    );

    if single_pixel != reference_pixel {
        println!(
            "\nBUG CONFIRMED: decode_pixel_data_frame(1)'s to_dynamic_image(0) = {single_pixel}, \
             but the correct value (as computed by the full decode_pixel_data() path for the \
             same real frame) is {reference_pixel} - decode_pixel_data_frame silently applied \
             frame 0's window/rescale metadata to frame 1's pixel data."
        );
        std::process::exit(1);
    } else {
        println!("\nNo mismatch observed - decode_pixel_data_frame appears to use the correct per-frame metadata.");
    }
}
