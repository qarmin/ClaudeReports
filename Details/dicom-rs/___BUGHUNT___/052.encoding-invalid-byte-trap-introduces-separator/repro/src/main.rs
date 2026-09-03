use dicom_core::value::PrimitiveValue;
use dicom_core::Tag;
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

/// A minimal explicit-VR-LE data set: a `SpecificCharacterSet` (0008,0005)
/// declaring UTF-8 (ISO_IR 192), followed by an InstitutionName (0008,0080),
/// VR LO, whose value contains one byte (0x88) that is not valid standalone
/// UTF-8 (a continuation byte with no leading byte) - the kind of byte a
/// real DICOM implementation would refuse to produce, but which a
/// corrupted, truncated, or adversarial file can still contain.
#[rustfmt::skip]
const DATA: &[u8] = &[
    0x08, 0x00, 0x05, 0x00, b'C', b'S', 0x0a, 0x00,
    b'I', b'S', b'O', b'_', b'I', b'R', b' ', b'1', b'9', b'2',
    0x08, 0x00, 0x80, 0x00, b'L', b'O', 0x06, 0x00,
    b'A', b'B', 0x88, b'C', b'D', b' ',
];

fn main() {
    let ts = EXPLICIT_VR_LITTLE_ENDIAN.erased();
    let obj = InMemDicomObject::read_dataset_with_ts(DATA, &ts)
        .expect("should parse despite the invalid byte (that's the whole point of the trap)");

    let value = obj
        .element(Tag(0x0008, 0x0080))
        .expect("InstitutionName element")
        .value();

    let items = match value {
        dicom_core::value::Value::Primitive(PrimitiveValue::Strs(items)) => items,
        other => panic!("unexpected value shape: {other:?}"),
    };

    println!("decoded value: {items:?}");

    let contains_backslash = items.iter().any(|s| s.contains('\\'));
    let item_count = items.len();

    if contains_backslash {
        println!(
            "BUG - the invalid byte was decoded into a literal backslash character, \
             DICOM's own value multiplicity separator"
        );
    }
    if item_count != 1 {
        println!(
            "BUG - decoding a single value produced {item_count} items instead of 1 - \
             the invalid byte's decoded representation was later mistaken for a value \
             multiplicity separator"
        );
    }

    if contains_backslash || item_count != 1 {
        println!(
            "BUG REPRODUCED: an invalid byte under a declared non-default character set was \
             decoded in a way that corrupts the value's structure."
        );
        std::process::exit(1);
    }

    println!("No corruption observed: the invalid byte was replaced without introducing a separator.");
}
