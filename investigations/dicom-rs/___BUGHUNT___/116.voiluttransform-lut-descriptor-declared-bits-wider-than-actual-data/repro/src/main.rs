//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API
//! (`decode_pixel_data()` + `to_vec_frame(0)`), on a native (non-encapsulated), uncompressed,
//! 16-bits-allocated/12-bits-stored monochrome `FileDicomObject<InMemDicomObject>` built
//! entirely through this crate's own public element-insertion API (`put_element`), the same
//! public entry point any real caller (a DICOM viewer rendering an image) would use.
//!
//! Mirrors the real-world device behavior described in upstream PR `Enet4/dicom-rs#826`'s
//! sibling report `Enet4/dicom-rs#825` (Agfa CR, Horos, Cornerstone): the VOI LUT Sequence's
//! _LUT Descriptor_ third value declares 16 bits per entry, but the actual _LUT Data_ entries
//! only ever span 12 bits (max value 4095) - the real bit depth the LUT's own data uses, not
//! what its own descriptor happens to declare.

use dicom_core::value::{DataSetSequence, Value};
use dicom_core::{DataElement, Length, PrimitiveValue, Tag, VR, dicom_value, smallvec::smallvec};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::{ConvertOptions, PixelDecoder, VoiLutOption};

const VOILUT_SEQUENCE: Tag = tags::VOILUT_SEQUENCE;

fn build_object() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::IMPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.444444444444444444444444444444444444444")
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
    // 16-bit allocated / 12-bit stored - a very common real-world combination for CR/DX
    // devices (e.g. Agfa CR).
    obj.put_element(DataElement::new(tags::BITS_ALLOCATED, VR::US, dicom_value!(U16, [16])));
    obj.put_element(DataElement::new(tags::BITS_STORED, VR::US, dicom_value!(U16, [12])));
    obj.put_element(DataElement::new(tags::HIGH_BIT, VR::US, dicom_value!(U16, [11])));
    obj.put_element(DataElement::new(
        tags::PIXEL_REPRESENTATION,
        VR::US,
        dicom_value!(U16, [0]),
    ));

    // Single pixel, stored value 1: with LUT Descriptor's min_pixel_value = 0, this selects
    // the LUT's second (index 1) entry.
    let pixel_value: u16 = 1;
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OW,
        dicom_value!(U16, [pixel_value]),
    ));

    // VOI LUT Sequence: LUT Descriptor declares 16 bits per entry (the wire value real
    // Agfa CR files have been observed sending), but the actual LUT Data entries (0, 4095)
    // only ever span 12 bits - matching PR #825's own description exactly.
    let lut_descriptor = dicom_value!(I16, [2, 0, 16]); // [num_entries, min_pixel_value, bits_stored]
    let lut_data = dicom_value!(U16, [0, 4095]);
    let voi_lut_item = InMemDicomObject::from_element_iter(vec![
        DataElement::new(tags::LUT_DESCRIPTOR, VR::SS, lut_descriptor),
        DataElement::new(tags::LUT_DATA, VR::US, lut_data),
    ]);
    obj.put_element(DataElement::new(
        VOILUT_SEQUENCE,
        VR::SQ,
        Value::from(DataSetSequence::new(smallvec![voi_lut_item], Length::UNDEFINED)),
    ));

    obj
}

fn main() {
    let obj = build_object();

    let decoded = obj.decode_pixel_data().expect("decode_pixel_data should succeed");
    let seq = decoded
        .voi_lut_sequence()
        .expect("voi_lut_sequence() should succeed")
        .expect("a VOI LUT Sequence was set on this object");
    println!(
        "VOI LUT as parsed: min_pixel_value={} bits_stored (declared)={} data={:?} (actual max bit-width used: {})",
        seq[0].min_pixel_value,
        seq[0].bits_stored,
        seq[0].data,
        16 - (seq[0].data.iter().copied().max().unwrap_or(0)).leading_zeros(),
    );

    // `to_vec_frame`/`convert_pixel_slice` never even considers the explicit VOI LUT Sequence
    // table (only WindowCenter/WindowWidth) - `VoiLutTransform` is only reachable through the
    // image-rendering path, so this repro must use `to_dynamic_image` (with the default
    // `VoiLutOption::Default`, which does apply the VOI LUT table for image output) to reach
    // the real, affected code path any DICOM viewer would use.
    let options = ConvertOptions::default().with_voi_lut(VoiLutOption::Default);
    let image = decoded
        .to_dynamic_image_with_options(0, &options)
        .expect("to_dynamic_image_with_options(0) should succeed")
        .to_luma16();
    let pixel = image.get_pixel(0, 0)[0];

    // Correct behavior: the LUT's single relevant entry (index 1, value 4095) must be
    // rescaled from its ACTUAL 12-bit span up to the full 16-bit output range, i.e.
    // shifted left by (16 - 12) = 4 bits: 4095 << 4 = 65520 (near-white, as the LUT's own
    // ramp intends). If the declared-but-wrong 16-bit depth is trusted instead, no shift is
    // applied at all and the raw LUT entry (4095, ~6% of the 16-bit range) is used verbatim -
    // a hard-to-notice-in-code but obviously-wrong "washed out to near-black" rendering for
    // any real image using this LUT.
    println!("decoded 16-bit pixel value = {pixel}");
    println!("expected (correct, shifted to actual bit depth) = 65520");
    println!("buggy value if the declared-but-wrong bits_stored is trusted instead = 4095");

    if pixel == 4095 {
        println!("BUG REPRODUCED: LUT entry used verbatim at its own (wrong) declared bit depth, image rendered far too dark.");
        std::process::exit(1);
    } else if pixel == 65520 {
        println!("OK: LUT entry correctly rescaled to the actual (not declared) bit depth it uses.");
    } else {
        println!("UNEXPECTED value - neither the known-buggy nor the known-correct result.");
        std::process::exit(2);
    }
}
