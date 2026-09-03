use dicom_object::open_file;
use dicom_pixeldata::PixelDecoder;

fn main() {
    let test_file = dicom_test_files::path("pydicom/JPEG2000.dcm").unwrap();
    let obj = open_file(test_file).unwrap();

    // this file has a single frame / single fragment - frame index 1 is an
    // ordinary one-past-the-end request. It should return a graceful error,
    // not panic.
    println!("requesting out-of-range frame 1 via the GDCM decode path...");
    let result = obj.decode_pixel_data_frame(1);
    println!("frame 1: {result:?}");
}
