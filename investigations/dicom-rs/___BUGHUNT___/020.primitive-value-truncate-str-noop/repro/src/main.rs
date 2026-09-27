use dicom_core::header::HasLength;
use dicom_core::value::PrimitiveValue;

fn main() {
    let mut v = PrimitiveValue::Str("Doe^John".to_string());
    v.truncate(0);

    println!("PrimitiveValue::Str(\"Doe^John\").truncate(0) -> {:?}", v);
    println!("is_empty() -> {}", v.is_empty());

    if !v.is_empty() {
        eprintln!(
            "REPRODUCED - truncate(0) on a single-valued Str left the original value intact \
             instead of reducing it to 0 items, contradicting its own documented contract \
             (\"Truncate a value or a sequence to the given number of items ... does nothing if \
             the cardinality of the element is already lower than or equal to the given size\" - \
             a Str has cardinality 1, which is NOT <= 0)"
        );
        std::process::exit(1);
    }

    // Sanity check: truncate(1) must still be a no-op (a Str's own cardinality, 1, is already
    // <= 1) - the fix must only change behavior for limit == 0.
    let mut v2 = PrimitiveValue::Str("Doe^John".to_string());
    v2.truncate(1);
    assert_eq!(
        v2,
        PrimitiveValue::Str("Doe^John".to_string()),
        "truncate(1) must remain a no-op for a cardinality-1 value"
    );

    println!("did not reproduce - value was correctly reduced to empty, and truncate(1) is still a correct no-op");
}
