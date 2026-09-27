use dicom_core::header::{DataElement, Tag};
use dicom_core::value::PrimitiveValue;
use dicom_core::VR;
use dicom_object::InMemDicomObject;

fn main() {
    // A multi-valued LO (Long String) element with 3 items: an ordinary
    // string, a middle item that is *itself* a single space (a legitimate,
    // if unusual, value - not padding, since DICOM only ever pads the last
    // item of a joined multi-value string to make the whole wire
    // representation even-length), and a last item with its own trailing
    // space (which *is* legitimate padding, and should be trimmed).
    let elem = DataElement::new(
        Tag(0x0008, 0x0100),
        VR::LO,
        PrimitiveValue::Strs(vec!["a".to_string(), " ".to_string(), "b ".to_string()].into()),
    );

    let mut obj = InMemDicomObject::new_empty();
    obj.put(elem);

    let json = dicom_json::to_string(&obj).expect("should serialize");
    println!("JSON: {json}");

    let obj2: InMemDicomObject = dicom_json::from_str(&json).expect("should deserialize");

    if obj == obj2 {
        println!("No corruption observed: the middle item's whitespace content survived.");
    } else {
        println!("{:#?}", obj);
        println!("====");
        println!("{:#?}", obj2);
        println!(
            "BUG REPRODUCED: the middle item's whitespace-only content was silently trimmed \
             away during JSON serialization, corrupting the round trip."
        );
        std::process::exit(1);
    }
}
