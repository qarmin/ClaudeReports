use dicom_core::value::DicomDate;
use dicom_core::{DataElement, PrimitiveValue, Tag, VR};
use dicom_object::InMemDicomObject;

fn main() {
    // A data element whose declared VR (DS - Decimal String, dispatched by
    // the JSON serializer to its "encode as numbers" path) doesn't match its
    // actually-stored PrimitiveValue variant (Date) - constructible directly
    // via the public API (put_element/from_element_iter), or via
    // AttributeAction::SetVr, which is documented to change an element's VR
    // without touching its stored value.
    let obj = InMemDicomObject::from_element_iter([DataElement::new(
        Tag(0x0008, 0x0020),
        VR::DS,
        PrimitiveValue::from(DicomDate::from_ymd(2013, 4, 9).unwrap()),
    )]);

    println!("serializing object with mismatched VR/value...");
    let result = dicom_json::to_string(&obj);
    println!("result: {result:?}");
}
