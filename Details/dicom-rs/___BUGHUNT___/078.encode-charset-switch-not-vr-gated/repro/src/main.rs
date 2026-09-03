use dicom_core::Tag;
use dicom_dictionary_std::tags;
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};

fn short_vr_element(tag: Tag, vr: &[u8; 2], value: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&tag.group().to_le_bytes());
    out.extend_from_slice(&tag.element().to_le_bytes());
    out.extend_from_slice(vr);
    assert!(value.len() % 2 == 0, "value must already be even length");
    out.extend_from_slice(&(value.len() as u16).to_le_bytes());
    out.extend_from_slice(value);
    out
}

fn main() {
    let ts = TransferSyntaxRegistry
        .get("1.2.840.10008.1.2.1")
        .expect("explicit VR little endian should be registered");

    // Hand-built explicit-VR-LE dataset bytes:
    //   (0008,0005) LO "ISO_IR 126"  - declares the character set, but with a
    //                                  non-CS VR (LO is still a "text" VR, but
    //                                  it is not the VR the decoder actually
    //                                  gates the charset switch on: only
    //                                  VR::CS routes through read_value_cs).
    //   (0010,0010) PN <0xE9 0x00>   - PatientName, a single Latin-1 byte
    //                                  (0xE9 = 'e' with acute accent under the
    //                                  DEFAULT repertoire, which is what decode
    //                                  actually uses here since it never saw a
    //                                  VR::CS-tagged SpecificCharacterSet).
    let mut wire = Vec::new();
    wire.extend(short_vr_element(
        tags::SPECIFIC_CHARACTER_SET,
        b"LO",
        b"ISO_IR 126",
    ));
    wire.extend(short_vr_element(tags::PATIENT_NAME, b"PN", &[0xE9, 0x00]));

    let obj = InMemDicomObject::read_dataset_with_ts(&wire[..], ts).expect("dataset should parse");

    let cs_elem = obj
        .element(tags::SPECIFIC_CHARACTER_SET)
        .expect("SpecificCharacterSet should be present");
    println!("SpecificCharacterSet VR as decoded: {:?}", cs_elem.header().vr());

    let decoded_patient_name = obj
        .element(tags::PATIENT_NAME)
        .expect("PatientName should be present")
        .to_str()
        .expect("PatientName should decode as a string");

    println!("decoded PatientName (under default repertoire): {:?}", decoded_patient_name);
    assert_eq!(
        decoded_patient_name.trim_end_matches('\0'),
        "\u{e9}",
        "decode must have used the DEFAULT repertoire for PatientName, \
         proving the VR=LO SpecificCharacterSet element never actually \
         switched the decoder's active character set"
    );

    // Now try to write the very same, just-parsed object straight back out.
    let mut out = Vec::new();
    match obj.write_dataset_with_ts(&mut out, ts) {
        Ok(()) => {
            println!(
                "did not reproduce - write_dataset_with_ts succeeded on re-encoding an object \
                 whose only SpecificCharacterSet element has a non-CS VR"
            );
        }
        Err(e) => {
            eprintln!(
                "REPRODUCED - parsing succeeded (PatientName decoded as {:?} under the default \
                 repertoire, exactly as the decoder's own VR::CS-gated charset switch dictates) \
                 but writing the SAME object straight back out failed: {:?}",
                decoded_patient_name, e
            );
            std::process::exit(1);
        }
    }
}
