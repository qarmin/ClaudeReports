//! Candidate 136: `pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame` reads Planar Configuration
//! with `el.uint16().unwrap_or(0)` inline, instead of calling the shared, validating
//! `attribute::planar_configuration()` helper that every other attribute read in the same
//! function uses (SamplesPerPixel, BitsAllocated, BitsStored, HighBit, PixelRepresentation).
//!
//! That shared helper (before this candidate's fix, only compiled for non-`gdcm` builds) treats
//! a Planar Configuration element that is present but not readable as a `u16` (a non-conformant
//! wire VR) as a real conversion error. The `gdcm.rs` ad-hoc replacement instead collapses that
//! same case into "value is 0 (Standard)", identical to a genuinely absent element - and, unlike
//! the shared helper, never validates that the resulting value is actually 0 or 1 at all.
//!
//! Consequence: for a color image (SamplesPerPixel=3) whose PlanarConfiguration=1 (PixelFirst,
//! "each color plane encoded contiguously") is encoded under a non-conformant wire VR, the
//! `gdcm` build silently treats it as Standard and skips the `interleave_planes()` conversion
//! `decode_pixel_data_frame` would otherwise run - corrupting the returned pixel data's channel
//! order - instead of reporting the malformed element like every sibling attribute read does.
//!
//! Both objects are written out as real DICOM files and read back through `dicom_object::open_file`,
//! so the values under test come out of the crate's real parser. Single external artifact (a DICOM
//! file handed to the public decoding API), no second participant to fake - CONFIRMATION.md
//! section 11 does not apply. Requires the `gdcm` feature (this bug does not exist in the non-gdcm
//! path, which already used the shared, validating helper).

use dicom_core::{dicom_value, DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

fn base_object() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.777777777777777777777777777777777777778")
        .build()
        .unwrap();
    let mut obj = FileDicomObject::new_empty_with_meta(meta);
    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(
        tags::SAMPLES_PER_PIXEL,
        VR::US,
        dicom_value!(U16, [3]),
    ));
    obj.put_element(DataElement::new(
        tags::PHOTOMETRIC_INTERPRETATION,
        VR::CS,
        PrimitiveValue::from("RGB"),
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
    obj.put_element(DataElement::new(
        tags::HIGH_BIT,
        VR::US,
        dicom_value!(U16, [7]),
    ));
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
    // 1x1 RGB pixel: 3 bytes, deliberately distinguishable per channel/plane
    // (R-plane=10, G-plane=20, B-plane=30 if this were genuinely PixelFirst-interleaved
    // across a bigger image; for a single pixel the bytes are just data, the point under
    // test is purely whether the malformed PlanarConfiguration element is reported).
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        PrimitiveValue::from(vec![10u8, 20, 30]),
    ));
    obj
}

fn round_trip(obj: FileDicomObject<InMemDicomObject>, name: &str) -> FileDicomObject<InMemDicomObject> {
    let path = std::env::temp_dir().join(name);
    obj.write_to_file(&path).expect("could not write DICOM file");
    dicom_object::open_file(&path).expect("could not read DICOM file back")
}

fn main() {
    let mut ok = true;

    // PlanarConfiguration present with a non-conformant wire VR (SS instead of US), value 1.
    let mut obj = base_object();
    obj.put_element(DataElement::new(
        tags::PLANAR_CONFIGURATION,
        VR::CS,
        PrimitiveValue::from("YES"),
    ));
    let obj = round_trip(obj, "candidate136_cs_planar_configuration.dcm");
    println!("PlanarConfiguration present, declared VR=CS with a non-numeric text value (malformed wire VR)");
    println!(
        "  parsed value: {:?}",
        obj.element(tags::PLANAR_CONFIGURATION).unwrap().value()
    );
    match obj.decode_pixel_data_frame(0) {
        Ok(_decoded) => {
            println!(
                "  decode_pixel_data_frame(0) = Ok(..) <- BUG: the cast failure was silently \
                 swallowed and treated as PlanarConfiguration=Standard (0), so the \
                 PixelFirst-to-Standard interleave conversion this file actually needs was \
                 silently skipped"
            );
            ok = false;
        }
        Err(e) => {
            println!("  decode_pixel_data_frame(0) = Err({e})  <- correctly reported");
        }
    }

    println!();
    if ok {
        println!("RESULT: the malformed PlanarConfiguration element was correctly reported");
    } else {
        println!("RESULT: the malformed PlanarConfiguration element was silently swallowed (see BUG above)");
        std::process::exit(1);
    }
}
