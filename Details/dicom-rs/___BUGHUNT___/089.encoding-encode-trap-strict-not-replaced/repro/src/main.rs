use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

/// A minimal, well-formed explicit-VR-LE data set: a `SpecificCharacterSet`
/// (0008,0005) declaring "ISO_IR 13" (the Japanese single-byte character set,
/// backed by the `encoding` crate's WINDOWS_31J/Shift-JIS-CP932 codec),
/// followed by an InstitutionName (0008,0080), VR LO, whose value contains one
/// byte (0xF0) from Shift-JIS/CP932's "NEC selection of IBM extended
/// characters" row. That byte is *not* an "undecodable" byte - `decode`
/// legitimately maps it to a single Unicode Private-Use-Area character - but
/// the `encoding` crate's WINDOWS_31J *encoder* table doesn't include a
/// reverse mapping for that PUA character, so encoding it back fails.
#[rustfmt::skip]
const DATA: &[u8] = &[
    0x08, 0x00, 0x05, 0x00, b'C', b'S', 0x0a, 0x00,
    b'I', b'S', b'O', b'_', b'I', b'R', b' ', b'1', b'3', b' ',
    0x08, 0x00, 0x80, 0x00, b'L', b'O', 0x04, 0x00,
    b'A', 0xF0, b'B', b' ',
];

fn main() {
    let ts = EXPLICIT_VR_LITTLE_ENDIAN.erased();

    // Real public entry point: parse a data set exactly as a caller reading a
    // DICOM stream/file would.
    let obj = InMemDicomObject::read_dataset_with_ts(DATA, &ts)
        .expect("should decode: 0xF0 is a legitimately mapped Shift-JIS/CP932 byte, not an undecodable one");

    let value = obj
        .element(dicom_core::Tag(0x0008, 0x0080))
        .expect("InstitutionName element")
        .value();
    println!("decoded value: {:?}", value);

    // Real public entry point: write the object back out, exactly as a
    // caller relaying/re-saving a parsed object would.
    let mut out = Vec::new();
    match obj.write_dataset_with_ts(&mut out, &ts) {
        Ok(()) => {
            println!("round trip OK, {} bytes written", out.len());
        }
        Err(e) => {
            println!(
                "BUG REPRODUCED: a well-formed data set that decoded successfully failed to \
                 write back out: {e}"
            );
            std::process::exit(1);
        }
    }
}
