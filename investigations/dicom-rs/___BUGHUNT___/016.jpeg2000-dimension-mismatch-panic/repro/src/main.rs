use dicom_core::smallvec::smallvec;
use dicom_core::{DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::tags;
use dicom_pixeldata::PixelDecoder;

fn main() {
    // A real JPEG 2000-encoded DICOM file with a genuine 480x640, 8-bit, 3-samples-per-pixel
    // image, embedded via a real JPEG 2000 codestream. `from_reader` expects no preamble, so
    // skip the standard 128-byte preamble (the following 4 bytes are the "DICM" magic, which
    // `from_reader` itself still expects and checks).
    let bytes: &[u8] = &include_bytes!("../inputs/examples_jpeg2k.dcm")[128..];
    let mut obj = dicom_object::from_reader(bytes).expect("test file should parse");

    // Claim the image is much smaller than the JPEG 2000 codestream actually encodes - the DICOM
    // Rows/Columns attributes are independent metadata from the embedded codestream's own
    // dimensions, and nothing requires them to agree.
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
