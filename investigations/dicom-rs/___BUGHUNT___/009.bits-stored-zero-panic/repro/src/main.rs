use dicom_core::smallvec::smallvec;
use dicom_core::{DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::tags;
use dicom_object::open_file;
use dicom_pixeldata::PixelDecoder;

fn main() {
    let test_file =
        dicom_test_files::path("pydicom/CT_small.dcm").expect("test file should be available");
    let mut obj = open_file(&test_file).expect("test file should parse");

    obj.put_element(DataElement::new(
        tags::BITS_STORED,
        VR::US,
        PrimitiveValue::U16(smallvec![0u16]),
    ));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let decoded = obj.decode_pixel_data()?;
        decoded.to_vec::<u16>()
    }));

    match result {
        Ok(Ok(_)) => {
            println!("did not reproduce - rendering succeeded despite BitsStored=0");
        }
        Ok(Err(e)) => {
            println!(
                "did not reproduce - rejected gracefully with a normal Result::Err: {:?}",
                e
            );
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
