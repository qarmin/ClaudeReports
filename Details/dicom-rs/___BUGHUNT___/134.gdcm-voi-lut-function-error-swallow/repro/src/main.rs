//! Candidate 134: `pixeldata`'s two `PixelDecoder` implementations disagree about a
//! VOILUTFunction (0028,1056) element that is present but not usable as a string.
//!
//! `pixeldata/src/attribute.rs`'s `voi_lut_function()` returns `Err(CastValue)` both for a
//! present-but-zero-length element (a legal way of leaving this Type 3 attribute unspecified)
//! and for an element whose value genuinely is not text (e.g. a file declaring VR `US` on the
//! wire for this tag). The two decoders then handle that `Err` differently:
//!
//! * `pixeldata/src/lib.rs` (`ImagingProperties::from_obj`, non-`gdcm` build): `voi_lut_function(obj)?`
//!   - propagates, so `decode_pixel_data()` fails outright, including for the legal empty case.
//! * `pixeldata/src/gdcm.rs` (`gdcm` build): `voi_lut_function(self).unwrap_or(None)`
//!   - swallows, so a genuinely malformed value is silently rendered as if the attribute had
//!     never been specified (falls back to `VoiLutFunction::Linear`).
//!
//! So the crate's observable public behaviour for the exact same file depends on a Cargo feature
//! flag, and each path is wrong for one of the two input classes.
//!
//! This binary checks the behaviour that both paths should agree on, and is run twice by the
//! justfile: once without the `gdcm` feature and once with it.
//!
//! Both DICOM objects are written out as real files and re-read through
//! `dicom_object::open_file`, so the values under test come out of the crate's real parser, not
//! from hand-placed in-memory `PrimitiveValue`s. Single external artifact (a DICOM file handed to
//! the public decoding API), no second participant to fake - CONFIRMATION.md section 11 does not
//! apply.

use dicom_core::{dicom_value, DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

fn base_object() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.777777777777777777777777777777777777777")
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
    obj.put_element(DataElement::new(
        tags::WINDOW_CENTER,
        VR::DS,
        PrimitiveValue::from("100"),
    ));
    obj.put_element(DataElement::new(
        tags::WINDOW_WIDTH,
        VR::DS,
        PrimitiveValue::from("50"),
    ));
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        PrimitiveValue::from(vec![10u8, 20]),
    ));
    obj
}

/// Write the object out as a real DICOM file and read it back through the public parser.
fn round_trip(obj: FileDicomObject<InMemDicomObject>, name: &str) -> FileDicomObject<InMemDicomObject> {
    let path = std::env::temp_dir().join(name);
    obj.write_to_file(&path).expect("could not write DICOM file");
    dicom_object::open_file(&path).expect("could not read DICOM file back")
}

fn main() {
    let gdcm = cfg!(feature = "gdcm");
    println!(
        "decode path under test: {}",
        if gdcm {
            "pixeldata/src/gdcm.rs (gdcm feature ON)"
        } else {
            "pixeldata/src/lib.rs (gdcm feature OFF)"
        }
    );

    let mut ok = true;

    // Case A: VOILUTFunction present with a zero-length value.
    // VOILUTFunction is a Type 3 (optional) attribute; a zero-length element is a legal way of
    // saying "not specified", so decoding must succeed and simply report no VOI LUT function.
    let mut obj = base_object();
    obj.put_element(DataElement::new(
        tags::VOILUT_FUNCTION,
        VR::CS,
        PrimitiveValue::Empty,
    ));
    let obj = round_trip(obj, "candidate134_empty_voi_lut_function.dcm");
    println!();
    println!("case A: VOILUTFunction present, zero-length (legal 'unspecified')");
    println!("  parsed value: {:?}", obj.element(tags::VOILUT_FUNCTION).unwrap().value());
    match obj.decode_pixel_data() {
        Ok(decoded) => {
            let f = decoded.voi_lut_function();
            println!("  decode_pixel_data() = Ok, voi_lut_function() = {:?}", f);
            if !matches!(f, Ok(None)) {
                println!("  UNEXPECTED: expected no VOI LUT function");
                ok = false;
            }
        }
        Err(e) => {
            println!("  decode_pixel_data() = Err({e})");
            println!("  BUG: a legal empty Type 3 VOILUTFunction makes the whole decode fail");
            ok = false;
        }
    }

    // Case B: VOILUTFunction present with a non-textual value (VR US on the wire).
    // This is a genuinely malformed value, not an absent one - it must not be silently ignored.
    let mut obj = base_object();
    obj.put_element(DataElement::new(
        tags::VOILUT_FUNCTION,
        VR::US,
        dicom_value!(U16, [7u16]),
    ));
    let obj = round_trip(obj, "candidate134_us_voi_lut_function.dcm");
    println!();
    println!("case B: VOILUTFunction present, declared VR=US with a numeric value (malformed)");
    println!("  parsed value: {:?}", obj.element(tags::VOILUT_FUNCTION).unwrap().value());
    match obj.decode_pixel_data() {
        Ok(decoded) => {
            println!(
                "  decode_pixel_data() = Ok, voi_lut_function() = {:?}",
                decoded.voi_lut_function()
            );
            println!(
                "  BUG: the conversion error was swallowed - the image is rendered as if no VOI \
                 LUT function had been specified at all (falls back to VoiLutFunction::Linear)"
            );
            ok = false;
        }
        Err(e) => {
            println!("  decode_pixel_data() = Err({e})  <- correctly reported");
        }
    }

    println!();
    if ok {
        println!("RESULT: both cases behave correctly on this decode path");
    } else {
        println!("RESULT: this decode path mishandles at least one of the two cases (see BUG above)");
        std::process::exit(1);
    }
}
