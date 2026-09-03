//! Reproduction for: `pixeldata/src/overlay.rs` silently substitutes the DICOM
//! spec default for Overlay Bits Allocated (60xx,0100), Overlay Bit Position
//! (60xx,0102) and Samples per Pixel (0028,0002) when the element IS present
//! but its value cannot be read, and then runs the "is this value the
//! spec-mandated one?" validation against that substituted default.
//!
//! Each scenario below is written out as a real Explicit VR Little Endian
//! DICOM stream and read back with the crate's own parser
//! (`FileDicomObject::write_all` -> `dicom_object::from_reader`), so the
//! in-memory state fed to `PixelDecoder::decode_overlay` is genuinely produced
//! by the real DICOM reader from real bytes, not hand-assembled.
//!
//! The only difference between the "malformed" and the "baseline" object of
//! each scenario is the VR the offending element is encoded with: the value
//! itself is the same non-conformant number in both.

use dicom_core::{DataElement, PrimitiveValue, Tag, VR, dicom_value};
use dicom_dictionary_std::{tags, uids};
use dicom_object::{DefaultDicomObject, FileDicomObject, FileMetaTableBuilder};
use dicom_pixeldata::PixelDecoder;

fn empty_object() -> DefaultDicomObject {
    FileDicomObject::new_empty_with_meta(
        FileMetaTableBuilder::new()
            .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
            .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
            .media_storage_sop_instance_uid("2.25.221545913343461877066277885826230616209")
            .build()
            .unwrap(),
    )
}

/// A conformant overlay plane in `group`, with Overlay Data present.
fn put_overlay_plane(obj: &mut DefaultDicomObject, group: u16, rows: u16, columns: u16) {
    obj.put_element(DataElement::new(Tag(group, 0x0010), VR::US, dicom_value!(U16, rows)));
    obj.put_element(DataElement::new(Tag(group, 0x0011), VR::US, dicom_value!(U16, columns)));
    obj.put_element(DataElement::new(Tag(group, 0x0040), VR::CS, PrimitiveValue::from("R")));
    obj.put_element(DataElement::new(Tag(group, 0x0050), VR::SS, dicom_value!(I16, [1, 1])));
    obj.put_element(DataElement::new(Tag(group, 0x0100), VR::US, dicom_value!(U16, 1)));
    obj.put_element(DataElement::new(Tag(group, 0x0102), VR::US, dicom_value!(U16, 0)));
    let num_bytes = (rows as usize * columns as usize).div_ceil(8).div_ceil(2) * 2;
    obj.put_element(DataElement::new(
        Tag(group, 0x3000),
        VR::OB,
        PrimitiveValue::from(vec![0b0000_0011u8; num_bytes]),
    ));
}

/// A 2x4 16-bit image with a legacy overlay embedded at bit 15 of each sample
/// (no Overlay Data element, so the embedded-overlay path is taken).
fn embedded_overlay_object() -> DefaultDicomObject {
    let mut obj = empty_object();
    obj.put_element(DataElement::new(tags::ROWS, VR::US, dicom_value!(U16, 2)));
    obj.put_element(DataElement::new(tags::COLUMNS, VR::US, dicom_value!(U16, 4)));
    obj.put_element(DataElement::new(tags::BITS_ALLOCATED, VR::US, dicom_value!(U16, 16)));
    let mut samples = vec![0x0123u16; 8];
    samples[0] |= 0x8000;
    samples[5] |= 0x8000;
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OW,
        PrimitiveValue::U16(samples.into()),
    ));

    put_overlay_plane(&mut obj, 0x6000, 2, 4);
    obj.remove_element(Tag(0x6000, 0x3000));
    obj.put_element(DataElement::new(Tag(0x6000, 0x0100), VR::US, dicom_value!(U16, 16)));
    obj.put_element(DataElement::new(Tag(0x6000, 0x0102), VR::US, dicom_value!(U16, 15)));
    obj
}

/// Serialize to a real DICOM stream and read it back with the real parser.
fn roundtrip(obj: &DefaultDicomObject) -> DefaultDicomObject {
    let mut buf = Vec::new();
    obj.write_all(&mut buf).expect("failed to write DICOM stream");
    dicom_object::from_reader(&buf[..]).expect("failed to read DICOM stream back")
}

fn outcome(obj: &DefaultDicomObject) -> String {
    match obj.decode_overlay(0) {
        Ok(Some(plane)) => format!("Ok(Some(plane with {} pixels))", plane.data().len()),
        Ok(None) => "Ok(None)".to_string(),
        Err(e) => format!("Err({e})"),
    }
}

/// Returns true if this scenario still shows the bug.
fn check(name: &str, malformed: &DefaultDicomObject, baseline: &DefaultDicomObject) -> bool {
    let malformed_out = outcome(&roundtrip(malformed));
    let baseline_out = outcome(&roundtrip(baseline));
    println!("  correct VR (baseline) -> {baseline_out}");
    println!("  wrong VR (malformed)  -> {malformed_out}");
    let diverges = !malformed_out.starts_with("Err(") && baseline_out.starts_with("Err(");
    if diverges {
        println!("  BUG: {name}: the malformed element was silently accepted as the spec default");
    } else {
        println!("  OK: {name}: the malformed element is reported instead of silently defaulted");
    }
    diverges
}

fn main() {
    let mut bugs = 0;

    // ---------------------------------------------------------------
    // A) Overlay Bits Allocated (6000,0100) = 16 (non-conformant; only 1
    //    is supported), encoded with VR SS instead of US.
    println!("A) Overlay Bits Allocated = 16, VR SS instead of US");
    let mut malformed = empty_object();
    put_overlay_plane(&mut malformed, 0x6000, 4, 8);
    malformed.put_element(DataElement::new(Tag(0x6000, 0x0100), VR::SS, dicom_value!(I16, 16)));
    let mut baseline = empty_object();
    put_overlay_plane(&mut baseline, 0x6000, 4, 8);
    baseline.put_element(DataElement::new(Tag(0x6000, 0x0100), VR::US, dicom_value!(U16, 16)));
    bugs += check("Overlay Bits Allocated", &malformed, &baseline) as u32;

    // ---------------------------------------------------------------
    // B) Overlay Bit Position (6000,0102) = 3 (non-conformant for a plane
    //    stored in Overlay Data; only 0 is supported), VR SS instead of US.
    println!("\nB) Overlay Bit Position = 3, VR SS instead of US");
    let mut malformed = empty_object();
    put_overlay_plane(&mut malformed, 0x6000, 4, 8);
    malformed.put_element(DataElement::new(Tag(0x6000, 0x0102), VR::SS, dicom_value!(I16, 3)));
    let mut baseline = empty_object();
    put_overlay_plane(&mut baseline, 0x6000, 4, 8);
    baseline.put_element(DataElement::new(Tag(0x6000, 0x0102), VR::US, dicom_value!(U16, 3)));
    bugs += check("Overlay Bit Position", &malformed, &baseline) as u32;

    // ---------------------------------------------------------------
    // C) Samples per Pixel (0028,0002) = 3 (unsupported for an embedded
    //    overlay), VR SS instead of US.
    println!("\nC) Samples per Pixel = 3, VR SS instead of US (embedded overlay)");
    let mut malformed = embedded_overlay_object();
    malformed.put_element(DataElement::new(tags::SAMPLES_PER_PIXEL, VR::SS, dicom_value!(I16, 3)));
    let mut baseline = embedded_overlay_object();
    baseline.put_element(DataElement::new(tags::SAMPLES_PER_PIXEL, VR::US, dicom_value!(U16, 3)));
    bugs += check("Samples per Pixel", &malformed, &baseline) as u32;

    // ---------------------------------------------------------------
    // D) Regression guard for the fix: an absent element must still resolve
    //    to its spec default, and a conformant plane must still decode.
    println!("\nD) control: the same attributes absent entirely (must keep working)");
    let mut obj = empty_object();
    put_overlay_plane(&mut obj, 0x6000, 4, 8);
    obj.remove_element(Tag(0x6000, 0x0100));
    obj.remove_element(Tag(0x6000, 0x0102));
    let control = outcome(&roundtrip(&obj));
    println!("  absent Overlay Bits Allocated + Overlay Bit Position -> {control}");
    let control_ok = control.starts_with("Ok(Some(");
    if !control_ok {
        println!("  REGRESSION: an absent element no longer falls back to the spec default");
    }

    println!("\n{bugs} of 3 sites silently accepted a malformed element as the spec default");
    if bugs > 0 || !control_ok {
        std::process::exit(1);
    }
    println!("all sites report the malformed element, and absent elements still default");
}
