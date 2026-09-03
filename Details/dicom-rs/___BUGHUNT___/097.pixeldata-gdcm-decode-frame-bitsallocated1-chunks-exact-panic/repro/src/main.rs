//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API
//! (the `gdcm` feature - the exact code path in `pixeldata/src/gdcm.rs` used whenever
//! that feature is enabled) on an ordinary, native (non-encapsulated) uncompressed
//! `FileDicomObject<InMemDicomObject>` built entirely through this crate's own public
//! element-insertion API (`put_element`) - the same public entry point any real caller
//! (a DICOM viewer, a PACS client) would use to decode a single frame out of a
//! multi-frame file.
//!
//! `BitsAllocated = 1` is an ordinary, DICOM-legal attribute value (a real fixture,
//! `pydicom/liver.dcm`, uses it for its bitmap-mask frames) - `PixelDecoder::decode_pixel_data_frame`'s
//! `DicomValue::Primitive` branch in `pixeldata/src/gdcm.rs` computed
//! `frame_size = cols * rows * samples_per_pixel * (bits_allocated / 8)`; integer
//! division truncates `1 / 8` to `0`, so `frame_size = 0`, and the subsequent
//! `data.chunks_exact(frame_size)` call panics with "chunk size must be non-zero" -
//! reachable simply by asking for any frame of any 1-bit-per-pixel image through the
//! `gdcm` feature's decoder.

use dicom_core::{DataElement, PrimitiveValue, VR, dicom_value};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

fn build_object() -> FileDicomObject<InMemDicomObject> {
    // 8x8 = 64 pixels/frame, 1 bit each -> 8 packed bytes/frame, 2 frames.
    let cols: u16 = 8;
    let rows: u16 = 8;

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
        PrimitiveValue::from("2"),
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

    println!("calling decode_pixel_data_frame(1) via the gdcm feature's native/uncompressed path...");
    match obj.decode_pixel_data_frame(1) {
        Ok(decoded) => {
            println!(
                "did not panic - decode_pixel_data_frame(1) returned Ok: cols={} rows={} number_of_frames={} data.len()={}",
                decoded.columns(),
                decoded.rows(),
                decoded.number_of_frames(),
                decoded.data().len()
            );
        }
        Err(e) => {
            println!("did not panic - decode_pixel_data_frame(1) returned a graceful Err: {e}");
        }
    }
}
