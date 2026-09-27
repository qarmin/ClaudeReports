use dicom_core::smallvec::smallvec;
use dicom_core::value::PrimitiveValue;

fn main() {
    let original: PrimitiveValue =
        PrimitiveValue::Strs(smallvec!["A\\B".to_string(), "C".to_string()]);
    let multi = original.to_multi_str();

    println!("original items: [\"A\\\\B\", \"C\"] (2 items, one containing a literal backslash)");
    println!("to_multi_str() result: {:?} (len {})", multi, multi.len());

    if multi.len() != 2 || multi[0] != "A\\B" || multi[1] != "C" {
        eprintln!(
            "REPRODUCED - to_multi_str() corrupted the item structure: expected [\"A\\\\B\", \
             \"C\"], got {:?}",
            multi
        );
        std::process::exit(1);
    }

    println!("did not reproduce");
}
