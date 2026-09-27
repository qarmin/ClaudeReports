use dicom_core::Tag;
use dicom_dictionary_std::tags;
use dicom_encoding::text::{SpecificCharacterSet, TextCodec};
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

    let greek = SpecificCharacterSet::from_code("ISO_IR 126").expect("ISO_IR 126 is supported");
    let greek_sop_class_uid = "Δοκιμή";
    let mut greek_bytes = greek.encode(greek_sop_class_uid).expect("greek encode should succeed");
    if greek_bytes.len() % 2 == 1 {
        greek_bytes.push(0);
    }

    // Hand-built explicit-VR-LE dataset bytes:
    //   (0008,0005) CS "ISO_IR 126"          - declares the data set's character set
    //   (0008,0016) UI <Greek-encoded bytes> - SOPClassUID, decoded under the declared charset
    // per parser/src/stateful/decode.rs's read_value_strs, VR::UI is NOT in the
    // charset-override-to-default exclusion list (only AE/CS/AS/UR are), so this decodes using
    // the Greek codec, not the DICOM-mandated default repertoire for UI.
    let mut wire = Vec::new();
    wire.extend(short_vr_element(
        tags::SPECIFIC_CHARACTER_SET,
        b"CS",
        b"ISO_IR 126",
    ));
    wire.extend(short_vr_element(
        tags::SOP_CLASS_UID,
        b"UI",
        &greek_bytes,
    ));

    let obj =
        InMemDicomObject::read_dataset_with_ts(&wire[..], ts).expect("dataset should parse");

    let decoded_sop_class_uid = obj
        .element(tags::SOP_CLASS_UID)
        .expect("SOPClassUID should be present")
        .to_str()
        .expect("SOPClassUID should decode as a string");

    println!("decoded SOPClassUID: {:?}", decoded_sop_class_uid);
    println!(
        "matches original Greek text (post charset-decode, ignoring trailing pad): {}",
        decoded_sop_class_uid.trim_end_matches('\0') == greek_sop_class_uid
    );

    // Now try to write the very same object straight back out.
    let mut out = Vec::new();
    match obj.write_dataset_with_ts(&mut out, ts) {
        Ok(()) => {
            println!("did not reproduce - write_dataset_with_ts succeeded on re-encoding a UI element decoded under a non-default character set");
        }
        Err(e) => {
            eprintln!(
                "REPRODUCED - parsing succeeded (SOPClassUID decoded as {:?}) but writing the \
                 SAME object straight back out failed: {:?}",
                decoded_sop_class_uid, e
            );
            std::process::exit(1);
        }
    }
}
