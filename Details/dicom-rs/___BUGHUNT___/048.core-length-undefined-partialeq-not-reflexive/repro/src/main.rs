use dicom_core::header::{DataElementHeader, HasLength, SequenceItemHeader};
use dicom_core::value::{DataSetSequence, Value};
use dicom_core::{DataElement, Length, PrimitiveValue, Tag, VR};
use dicom_object::InMemDicomObject;

fn main() {
    // 1. The most minimal form of the bug: a bare `Length::UNDEFINED` does
    //    not equal itself. This is documented, intentional behavior for
    //    `Length` alone (see its own doc test), analogous to floating-point
    //    NaN - but it was never intended to propagate to structs/enums that
    //    *contain* a `Length` and derive `PartialEq` naively.
    println!(
        "Length::UNDEFINED == Length::UNDEFINED: {}",
        Length::UNDEFINED == Length::UNDEFINED
    );

    // 2. DataElementHeader: an ordinary SQ header (undefined length is the
    //    normal encoding for SQ) does not equal a copy of itself.
    let header = DataElementHeader {
        tag: Tag(0x0040, 0xa168),
        vr: VR::SQ,
        len: Length::UNDEFINED,
    };
    println!("DataElementHeader == itself: {}", header == header);

    // 3. SequenceItemHeader::Item: an ordinary item with undefined length
    //    (equally normal encoding) does not equal a copy of itself either.
    let item_header = SequenceItemHeader::Item {
        len: Length::UNDEFINED,
    };
    println!(
        "SequenceItemHeader::Item == itself: {}",
        item_header == item_header
    );

    // 4. Real-world impact: an ordinary InMemDicomObject containing a single
    //    SQ element (undefined length) does not equal its own exact clone -
    //    breaking `==`/`assert_eq!` for any object containing a sequence or
    //    encapsulated pixel data, which is extremely common in real files.
    let inner = InMemDicomObject::from_element_iter([DataElement::new(
        Tag(0x0008, 0x0100),
        VR::SH,
        PrimitiveValue::from("x"),
    )]);
    let mut obj = InMemDicomObject::new_empty();
    obj.put(DataElement::new(
        Tag(0x0040, 0xa168),
        VR::SQ,
        Value::Sequence(DataSetSequence::new(vec![inner], Length::UNDEFINED)),
    ));

    let clone = obj.clone();
    println!(
        "InMemDicomObject (with an SQ element) == its own clone: {}",
        obj == clone
    );
    println!(
        "InMemDicomObject::length() == its own clone's length(): {}",
        obj.length() == clone.length()
    );

    if header != header || item_header != item_header || obj != clone {
        println!("BUG: reflexivity (`x == x`) is violated for one or more of the above.");
        std::process::exit(1);
    }
}
