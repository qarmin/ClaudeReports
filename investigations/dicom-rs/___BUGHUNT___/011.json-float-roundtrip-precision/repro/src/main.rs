fn main() {
    let literal = "0.09510497055947781";
    let json_in = format!(
        r#"{{"00089459":{{"vr":"SQ","Value":[{{"00189078":{{"vr":"FD","Value":[{literal}]}}}}]}}}}"#
    );

    let obj: dicom_object::InMemDicomObject =
        dicom_json::from_str(&json_in).expect("should parse");
    let json_out = dicom_json::to_string(obj).expect("should serialize");

    println!("input:  {json_in}");
    println!("output: {json_out}");

    if !json_out.contains(literal) {
        eprintln!(
            "REPRODUCED - the FD literal changed across a single deserialize/serialize cycle: \
             expected {literal:?} to still appear in the output, but it did not"
        );
        std::process::exit(1);
    }

    println!("did not reproduce - the float literal round-tripped exactly");
}
