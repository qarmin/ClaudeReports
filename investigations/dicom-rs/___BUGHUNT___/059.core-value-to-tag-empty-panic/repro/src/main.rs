use dicom_core::value::{PrimitiveValue, Value};
use smallvec::smallvec;

fn main() {
    // A zero-length AT (Attribute Tag) element decodes to an empty `Tags` vector.
    let value: Value = Value::Primitive(PrimitiveValue::Tags(smallvec![]));

    println!("calling Value::to_tag() on an empty Tags value...");
    match value.to_tag() {
        Ok(tag) => println!("unexpectedly got a tag: {tag:?}"),
        Err(e) => println!("correctly got an error: {e}"),
    }
    println!("done, no panic");
}
