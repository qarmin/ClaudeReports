use dicom_object::{DefaultDicomObject, open_file};
use dicom_pixeldata::{DecodedPixelData, PixelDecoder, Result};

/// A type that implements `PixelDecoder` by delegating only the required
/// `decode_pixel_data` method - deliberately not overriding
/// `decode_pixel_data_frame`, so calling it exercises the trait's *default*
/// implementation. This is exactly the situation any downstream crate is in
/// if it implements `PixelDecoder` for its own type without overriding that
/// method (which is optional - only `decode_pixel_data` is required).
struct DefaultFrameDecoder(DefaultDicomObject);

impl PixelDecoder for DefaultFrameDecoder {
    fn decode_pixel_data(&self) -> Result<DecodedPixelData<'_>> {
        self.0.decode_pixel_data()
    }
}

fn main() {
    let test_file = dicom_test_files::path("pydicom/SC_rgb_16bit.dcm").unwrap();
    let obj = open_file(test_file).unwrap();
    let wrapper = DefaultFrameDecoder(obj);

    // valid frame index (0) works fine
    let ok = wrapper.decode_pixel_data_frame(0);
    println!("frame 0: {}", if ok.is_ok() { "Ok" } else { "Err" });

    // this file has a single frame - frame index 1 is an ordinary
    // one-past-the-end request. It should return a graceful error, matching
    // the crate's own `PixelData::frame_data` behavior for the same kind of
    // request - not panic.
    println!("requesting out-of-range frame 1...");
    let result = wrapper.decode_pixel_data_frame(1);
    println!("frame 1: {result:?}");
}
