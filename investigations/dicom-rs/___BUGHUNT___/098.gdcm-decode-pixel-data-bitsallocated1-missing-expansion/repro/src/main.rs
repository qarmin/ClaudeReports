//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API
//! (the `gdcm` feature - the exact code path in `pixeldata/src/gdcm.rs` used whenever
//! that feature is enabled) on an ordinary, native (non-encapsulated) uncompressed
//! `FileDicomObject<InMemDicomObject>` built entirely through this crate's own public
//! element-insertion API (`put_element`) - the same public entry point any real caller
//! (a DICOM viewer, a PACS client) would use to decode pixel data out of a file.
//!
//! `BitsAllocated = 1` is an ordinary, DICOM-legal attribute value (a real fixture,
//! `pydicom/liver.dcm`, uses it for its bitmap-mask frames). `PixelDecoder::decode_pixel_data`'s
//! `DicomValue::Primitive` branch in `pixeldata/src/gdcm.rs` just returns
//! `p.to_bytes().to_vec()` verbatim, with no 1-bit-to-byte expansion, unlike its `lib.rs`
//! sibling (the non-`gdcm` default implementation) and unlike this same file's own
//! `decode_pixel_data_frame` (candidate 097). The resulting `DecodedPixelData::data` buffer
//! ends up 8x smaller than what `bits_allocated`/`cols`/`rows`/`number_of_frames` imply,
//! which doesn't panic inside `gdcm.rs` itself but makes every downstream consumer that
//! trusts those fields (e.g. `DecodedPixelData::frame_data`/`to_dynamic_image`) fail with
//! `FrameOutOfRange` - reachable simply by decoding any 1-bit-per-pixel image's pixel data
//! and then asking for a frame's data through the `gdcm` feature's decoder.

use dicom_core::{DataElement, PrimitiveValue, VR, dicom_value};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

fn build_object() -> FileDicomObject<InMemDicomObject> {
    // 8x8 = 64 pixels/frame, 1 bit each -> 8 packed bytes/frame, 2 frames.
    let cols: u16 = 8;
    let rows: u16 = 8;
    let number_of_frames: u32 = 2;

    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::IMPLICIT_VR_LITTLE_ENDIAN)
        .media_storage_sop_class_uid(dicom_dictionary_std::uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.111111111111111111111111111111111111111")
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
    obj.put_element(DataElement::new(tags::BITS_ALLOCATED, VR::US, dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::BITS_STORED, VR::US, dicom_value!(U16, [1])));
    obj.put_element(DataElement::new(tags::HIGH_BIT, VR::US, dicom_value!(U16, [0])));
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

    // 2 frames * 8 packed bytes/frame = 16 bytes total, well-formed (exact length).
    let pixel_bytes: Vec<u8> = vec![0xAAu8; 16];
    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        PrimitiveValue::from(pixel_bytes),
    ));

    obj
}

fn main() {
    let obj = build_object();

    let expected_len = 8usize * 8 * 2; // rows*cols*frames, one expanded byte per pixel

    println!("calling decode_pixel_data() via the gdcm feature's native/uncompressed path...");
    let decoded = obj.decode_pixel_data().expect("decode_pixel_data should succeed");
    println!(
        "decode_pixel_data() returned Ok: cols={} rows={} number_of_frames={} data.len()={} (expected {})",
        decoded.columns(),
        decoded.rows(),
        decoded.number_of_frames(),
        decoded.data().len(),
        expected_len,
    );

    if decoded.data().len() != expected_len {
        println!(
            "BUG: data buffer is {}x smaller than implied by bits_allocated/cols/rows/number_of_frames",
            expected_len / decoded.data().len().max(1)
        );
    }

    println!("calling frame_data(1) (second frame) on the decoded result...");
    match decoded.frame_data(1) {
        Ok(_) => println!("frame_data(1) succeeded (no bug observed)"),
        Err(e) => println!("REPRODUCED - frame_data(1) failed: {e}"),
    }
}
