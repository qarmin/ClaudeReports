use dicom_encoding::decode::Decode;
use dicom_encoding::decode::explicit_be::ExplicitVRBigEndianDecoder;

fn main() {
    // (0010,0040) VR=CS, then EOF - no length bytes follow, even though CS is a "short form"
    // VR needing exactly 2 more bytes for its value length.
    let wire: &[u8] = &[0x00, 0x10, 0x00, 0x40, b'C', b'S'];
    let mut cursor = std::io::Cursor::new(wire);

    let decoder = ExplicitVRBigEndianDecoder::default();
    let err = decoder
        .decode_header(&mut cursor)
        .expect_err("should fail: truncated before the length field");

    let message = err.to_string();
    println!("error message: {message}");

    if message.contains("item length") {
        eprintln!(
            "REPRODUCED - wrong error context: failing to read a short-form VR's 2-byte *element* \
             length field reports \"{message}\" (an item-length message) instead of an \
             element-length message"
        );
        std::process::exit(1);
    }

    println!("did not reproduce - correct error message");
}
