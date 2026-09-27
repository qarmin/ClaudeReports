use dicom_object::InMemDicomObject;

fn main() {
    // A DICOM+JSON data element that supplies both "Value" and "InlineBinary" for the
    // same element, with "Value" appearing first in the JSON object's key order.
    let json = r#"{"00100010":{"vr":"SH","Value":["abc"],"InlineBinary":"AAAA"}}"#;

    println!("input: {json}");
    let result: Result<InMemDicomObject, _> = dicom_json::from_str(json);
    println!("result: {result:?}");
}
