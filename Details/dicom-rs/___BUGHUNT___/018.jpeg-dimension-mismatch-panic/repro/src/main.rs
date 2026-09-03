use dicom_core::smallvec::smallvec;
use dicom_core::{DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::tags;
use dicom_object::open_file;
use dicom_pixeldata::PixelDecoder;

fn main() {
    let path = dicom_test_files::path("pydicom/SC_rgb_jpeg_gdcm.dcm")
        .expect("test file should be available");
    let mut obj = open_file(&path).expect("test file should parse");

    let real_rows = obj
        .element(tags::ROWS)
        .unwrap()
        .to_int::<u16>()
        .unwrap();
    let real_cols = obj
        .element(tags::COLUMNS)
        .unwrap()
        .to_int::<u16>()
        .unwrap();
    println!("real image dimensions: {real_rows}x{real_cols}");

    // Claim the image is much smaller than the JPEG codestream actually encodes - Rows/Columns
    // are DICOM-level metadata, independent of the embedded JPEG's own dimensions.
    obj.put_element(DataElement::new(
        tags::ROWS,
        VR::US,
        PrimitiveValue::U16(smallvec![4]),
    ));
    obj.put_element(DataElement::new(
        tags::COLUMNS,
        VR::US,
        PrimitiveValue::U16(smallvec![4]),
    ));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let decoded = obj.decode_pixel_data()?;
        decoded.to_vec::<u8>()
    }));

    match result {
        Ok(Ok(v)) => println!(
            "did not reproduce - decode succeeded, produced {} bytes for a declared 4x4x3 image",
            v.len()
        ),
        Ok(Err(e)) => println!(
            "did not reproduce - decode failed gracefully with a normal Result::Err: {:?}",
            e
        ),
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
