//! Follow-up to candidate 114 (`pixeldata/src/attribute.rs`'s `get_from_per_frame`, shared by 6
//! accessors: `voi_lut_sequence`/`voi_lut_function`/`rescale_intercept`/`rescale_slope`/
//! `window_center`/`window_width`). Candidate 114 built a dedicated repro only for
//! `voi_lut_sequence`; this repro independently exercises `rescale_intercept`/`rescale_slope`
//! (via `DecodedPixelData::rescale()`) and `window_center`/`window_width` (via
//! `DecodedPixelData::window()`) with the exact same "gap in the middle of
//! PerFrameFunctionalGroupsSequence" shape, through the real public
//! `dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point.
//!
//! Single external artifact (a real, though internally gapped, `PerFrameFunctionalGroupsSequence`
//! - the same real-world-plausible shape candidate 114 already established), no second
//! participant to fake - CONFIRMATION.md section 11 does not apply.

use dicom_core::value::{DataSetSequence, Value};
use dicom_core::{DataElement, Length, PrimitiveValue, Tag, VR, dicom_value, smallvec::smallvec};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

const PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE: Tag = tags::PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE;

fn frame_group_with_rescale_and_window(intercept: f64, slope: f64, center: f64, width: f64) -> InMemDicomObject {
    let pixel_value_transformation_item = InMemDicomObject::from_element_iter(vec![
        DataElement::new(tags::RESCALE_INTERCEPT, VR::DS, PrimitiveValue::from(intercept.to_string())),
        DataElement::new(tags::RESCALE_SLOPE, VR::DS, PrimitiveValue::from(slope.to_string())),
    ]);
    let frame_voi_lut_item = InMemDicomObject::from_element_iter(vec![
        DataElement::new(tags::WINDOW_CENTER, VR::DS, PrimitiveValue::from(center.to_string())),
        DataElement::new(tags::WINDOW_WIDTH, VR::DS, PrimitiveValue::from(width.to_string())),
    ]);
    InMemDicomObject::from_element_iter(vec![
        DataElement::new(
            tags::PIXEL_VALUE_TRANSFORMATION_SEQUENCE,
            VR::SQ,
            Value::from(DataSetSequence::new(smallvec![pixel_value_transformation_item], Length::UNDEFINED)),
        ),
        DataElement::new(
            tags::FRAME_VOILUT_SEQUENCE,
            VR::SQ,
            Value::from(DataSetSequence::new(smallvec![frame_voi_lut_item], Length::UNDEFINED)),
        ),
    ])
}

fn frame_group_empty() -> InMemDicomObject {
    // A real per-frame functional group item that carries neither PixelValueTransformationSequence
    // nor FrameVOILUTSequence for its frame - the gap.
    InMemDicomObject::from_element_iter(Vec::<DataElement<InMemDicomObject>>::new())
}

fn build_object() -> FileDicomObject<InMemDicomObject> {
    let cols: u16 = 1;
    let rows: u16 = 1;
    let number_of_frames: u32 = 3;

    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::IMPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.777777777777777777777777777777777777777")
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
    obj.put_element(DataElement::new(tags::PIXEL_DATA, VR::OB, PrimitiveValue::from(vec![10u8, 20, 30])));

    // Frame 0: real RescaleIntercept/Slope = 10.0/1.0, WindowCenter/Width = 100.0/50.0.
    // Frame 1: the gap - no PixelValueTransformationSequence, no FrameVOILUTSequence at all.
    // Frame 2: real RescaleIntercept/Slope = 30.0/2.0, WindowCenter/Width = 300.0/80.0.
    let per_frame_items = smallvec![
        frame_group_with_rescale_and_window(10.0, 1.0, 100.0, 50.0),
        frame_group_empty(),
        frame_group_with_rescale_and_window(30.0, 2.0, 300.0, 80.0),
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

    let rescale = decoded.rescale().expect("rescale() should not error (enforce_frame_fg_vm_match is false)");
    println!("decoded.rescale() = {:?}", rescale);

    let window = decoded.window().expect("window() should not error (enforce_frame_fg_vm_match is false)");
    println!("decoded.window() = {:?}", window);

    println!();
    println!(
        "Frame 0's real values: intercept=10.0/slope=1.0, center=100.0/width=50.0. \
         Frame 2's real (different) values: intercept=30.0/slope=2.0, center=300.0/width=80.0. \
         Frame 1 has neither (the gap)."
    );

    // Pre-fix (get_from_per_frame silently skips frame 1's group-less item via .filter_map,
    // producing a misaligned 2-entry array [frame0, frame2] against a declared 3-frame count):
    // the length-mismatch fallback in rescale()/window() then silently returns element 0
    // (frame 0's REAL 10.0/1.0 and 100.0/50.0 values) for use on every frame - looking like
    // legitimate per-frame data instead of the honest "no valid per-frame data" default.
    //
    // Post-fix (get_from_per_frame returns None outright as soon as any item is missing the
    // attribute): rescale_intercept()/window_center() fall through past the (now entirely
    // per-frame-absent) per-frame source to the shared-functional-groups lookup (also absent
    // here) and finally to the documented safe defaults: intercept=0.0/slope=1.0 for rescale,
    // and window() returns None entirely (there is no top-level WindowCenter/Width and no valid
    // per-frame or shared source either).
    let bug_reproduces_rescale = rescale.len() == 1 && rescale[0].intercept == 10.0 && rescale[0].slope == 1.0;
    let bug_reproduces_window = matches!(window, Some(w) if w.len() == 1 && w[0].center == 100.0 && w[0].width == 50.0);

    println!();
    println!(
        "BUG (rescale() silently applies frame 0's real 10.0/1.0 to every frame instead of the \
         documented 0.0/1.0 default): {bug_reproduces_rescale}"
    );
    println!(
        "BUG (window() silently applies frame 0's real 100.0/50.0 to every frame instead of \
         correctly reporting no valid window level, i.e. None): {bug_reproduces_window}"
    );

    if bug_reproduces_rescale || bug_reproduces_window {
        std::process::exit(1);
    }
}
