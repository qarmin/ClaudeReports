use dicom_encoding::decode::basic::LittleEndianBasicDecoder;
use dicom_encoding::decode::explicit_le::ExplicitVRLittleEndianDecoder;
use dicom_encoding::text::SpecificCharacterSet;
use dicom_parser::{StatefulDecode, StatefulDecoder};
use std::io::Cursor;

fn main() {
    // This replicates what happens when `dicom_parser::dataset::read::DataSetReader`
    // reads the Basic Offset Table item of an encapsulated PixelData sequence:
    // it calls `read_u32_to_vec(len, ...)` where `len` is the item's own
    // declared byte length, taken directly from the file/stream (fully
    // attacker/file-controlled). Here `len = 5` - a full u32 (4 bytes) plus 1
    // extra byte, not a multiple of 4.
    let raw: &[u8] = &[0x01, 0x00, 0x00, 0x00, 0xFF];
    let mut cursor = Cursor::new(raw);
    let mut decoder = StatefulDecoder::new(
        &mut cursor,
        ExplicitVRLittleEndianDecoder::default(),
        LittleEndianBasicDecoder,
        SpecificCharacterSet::default(),
    );

    let mut vec = Vec::new();
    println!("reading a 5-byte offset table (not a multiple of 4)...");
    match decoder.read_u32_to_vec(5, &mut vec) {
        Ok(()) => {
            println!("read succeeded: {vec:?}, decoder position = {}", decoder.position());
            assert_eq!(
                decoder.position(),
                5,
                "BUG: only {} of the declared 5 bytes were consumed - the stream is now \
                 desynchronized by {} byte(s), corrupting whatever gets parsed next \
                 (e.g. the following item's header)",
                decoder.position(),
                5 - decoder.position()
            );
        }
        Err(e) => {
            println!("read rejected with an error (correct behavior): {e}");
        }
    }
    println!("done");
}
