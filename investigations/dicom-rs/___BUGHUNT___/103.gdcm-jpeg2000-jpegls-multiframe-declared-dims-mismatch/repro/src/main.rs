//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API (the
//! `gdcm` feature, i.e. the exact FFI path `gdcm-rs`/vendored GDCM 3.1 uses) on a genuine
//! *multi-frame* `pydicom` DICOM fixture (`emri_small_jpeg_2k_lossless.dcm` /
//! `emri_small_jpeg_ls_lossless.dcm`, 10 real embedded JPEG 2000/JPEG-LS frames, one per
//! fragment) opened through `dicom_object::open_file`, with only its `Rows`/`Columns` header
//! attributes patched through this crate's own public element-insertion API (`put_element`) -
//! the embedded compressed pixel data itself is left completely untouched and genuinely valid.
//!
//! This is a follow-on to candidate 100, which found the same declared-vs-actual dimension
//! mismatch bug but only demonstrated it on single-fragment files (`NumberOfFrames` absent /
//! 1, `SequenceOfFragments` with 1 fragment). This repro instead uses real multi-frame fixtures
//! (`fragments.len() == 10`, `NumberOfFrames == 10`) to confirm the same bug is reachable
//! through `pixeldata/src/gdcm.rs`'s `decode_multi_frame_compressed` call (`dims = [cols, rows,
//! number_of_frames]`, i.e. GDCM's own `Dimensions[2] == 10`, not `1`) - genuinely exercising
//! the "process every fragment/frame in a loop" code path inside GDCM's own
//! `JPEG2000Codec::Decode`/`JPEGLSCodec::Decode` (`NumberOfDimensions == 3` branch), not just a
//! single most-1-iteration pass of it.
//!
//! (Note: dicom-rs's own `gdcm_wrapper.cc` always calls `image.SetNumberOfDimensions(3)`
//! regardless of frame count, so both candidate 100's single-fragment repro and this one
//! actually go through the *same* GDCM branch - there is no way to reach GDCM's true
//! `NumberOfDimensions == 2` branch from this crate's `gdcm` feature at all. See this
//! candidate's `RESULT.md` for the correction to candidate 100's original "2-D" labeling.)
//!
//! Run with `--variant jpeg2000` (default) or `--variant jpegls` to pick which one this
//! process demonstrates - each crashes the whole process (SIGSEGV/SIGABRT/glibc heap
//! corruption abort), so both can't be demonstrated in the same run.

use dicom_core::header::Tag;
use dicom_core::{DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::tags;
use dicom_object::open_file;
use dicom_pixeldata::PixelDecoder;

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_else(|| "jpeg2000".to_string());

    let (fixture, declared_rows_cols): (&str, u16) = match variant.as_str() {
        "jpeg2000" => ("pydicom/emri_small_jpeg_2k_lossless.dcm", 32),
        "jpegls" => ("pydicom/emri_small_jpeg_ls_lossless.dcm", 256),
        other => {
            eprintln!("unknown variant {other:?}, expected \"jpeg2000\" or \"jpegls\"");
            std::process::exit(2);
        }
    };

    let test_file = dicom_test_files::path(fixture).unwrap();
    let mut obj = open_file(test_file).unwrap();

    let rows_before = obj.element(tags::ROWS).unwrap().to_int::<u16>().unwrap();
    let cols_before = obj.element(tags::COLUMNS).unwrap().to_int::<u16>().unwrap();
    let n_frames = obj
        .element(tags::NUMBER_OF_FRAMES)
        .unwrap()
        .to_int::<u32>()
        .unwrap();
    println!(
        "variant={variant} fixture={fixture} n_frames={n_frames} real embedded per-frame \
         codestream declares {rows_before}x{cols_before}, patching DICOM header Rows/Columns to \
         {declared_rows_cols}x{declared_rows_cols}"
    );

    obj.put_element(DataElement::new(
        Tag(0x0028, 0x0010),
        VR::US,
        PrimitiveValue::from(declared_rows_cols),
    )); // Rows
    obj.put_element(DataElement::new(
        Tag(0x0028, 0x0011),
        VR::US,
        PrimitiveValue::from(declared_rows_cols),
    )); // Columns

    println!("calling decode_pixel_data() via the gdcm feature's FFI path...");
    let result = obj.decode_pixel_data();
    // If we get here without the process being killed by a signal, the bug did not fire on
    // this run - print the outcome so a caller can tell reproduced-vs-not from stdout alone.
    println!(
        "did not crash - decode_pixel_data() returned: {:?}",
        result.map(|d| d.data().len())
    );
}
