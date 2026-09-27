use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

/// A minimal, well-formed explicit-VR-LE data set: a `SpecificCharacterSet`
/// (0008,0005) declaring "ISO_IR 87" (the `encoding` crate's `ISO_2022_JP`
/// codec), followed by an InstitutionName (0008,0080), VR LO, whose value
/// switches into JIS X 0212-1990 mode via the `ESC $ ( D` escape sequence and
/// then encodes one valid JIS X 0212 code point (lead=0x22, trail=0x2f,
/// U+02D8 BREVE). The underlying `encoding` crate's own `ISO2022JPDecoder`
/// (encoding-0.2.33/src/codec/japanese.rs) explicitly documents supporting
/// this escape sequence for *decoding* ("JIS X 0212-1990 (`ESC $ ( D`, XXX
/// asymmetric support)") - but its `ISO2022JPEncoder` has no code path for
/// JIS X 0212 characters at all (only ASCII, JIS X 0201 Katakana, and JIS X
/// 0208 via `index::jis0208::backward`), so re-encoding a successfully
/// decoded JIS X 0212 character always fails under `EncoderTrap::Strict`.
#[rustfmt::skip]
const DATA: &[u8] = &[
    0x08, 0x00, 0x05, 0x00, b'C', b'S', 0x0a, 0x00,
    b'I', b'S', b'O', b'_', b'I', b'R', b' ', b'8', b'7', b' ',
    0x08, 0x00, 0x80, 0x00, b'L', b'O', 0x06, 0x00,
    0x1b, 0x24, 0x28, 0x44, 0x22, 0x2f,
];

fn main() {
    let ts = EXPLICIT_VR_LITTLE_ENDIAN.erased();

    // Real public entry point: parse a data set exactly as a caller reading a
    // DICOM stream/file would.
    let obj = InMemDicomObject::read_dataset_with_ts(DATA, &ts).expect(
        "should decode: ESC $ ( D + 0x22 0x2f is a legitimately mapped JIS X 0212 code point, \
         not an undecodable byte sequence",
    );

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
