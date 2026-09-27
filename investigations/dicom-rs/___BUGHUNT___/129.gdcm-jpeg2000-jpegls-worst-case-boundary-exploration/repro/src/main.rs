//! Follow-up to candidate 103's own "what's not done": explores whether a *larger* declared-vs-
//! actual size gap (candidate 103 fixed JPEG2000 at 32x32 declared vs 64x64 real) changes the
//! crash's observable severity/character, using the same real, unmodified, genuinely multi-frame
//! `pydicom/emri_small_jpeg_2k_lossless.dcm` fixture (10 real embedded JPEG 2000 frames) via the
//! real public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point (the `gdcm`
//! feature). Each run crashes the whole process - this binary demonstrates exactly one declared
//! size per invocation (passed as argv[1]), so a driver script runs it repeatedly as a fresh
//! subprocess and records the exit signal per size. Frame count itself cannot be independently
//! varied without hand-crafting a larger synthetic multi-frame JPEG2000 fixture from scratch,
//! which was judged out of scope for this bounded follow-up (see RESULT.md).

use dicom_core::header::Tag;
use dicom_core::{DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::tags;
use dicom_object::open_file;
use dicom_pixeldata::PixelDecoder;

fn main() {
    let declared_rows_cols: u16 = std::env::args()
        .nth(1)
        .expect("usage: repro <declared_rows_cols>")
        .parse()
        .expect("declared_rows_cols must be a u16");

    let fixture = "pydicom/emri_small_jpeg_2k_lossless.dcm";
    let test_file = dicom_test_files::path(fixture).unwrap();
    let mut obj = open_file(test_file).unwrap();

    let n_frames = obj.element(tags::NUMBER_OF_FRAMES).unwrap().to_int::<u32>().unwrap();
    println!(
        "fixture={fixture} n_frames={n_frames} real embedded per-frame codestream declares \
         64x64, patching DICOM header Rows/Columns to {declared_rows_cols}x{declared_rows_cols} \
         (gap factor vs real: {:.1}x fewer declared pixels/frame)",
        (64.0 * 64.0) / (declared_rows_cols as f64 * declared_rows_cols as f64)
    );

    obj.put_element(DataElement::new(Tag(0x0028, 0x0010), VR::US, PrimitiveValue::from(declared_rows_cols))); // Rows
    obj.put_element(DataElement::new(Tag(0x0028, 0x0011), VR::US, PrimitiveValue::from(declared_rows_cols))); // Columns

    println!("calling decode_pixel_data() via the gdcm feature's FFI path...");
    let result = obj.decode_pixel_data();
    println!("did not crash - decode_pixel_data() returned: {:?}", result.map(|d| d.data().len()));
}
